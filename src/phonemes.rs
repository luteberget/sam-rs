pub struct Phonemes {
    pub stress: [u8; 256],
    pub phoneme_length: [u8; 256],
    pub phoneme_index: [u8; 256],
}

impl Default for Phonemes {
    fn default() -> Self {
        Self {
            stress: [0; 256],
            phoneme_length: [0; 256],
            phoneme_index: [0; 256],
        }
    }
}

pub fn print_phonemes(mem: &Phonemes) {
    // int i = 0;
    println!("===========================================");

    println!("Internal Phoneme presentation:");
    println!();
    println!(" idx    phoneme  length  stress");
    println!("------------------------------");

    for idx in 0..255 {
        let phoneme = mem.phoneme_index[idx];
        if phoneme == 255 {
            break;
        }
        if phoneme < 81 {
            println!(
                " {:3}      {}{}      {:3}       {}",
                phoneme,
                String::from_utf8_lossy(&[SIGN_INPUT_TABLE_1[phoneme as usize]]),
                String::from_utf8_lossy(&[SIGN_INPUT_TABLE_2[phoneme as usize]]),
                mem.phoneme_length[idx],
                mem.stress[idx]
            );
        } else {
            println!(
                " {:3}      ??      {:3}       {}\n",
                phoneme, mem.phoneme_length[idx], mem.stress[idx]
            );
        }
    }

    println!("===========================================");
    println!();
}

pub fn convert_phonemes(phonetic: &[u8]) -> Phonemes {
    let mut mem = Phonemes::default();
    parse_1(&mut mem, phonetic);
    print_phonemes(&mem);
    parse_2(&mut mem);
    copy_stress(&mut mem);
    set_phoneme_length(&mut mem);
    code41240(&mut mem);
    delete_errors(&mut mem);
    insert_breath(&mut mem);
    mem
}

fn insert_breath(mem: &mut Phonemes) {
    let mut cum_length = 0;
    let mut idx = 0;
    let mut other_idx = 255;
    while mem.phoneme_index[idx] != 255 {
        let phoneme = mem.phoneme_index[idx];
        cum_length += mem.phoneme_length[idx];

        if cum_length < 232 {
            if phoneme != 254 && FLAGS2[phoneme as usize] & 1 != 0 {
                insert(mem, idx + 1, 254, 0, 0);
                idx += 2;
                continue;
            }
            if phoneme == 0 {
                other_idx = idx;
            }

            idx += 1;
            continue;
        }

        mem.phoneme_index[other_idx] = 31;
        mem.phoneme_length[other_idx] = 4;
        mem.stress[other_idx] = 0;

        cum_length = 0;
        insert(mem, other_idx + 1, 254, 0, 0);
        idx = other_idx + 2;
    }
}

fn delete_errors(mem: &mut Phonemes) {
    let error_idx = mem.phoneme_index.iter().position(|n| *n > 80).unwrap();
    mem.phoneme_index[error_idx] = 255;
}

const FLAGS: [u8; 81] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0xA4, 0xA4, 0xA4, 0xA4, 0xA4, 0xA4, 0x84, 0x84, 0xA4, 0xA4, 0x84,
    0x84, 0x84, 0x84, 0x84, 0x84, 0x84, 0x44, 0x44, 0x44, 0x44, 0x44, 0x4C, 0x4C, 0x4C, 0x48, 0x4C,
    0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x44, 0x44, 0x44, 0x44, 0x48, 0x40, 0x4C, 0x44, 0x00, 0x00,
    0xB4, 0xB4, 0xB4, 0x94, 0x94, 0x94, 0x4E, 0x4E, 0x4E, 0x4E, 0x4E, 0x4E, 0x4E, 0x4E, 0x4E, 0x4E,
    0x4E, 0x4E, 0x4B, 0x4B, 0x4B, 0x4B, 0x4B, 0x4B, 0x4B, 0x4B, 0x4B, 0x4B, 0x4B, 0x4B, 0x80, 0xC1,
    0xC1,
];
const FLAGS2: [u8; 78] = [
    0x80, 0xC1, 0xC1, 0xC1, 0xC1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x10, 0x10, 0x10, 0x08, 0x0C, 0x08, 0x04, 0x40,
    0x24, 0x20, 0x20, 0x24, 0x00, 0x00, 0x24, 0x20, 0x20, 0x24, 0x20, 0x20, 0x00, 0x20, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
fn code41240(mem: &mut Phonemes) {
    let mut idx = 0;
    while mem.phoneme_index[idx] != 255 {
        let phoneme = mem.phoneme_index[idx];
        if FLAGS[phoneme as usize] & 2 == 0 {
            idx += 1;
            continue;
        }

        if FLAGS[phoneme as usize] & 1 == 0 {
            insert(
                mem,
                idx + 1,
                phoneme + 1,
                PHONEME_LENGTH_TABLE[(phoneme + 1) as usize],
                mem.stress[idx],
            );
            insert(
                mem,
                idx + 2,
                phoneme + 2,
                PHONEME_LENGTH_TABLE[(phoneme + 2) as usize],
                mem.stress[idx],
            );
            idx += 3;
            continue;
        }

        let next_phoneme_idx = {
            let mut next_phoneme_idx = idx + 1;
            while mem.phoneme_index[next_phoneme_idx] == 0 {
                next_phoneme_idx += 1;
            }
            next_phoneme_idx
        };

        let next_phoneme = mem.phoneme_index[next_phoneme_idx];
        if next_phoneme != 255
            && (FLAGS[next_phoneme as usize] & 8 != 0 || next_phoneme == 36 || next_phoneme == 37)
        {
            idx += 1;
            continue;
        }

        insert(
            mem,
            idx + 1,
            phoneme + 1,
            PHONEME_LENGTH_TABLE[(phoneme + 1) as usize],
            mem.stress[idx],
        );
        insert(
            mem,
            idx + 2,
            phoneme + 2,
            PHONEME_LENGTH_TABLE[(phoneme + 2) as usize],
            mem.stress[idx],
        );
        idx += 3;
    }
}
const PHONEME_LENGTH_TABLE: [u8; 80] = [
    0, 0x12, 0x12, 0x12, 8, 8, 8, 8, 8, 0xB, 6, 0xC, 0xA, 5, 5, 0xB, 0xA, 0xA, 0xA, 9, 8, 7, 9, 7,
    6, 8, 6, 7, 7, 7, 2, 5, 2, 2, 2, 2, 2, 2, 6, 6, 7, 6, 6, 2, 8, 3, 1, 0x1E, 0xD, 0xC, 0xC, 0xC,
    0xE, 9, 6, 1, 2, 5, 1, 1, 6, 1, 2, 6, 1, 2, 8, 2, 2, 4, 2, 2, 6, 1, 4, 6, 1, 4, 0xC7, 0xFF,
];
fn set_phoneme_length(mem: &mut Phonemes) {
    // INCLUDES AdjustLengths from sam.c

    let phoneme_stressed_length_table = [
        0x00, 0x12, 0x12, 0x12, 8, 0xB, 9, 0xB, 0xE, 0xF, 0xB, 0x10, 0xC, 6, 6, 0xE, 0xC, 0xE, 0xC,
        0xB, 8, 8, 0xB, 0xA, 9, 8, 8, 8, 8, 8, 3, 5, 2, 2, 2, 2, 2, 2, 6, 6, 8, 6, 6, 2, 9, 4, 2,
        1, 0xE, 0xF, 0xF, 0xF, 0xE, 0xE, 8, 2, 2, 7, 2, 1, 7, 2, 2, 7, 2, 2, 8, 2, 2, 6, 2, 2, 7,
        2, 4, 7, 1, 4, 5, 5,
    ];

    //change phonemelength depedendent on stress
    {
        let mut idx = 0;
        while mem.phoneme_index[idx] != 255 {
            let stress = mem.stress[idx];
            let table = if stress == 0 || stress & 128 != 0 {
                &PHONEME_LENGTH_TABLE
            } else {
                &phoneme_stressed_length_table
            };

            mem.phoneme_length[idx] = table[mem.phoneme_index[idx] as usize];
            idx += 1;
        }
    }

    {
        // LENGTHEN VOWELS PRECEDING PUNCTUATION
        //
        // Search for punctuation. If found, back up to the first vowel, then
        // process all phonemes between there and up to (but not including) the punctuation.
        // If any phoneme is found that is a either a fricative or voiced, the duration is
        // increased by (length * 1.5) + 1

        {
            let mut idx = 0;
            while mem.phoneme_index[idx] != 255 {
                let phoneme = mem.phoneme_index[idx];

                if FLAGS2[phoneme as usize] & 1 == 0 {
                    // Not punctuation.
                    idx += 1;
                    continue;
                }

                let end_idx = idx;
                let start_idx = {
                    let mut start_idx = idx - 1;
                    while start_idx > 0
                        && mem.phoneme_index[start_idx] != 255
                        && FLAGS[mem.phoneme_index[start_idx] as usize] & 128 == 0
                    {
                        start_idx -= 1;
                    }
                    start_idx
                };

                for lengthen_idx in start_idx..end_idx {
                    let phoneme = mem.phoneme_index[lengthen_idx];
                    if phoneme != 255
                        // test for fricative/unvoiced or not voiced
                        && (FLAGS2[phoneme as usize] & 32 == 0 || FLAGS[phoneme as usize] & 4 != 0)
                    {
                        let length = mem.phoneme_length[lengthen_idx];
                        let length = (length >> 1) + length + 1;
                        mem.phoneme_length[lengthen_idx] = length;
                    }
                }
                idx += 1;
            }
        }

        {
            let mut idx = 0;
            loop {
                let phoneme = mem.phoneme_index[idx];
                if phoneme == 255 {
                    break;
                }

                if FLAGS[phoneme as usize] & 128 != 0 {
                    let next_phoneme = mem.phoneme_index[idx + 1];
                    let flags = if next_phoneme == 255 {
                        65
                    } else {
                        FLAGS[next_phoneme as usize]
                    };
                    if FLAGS[next_phoneme as usize] & 64 == 0 {
                        if next_phoneme == 18 || next_phoneme == 19 {
                            // 'RX' or 'LX'
                            let nextnext_phoneme = mem.phoneme_index[idx + 1];
                            if FLAGS[nextnext_phoneme as usize] & 64 != 0 {
                                // RULE: <VOWEL> RX | LX <CONSONANT>
                                mem.phoneme_length[idx] -= 1;
                            }
                        }

                        idx += 1;
                        continue;
                    }

                    // Got here if not <VOWEL>
                    if flags & 4 == 0 {
                        if flags & 1 != 0 {
                            let length = mem.phoneme_length[idx - 1] >> 3;
                            mem.phoneme_length[idx - 1] = length;
                        }
                        idx += 1;
                        continue;
                    }

                    // RULE: <VOWEL> <VOICED CONSONANT>
                    // <VOWEL> <WH, R*, L*, W*, Y*, M*, N*, NX, DX, Q*, Z*, ZH, V*, DH, J*, B*, D*, G*, GX>
                    let length = mem.phoneme_length[idx];
                    let length = (length >> 2) + length + 1;
                    mem.phoneme_length[idx] = length;

                    idx += 1;
                    continue;
                }

                // RULE: <NASAL> <STOP CONSONANT>
                //       Set punctuation length to 6
                //       Set stop consonant length to 5
                if FLAGS2[phoneme as usize] & 8 != 0 {
                    let next_phoneme = mem.phoneme_index[idx + 1];
                    let flags = if next_phoneme == 255 {
                        65
                    } else {
                        FLAGS[next_phoneme as usize]
                    };
                    if flags & 2 != 0 {
                        // Stop consonant

                        mem.phoneme_length[idx + 1] = 6;
                        mem.phoneme_length[idx] = 5;
                    }
                    idx += 1;
                    continue;
                }

                // RULE: <VOICED STOP CONSONANT> {optional silence} <STOP CONSONANT>
                //       Shorten both to (length/2 + 1)
                if FLAGS[phoneme as usize] & 2 != 0 {
                    let next_idx = {
                        let mut next_idx = idx + 1;
                        while mem.phoneme_index[next_idx] == 0 {
                            next_idx += 1;
                        }
                        next_idx
                    };

                    let next_phoneme = mem.phoneme_index[next_idx];
                    if next_phoneme == 255 || FLAGS[next_phoneme as usize] & 2 == 0 {
                        idx += 1;
                        continue;
                    }

                    let length = mem.phoneme_length[next_idx];
                    let length = (length >> 1) + 1;
                    mem.phoneme_length[next_idx] = length;

                    let length = mem.phoneme_length[idx];
                    let length = (length >> 1) + 1;
                    mem.phoneme_length[idx] = length;

                    idx += 1;
                    continue;
                }

                // RULE: <VOICED NON-VOWEL> <DIPHTONG>
                //       Decrease <DIPHTONG> by 2

                // liquic consonant?

                if FLAGS2[phoneme as usize] & 16 != 0 {
                    let prev_phoneme = mem.phoneme_index[idx - 1];
                    if FLAGS[prev_phoneme as usize] % 2 != 0 {
                        mem.phoneme_length[idx] -= 2;
                    }
                }

                idx += 1;
            }
        }
    }
}

fn copy_stress(mem: &mut Phonemes) {
    // Iterates through the phoneme buffer, copying the stress value from
    // the following phoneme under the following circumstance:

    //     1. The current phoneme is voiced, excluding plosives and fricatives
    //     2. The following phoneme is voiced, excluding plosives and fricatives, and
    //     3. The following phoneme is stressed
    //
    //  In those cases, the stress value+1 from the following phoneme is copied.
    //
    // For example, the word LOITER is represented as LOY5TER, with as stress
    // of 5 on the diphtong OY. This routine will copy the stress value of 6 (5+1)
    // to the L that precedes it.

    // TODO(bjornarl): this seems to have bugs.

    let mut idx = 0;
    while mem.phoneme_index[idx + 1] != 255 {
        let phoneme = mem.phoneme_index[idx];
        let next_phoneme = mem.phoneme_index[idx + 1];
        let next_stress = mem.stress[idx + 1];
        idx += 1;

        // This is consonant
        if FLAGS[phoneme as usize] & 64 == 0 {
            continue;
        }

        // Next is vowel
        if FLAGS[next_phoneme as usize] & 128 == 0 {
            continue;
        }

        if next_stress == 0 {
            continue;
        }

        if next_stress & 128 != 0 {
            // ????, see sam.c:308
            continue;
        }

        mem.stress[idx] = next_stress + 1;
    }
}

fn parse_2(mem: &mut Phonemes) {
    // Rewrites the phonemes using the following rules:
    //
    //       <DIPHTONG ENDING WITH WX> -> <DIPHTONG ENDING WITH WX> WX
    //       <DIPHTONG NOT ENDING WITH WX> -> <DIPHTONG NOT ENDING WITH WX> YX
    //       UL -> AX L
    //       UM -> AX M
    //       <STRESSED VOWEL> <SILENCE> <STRESSED VOWEL> -> <STRESSED VOWEL> <SILENCE> Q <VOWEL>
    //       T R -> CH R
    //       D R -> J R
    //       <VOWEL> R -> <VOWEL> RX
    //       <VOWEL> L -> <VOWEL> LX
    //       G S -> G Z
    //       K <VOWEL OR DIPHTONG NOT ENDING WITH IY> -> KX <VOWEL OR DIPHTONG NOT ENDING WITH IY>
    //       G <VOWEL OR DIPHTONG NOT ENDING WITH IY> -> GX <VOWEL OR DIPHTONG NOT ENDING WITH IY>
    //       S P -> S B
    //       S T -> S D
    //       S K -> S G
    //       S KX -> S GX
    //       <ALVEOLAR> UW -> <ALVEOLAR> UX
    //       CH -> CH CH' (CH requires two phonemes to represent it)
    //       J -> J J' (J requires two phonemes to represent it)
    //       <UNSTRESSED VOWEL> T <PAUSE> -> <UNSTRESSED VOWEL> DX <PAUSE>
    //       <UNSTRESSED VOWEL> D <PAUSE>  -> <UNSTRESSED VOWEL> DX <PAUSE>

    let mut idx = 0;
    loop {
        let phoneme = mem.phoneme_index[idx];
        println!("idx{} phoneme {}", idx, phoneme);
        if phoneme == 0 {
            idx += 1;
            continue;
        }

        if phoneme == 255 {
            break;
        }

        // RULE:
        //       <DIPHTONG ENDING WITH WX> -> <DIPHTONG ENDING WITH WX> WX
        //       <DIPHTONG NOT ENDING WITH WX> -> <DIPHTONG NOT ENDING WITH WX> YX
        // Example: OIL, COW
        if FLAGS[phoneme as usize] & 16 != 0 {
            let new_phoneme = if FLAGS[phoneme as usize] & 32 == 0 {
                20
            } else {
                21
            };
            insert(
                mem,
                idx + 1,
                new_phoneme,
                mem.phoneme_length[idx],
                mem.stress[idx],
            );

            last_rules(phoneme, mem, &mut idx);
            continue; // TODO // Jump to ??? goto pos41749;
        }

        // RULE:
        //       UL -> AX L
        // Example: MEDDLE
        if phoneme == 78 {
            mem.phoneme_index[idx] = 13; // 'AX'
            insert(mem, idx + 1, 24, mem.phoneme_length[idx], mem.stress[idx]);
            idx += 1;
            continue;
        }

        // RULE:
        //       UM -> AX M
        // Example: ASTRONOMY

        if phoneme == 79 {
            mem.phoneme_index[idx] = 13; // 'AX'
            insert(mem, idx + 1, 27, mem.phoneme_length[idx], mem.stress[idx]);
            idx += 1;
            continue;
        }

        // RULE:
        //       UN -> AX N
        // Example: FUNCTION
        if phoneme == 80 {
            mem.phoneme_index[idx] = 13; // 'AX'
            insert(mem, idx + 1, 28, mem.phoneme_length[idx], mem.stress[idx]);
            idx += 1;
            continue;
        }

        // RULE:
        //       <STRESSED VOWEL> <SILENCE> <STRESSED VOWEL> -> <STRESSED VOWEL> <SILENCE> Q <VOWEL>
        // EXAMPLE: AWAY EIGHT

        if FLAGS[phoneme as usize] & 128 != 0
            && mem.stress[idx] != 0
            && mem.phoneme_index[idx + 1] == 0
        {
            let next_phoneme = mem.phoneme_index[idx + 2];
            if (next_phoneme != 255 && FLAGS[next_phoneme as usize] & 128 != 0)
                && mem.stress[idx + 2] != 0
            {
                insert(mem, idx + 2, 31, 0, 0);
                idx += 1;
                continue;
            }
        }

        // RULES FOR PHONEMES BEFORE R
        //        T R -> CH R
        // Example: TRACK
        if phoneme == 23 {
            // 'R'
            let prev_phoneme = mem.phoneme_index[idx - 1];
            if prev_phoneme == 69 {
                // 'T'
                mem.phoneme_index[idx - 1] = 42; // 'CH'
                insert(mem, idx, 43, 0, mem.stress[idx - 1]);
                idx += 1;
                continue;
            }

            if prev_phoneme == 57 {
                //'D'
                mem.phoneme_index[idx - 1] = 44; //'J'
                insert(mem, idx, 45, 0, mem.stress[idx - 1]);
            }

            // RULES FOR PHONEMES BEFORE R
            //        <VOWEL> R -> <VOWEL> RX
            // Example: ART

            if phoneme != 0 && FLAGS[phoneme as usize] & 128 != 0 {
                // vowel
                mem.phoneme_index[idx] = 18; // 'RX'
            }

            idx += 1;
            continue;
        }

        // RULE:
        //       <VOWEL> L -> <VOWEL> LX
        // Example: ALL

        if phoneme == 24 {
            //'L'
            if FLAGS[mem.phoneme_index[idx - 1] as usize] & 128 != 0 {
                mem.phoneme_index[idx] = 19; //'LX'
            }

            idx += 1;
            continue;
        }

        // RULE:
        //       G S -> G Z
        //
        // Can't get to fire -
        //       1. The G -> GX rule intervenes
        //       2. Reciter already replaces GS -> GZ

        if phoneme == 32 {
            // 'S'
            if mem.phoneme_index[idx - 1] == 60 {
                // 'G'
                mem.phoneme_index[idx] = 38; // 'Z'
            }

            idx += 1;
            continue;
        }
        // RULE:
        //             K <VOWEL OR DIPHTONG NOT ENDING WITH IY> -> KX <VOWEL OR DIPHTONG NOT ENDING WITH IY>
        // Example: COW

        if phoneme == 72 {
            // 'K'
            let next_phoneme = mem.phoneme_index[idx + 1];
            if next_phoneme == 255 || FLAGS[next_phoneme as usize] & 32 == 0 {
                mem.phoneme_index[idx] = 75;
            }
        } else if phoneme == 60 {
            // 'G'
            // RULE:
            //             G <VOWEL OR DIPHTONG NOT ENDING WITH IY> -> GX <VOWEL OR DIPHTONG NOT ENDING WITH IY>
            // Example: GO

            let next_phoneme = mem.phoneme_index[idx + 1];
            if next_phoneme != 255 && FLAGS[next_phoneme as usize] & 32 == 0 {
                mem.phoneme_index[idx] = 63; // 'GX'
            }

            idx += 1;
            continue;
        }

        let can_soften = FLAGS[phoneme as usize] & 1 != 0;
        if can_soften && mem.phoneme_index[idx - 1] == 32 {
            mem.phoneme_index[idx] = phoneme - 12;
            idx += 1;
            continue;
        }

        // NOTE(bjornarl) taking a chance here, that
        //  sam.c:936:             goto pos41812;
        // ... is unnecessary, it seems to be an optimization.

        last_rules(phoneme, mem, &mut idx);
    }
}

fn last_rules(phoneme: u8, mem: &mut Phonemes, idx: &mut usize) {
    if phoneme == 53 {
        // 'UW'
        if FLAGS2[mem.phoneme_index[*idx - 1] as usize] & 4 != 0 {
            mem.phoneme_index[*idx] = 16;
        }
        *idx += 1;
        return;
    }
    if phoneme == 42 {
        insert(mem, *idx + 1, 43, 0, mem.stress[*idx]);
        *idx += 1;
        return;
    }
    if phoneme == 44 {
        insert(mem, *idx + 1, 45, 0, mem.stress[*idx]);
        *idx += 1;
        return;
    }
    if (phoneme == 69 || phoneme == 57) && FLAGS[mem.phoneme_index[*idx - 1] as usize] & 128 != 0 {
        // T or D after vowel.
        let next_phoneme = mem.phoneme_index[*idx + 1];

        if next_phoneme == 255
            || (next_phoneme != 0 && FLAGS[next_phoneme as usize] & 128 != 0)
            || (FLAGS[next_phoneme as usize] & 128 != 0 && mem.stress[*idx + 1] != 0)
        {
            mem.phoneme_index[*idx] = 30; // 'DX'
        }
    }
    *idx += 1;
}

fn insert(mem: &mut Phonemes, position: usize, ph_idx: u8, ph_len: u8, stress: u8) {
    for i in (position..=253).rev() {
        mem.phoneme_index[i + 1] = mem.phoneme_index[i];
        mem.phoneme_length[i + 1] = mem.phoneme_length[i];
        mem.stress[i + 1] = mem.stress[i];
    }

    mem.phoneme_index[position] = ph_idx;
    mem.phoneme_length[position] = ph_len;
    mem.stress[position] = stress;
}

const SIGN_INPUT_TABLE_1: [u8; 81] = [
    b' ', b'.', b'?', b',', b'-', b'I', b'I', b'E', b'A', b'A', b'A', b'A', b'U', b'A', b'I', b'E',
    b'U', b'O', b'R', b'L', b'W', b'Y', b'W', b'R', b'L', b'W', b'Y', b'M', b'N', b'N', b'D', b'Q',
    b'S', b'S', b'F', b'T', b'/', b'/', b'Z', b'Z', b'V', b'D', b'C', b'*', b'J', b'*', b'*', b'*',
    b'E', b'A', b'O', b'A', b'O', b'U', b'B', b'*', b'*', b'D', b'*', b'*', b'G', b'*', b'*', b'G',
    b'*', b'*', b'P', b'*', b'*', b'T', b'*', b'*', b'K', b'*', b'*', b'K', b'*', b'*', b'U', b'U',
    b'U',
];
const SIGN_INPUT_TABLE_2: [u8; 81] = [
    b'*', b'*', b'*', b'*', b'*', b'Y', b'H', b'H', b'E', b'A', b'H', b'O', b'H', b'X', b'X', b'R',
    b'X', b'H', b'X', b'X', b'X', b'X', b'H', b'*', b'*', b'*', b'*', b'*', b'*', b'X', b'X', b'*',
    b'*', b'H', b'*', b'H', b'H', b'X', b'*', b'H', b'*', b'H', b'H', b'*', b'*', b'*', b'*', b'*',
    b'Y', b'Y', b'Y', b'W', b'W', b'W', b'*', b'*', b'*', b'*', b'*', b'*', b'*', b'*', b'*', b'X',
    b'*', b'*', b'*', b'*', b'*', b'*', b'*', b'*', b'*', b'*', b'*', b'X', b'*', b'*', b'L', b'M',
    b'N',
];

fn parse_1(mem: &mut Phonemes, phonetic: &[u8]) {
    mem.phoneme_index[255] = 32;

    let stress_input_table = [b'*', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8'];
    let mut output_idx = 0;
    let mut input_idx = 0;
    'phoneme_loop: while input_idx < phonetic.len() {
        let sign1 = phonetic[input_idx];
        input_idx += 1;

        // READ 2 (NON-WILDCARD IN TABLE2)
        if input_idx + 1 < phonetic.len() {
            let sign2 = phonetic[input_idx];

            for (table_idx, (t1, t2)) in SIGN_INPUT_TABLE_1
                .iter()
                .zip(SIGN_INPUT_TABLE_2.iter())
                .enumerate()
            {
                if sign1 == *t1 && sign2 == *t2 && *t2 != b'*' {
                    mem.phoneme_index[output_idx] = table_idx as _;
                    output_idx += 1;
                    input_idx += 1;
                    continue 'phoneme_loop;
                }
            }
        }

        // READ 1 (WITH WILDCARD IN TABLE2)
        {
            for (table_idx, (t1, t2)) in SIGN_INPUT_TABLE_1
                .iter()
                .zip(SIGN_INPUT_TABLE_2.iter())
                .enumerate()
            {
                if sign1 == *t1 && *t2 == b'*' {
                    mem.phoneme_index[output_idx] = table_idx as _;
                    output_idx += 1;
                    continue 'phoneme_loop;
                }
            }
        }

        // READ STRESS CHARACTER
        for (table_idx, stress_value) in stress_input_table.iter().enumerate().rev() {
            if table_idx > 0 && *stress_value == sign1 {
                mem.stress[output_idx - 1] = table_idx as _;
                continue 'phoneme_loop;
            }
        }

        panic!("Parse failed");
    }

    mem.phoneme_index[output_idx + 1] = 255;
}
