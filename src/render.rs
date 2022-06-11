use crate::rendertables::{RECTANGLE, SAMPLE_TABLE, SINE};
use crate::{phonemes::Phonemes, Params};
use std::cmp::Ordering;

pub struct FormantTables {
    mouth: [u8; 80],
    throat: [u8; 80],
}

impl Default for FormantTables {
    fn default() -> Self {
        let mouth = [
            0x00, 0x13, 0x13, 0x13, 0x13, 0xA, 0xE, 0x12, 0x18, 0x1A, 0x16, 0x14, 0x10, 0x14, 0xE,
            0x12, 0xE, 0x12, 0x12, 0x10, 0xC, 0xE, 0xA, 0x12, 0xE, 0xA, 8, 6, 6, 6, 6, 0x11, 6, 6,
            6, 6, 0xE, 0x10, 9, 0xA, 8, 0xA, 6, 6, 6, 5, 6, 0, 0x12, 0x1A, 0x14, 0x1A, 0x12, 0xC,
            6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xA, 0xA, 6, 6, 6, 0x2C, 0x13,
        ];
        let throat = [
            0x00, 0x43, 0x43, 0x43, 0x43, 0x54, 0x48, 0x42, 0x3E, 0x28, 0x2C, 0x1E, 0x24, 0x2C,
            0x48, 0x30, 0x24, 0x1E, 0x32, 0x24, 0x1C, 0x44, 0x18, 0x32, 0x1E, 0x18, 0x52, 0x2E,
            0x36, 0x56, 0x36, 0x43, 0x49, 0x4F, 0x1A, 0x42, 0x49, 0x25, 0x33, 0x42, 0x28, 0x2F,
            0x4F, 0x4F, 0x42, 0x4F, 0x6E, 0x00, 0x48, 0x26, 0x1E, 0x2A, 0x1E, 0x22, 0x1A, 0x1A,
            0x1A, 0x42, 0x42, 0x42, 0x6E, 0x6E, 0x6E, 0x54, 0x54, 0x54, 0x1A, 0x1A, 0x1A, 0x42,
            0x42, 0x42, 0x6D, 0x56, 0x6D, 0x54, 0x54, 0x54, 0x7F, 0x7F,
        ];

        Self { mouth, throat }
    }
}

impl FormantTables {
    pub fn from_params(params: &Params) -> Self {
        let mut tables = FormantTables::default();
        let mouth_formants5_29: [u8; 30] = [
            0, 0, 0, 0, 0, 10, 14, 19, 24, 27, 23, 21, 16, 20, 14, 18, 14, 18, 18, 16, 13, 15, 11,
            18, 14, 11, 9, 6, 6, 6,
        ];
        let throat_formants5_29: [u8; 30] = [
            255, 255, 255, 255, 255, 84, 73, 67, 63, 40, 44, 31, 37, 45, 73, 49, 36, 30, 51, 37,
            29, 69, 24, 50, 30, 24, 83, 46, 54, 86,
        ];
        let mouth_formants48_53: [u8; 6] = [19, 27, 21, 27, 18, 13];
        let throat_formants48_53: [u8; 6] = [72, 39, 31, 43, 30, 34];

        for idx in 5..30 {
            if mouth_formants5_29[idx] != 0 {
                tables.mouth[idx] =
                    ((params.mouth as u32 * mouth_formants5_29[idx] as u32) / 2) as u8;
            }

            if throat_formants5_29[idx] != 0 {
                tables.throat[idx] =
                    ((params.throat as u32 * throat_formants5_29[idx] as u32) / 2) as u8;
            }
        }

        for (idx, table_idx) in (48..54).enumerate() {
            tables.mouth[table_idx] =
                ((params.mouth as u32 * mouth_formants48_53[idx] as u32) / 2) as u8;
            tables.throat[table_idx] =
                ((params.throat as u32 * throat_formants48_53[idx] as u32) / 2) as u8;
        }

        tables
    }
}

struct FramesTables {
    pitches: [u8; 256],
    frequency1: [u8; 256],
    frequency2: [u8; 256],
    frequency3: [u8; 256],
    amplitude1: [u8; 256],
    amplitude2: [u8; 256],
    amplitude3: [u8; 256],
    sampled_consonant_flag: [u8; 256],
}

// RENDER THE PHONEMES IN THE LIST
//
// The phoneme list is converted into sound through the steps:
//
// 1. Copy each phoneme <length> number of times into the frames list,
//    where each frame represents 10 milliseconds of sound.
//
// 2. Determine the transitions lengths between phonemes, and linearly
//    interpolate the values across the frames.
//
// 3. Offset the pitches by the fundamental frequency.
//
// 4. Render the each frame.

//void Code47574()
pub fn render(params: &Params, phonemes: &Phonemes, formants: &FormantTables) -> Vec<u8> {
    // CREATE FRAMES
    //
    // The length parameter in the list corresponds to the number of frames
    // to expand the phoneme to. Each frame represents 10 milliseconds of time.
    // So a phoneme with a length of 7 = 7 frames = 70 milliseconds duration.
    //
    // The parameters are copied from the phoneme to the frame verbatim.

    let mut frames = FramesTables {
        pitches: [0; 256],
        frequency1: [0; 256],
        frequency2: [0; 256],
        frequency3: [0; 256],
        amplitude1: [0; 256],
        amplitude2: [0; 256],
        amplitude3: [0; 256],
        sampled_consonant_flag: [0; 256],
    };

    const SAMPLED_CONSONANT_FLAGS: [u8; 80] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0xF1, 0xE2, 0xD3, 0xBB, 0x7C, 0x95, 1, 2, 3, 3, 0, 0x72, 0, 2, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1B, 0, 0, 0x19, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    const FREQ3: [u8; 80] = [
        0x00, 0x5B, 0x5B, 0x5B, 0x5B, 0x6E, 0x5D, 0x5B, 0x58, 0x59, 0x57, 0x58, 0x52, 0x59, 0x5D,
        0x3E, 0x52, 0x58, 0x3E, 0x6E, 0x50, 0x5D, 0x5A, 0x3C, 0x6E, 0x5A, 0x6E, 0x51, 0x79, 0x65,
        0x79, 0x5B, 0x63, 0x6A, 0x51, 0x79, 0x5D, 0x52, 0x5D, 0x67, 0x4C, 0x5D, 0x65, 0x65, 0x79,
        0x65, 0x79, 0x00, 0x5A, 0x58, 0x58, 0x58, 0x58, 0x52, 0x51, 0x51, 0x51, 0x79, 0x79, 0x79,
        0x70, 0x6E, 0x6E, 0x5E, 0x5E, 0x5E, 0x51, 0x51, 0x51, 0x79, 0x79, 0x79, 0x65, 0x65, 0x70,
        0x5E, 0x5E, 0x5E, 0x08, 0x01,
    ];

    const AMPLITUDE1: [u8; 80] = [
        0, 0, 0, 0, 0, 0xD, 0xD, 0xE, 0xF, 0xF, 0xF, 0xF, 0xF, 0xC, 0xD, 0xC, 0xF, 0xF, 0xD, 0xD,
        0xD, 0xE, 0xD, 0xC, 0xD, 0xD, 0xD, 0xC, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0xB, 0xB, 0xB, 0xB,
        0, 0, 1, 0xB, 0, 2, 0xE, 0xF, 0xF, 0xF, 0xF, 0xD, 2, 4, 0, 2, 4, 0, 1, 4, 0, 1, 4, 0, 0, 0,
        0, 0, 0, 0, 0, 0xC, 0, 0, 0, 0, 0xF, 0xF,
    ];

    const AMPLITUDE2: [u8; 80] = [
        0, 0, 0, 0, 0, 0xA, 0xB, 0xD, 0xE, 0xD, 0xC, 0xC, 0xB, 9, 0xB, 0xB, 0xC, 0xC, 0xC, 8, 8,
        0xC, 8, 0xA, 8, 8, 0xA, 3, 9, 6, 0, 0, 0, 0, 0, 0, 0, 0, 3, 5, 3, 4, 0, 0, 0, 5, 0xA, 2,
        0xE, 0xD, 0xC, 0xD, 0xC, 8, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0xA,
        0, 0, 0xA, 0, 0, 0,
    ];

    const AMPLITUDE3: [u8; 80] = [
        0, 0, 0, 0, 0, 8, 7, 8, 8, 1, 1, 0, 1, 0, 7, 5, 1, 0, 6, 1, 0, 7, 0, 5, 1, 0, 8, 0, 0, 3,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0xE, 1, 9, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 5, 0, 0x13, 0x10,
    ];

    {
        let mut phase_1;
        let mut frame_idx = 0;
        for idx in 0..255 {
            let phoneme = phonemes.phoneme_index[idx];
            if phoneme == 255 {
                break;
            }
            println!("converting phoneme {} at {} to frames.", phoneme, idx);

            if phoneme == 1 {
                // Period
                add_inflection(frame_idx, &mut frames, 1);
            }

            if phoneme == 2 {
                // Question mark
                add_inflection(frame_idx, &mut frames, 255);
            }

            let table: [u8; 11] = [0, 0, 0xE0, 0xE6, 0xEC, 0xF3, 0xF9, 0, 6, 0xC, 6];
            phase_1 = table[phonemes.stress[idx] as usize + 1];
            for _ in 0..phonemes.phoneme_length[idx] {
                frames.frequency1[frame_idx] = formants.mouth[phoneme as usize];
                frames.frequency2[frame_idx] = formants.throat[phoneme as usize];
                frames.frequency3[frame_idx] = FREQ3[phoneme as usize];
                frames.amplitude1[frame_idx] = AMPLITUDE1[phoneme as usize];
                frames.amplitude2[frame_idx] = AMPLITUDE2[phoneme as usize];
                frames.amplitude3[frame_idx] = AMPLITUDE3[phoneme as usize];
                frames.sampled_consonant_flag[frame_idx] =
                    SAMPLED_CONSONANT_FLAGS[phoneme as usize];
                frames.pitches[frame_idx] = params.pitch + phase_1;
                frame_idx += 1;
            }

            println!("{} frames", frame_idx);
        }
    }
    // CREATE TRANSITIONS
    //
    // Linear transitions are now created to smoothly connect the
    // end of one sustained portion of a phoneme to the following
    // phoneme.
    //
    // To do this, three tables are used:
    //
    //  Table         Purpose
    //  =========     ==================================================
    //  blendRank     Determines which phoneme's blend values are used.
    //
    //  blendOut      The number of frames at the end of the phoneme that
    //                will be used to transition to the following phoneme.
    //
    //  blendIn       The number of frames of the following phoneme that
    //                will be used to transition into that phoneme.
    //
    // In creating a transition between two phonemes, the phoneme
    // with the HIGHEST rank is used. Phonemes are ranked on how much
    // their identity is based on their transitions. For example,
    // vowels are and diphthongs are identified by their sustained portion,
    // rather than the transitions, so they are given low values. In contrast,
    // stop consonants (P, B, T, K) and glides (Y, L) are almost entirely
    // defined by their transitions, and are given high rank values.
    //
    // Here are the rankings used by SAM:
    //
    //     Rank    Type                         Phonemes
    //     2       All vowels                   IY, IH, etc.
    //     5       Diphthong endings            YX, WX, ER
    //     8       Terminal liquid consonants   LX, WX, YX, N, NX
    //     9       Liquid consonants            L, RX, W
    //     10      Glide                        R, OH
    //     11      Glide                        WH
    //     18      Voiceless fricatives         S, SH, F, TH
    //     20      Voiced fricatives            Z, ZH, V, DH
    //     23      Plosives, stop consonants    P, T, K, KX, DX, CH
    //     26      Stop consonants              J, GX, B, D, G
    //     27-29   Stop consonants (internal)   **
    //     30      Unvoiced consonants          /H, /X and Q*
    //     160     Nasal                        M
    //
    // To determine how many frames to use, the two phonemes are
    // compared using the blendRank[] table. The phoneme with the
    // higher rank is selected. In case of a tie, a blend of each is used:
    //
    //      if blendRank[phoneme1] ==  blendRank[phomneme2]
    //          // use lengths from each phoneme
    //          outBlendFrames = outBlend[phoneme1]
    //          inBlendFrames = outBlend[phoneme2]
    //      else if blendRank[phoneme1] > blendRank[phoneme2]
    //          // use lengths from first phoneme
    //          outBlendFrames = outBlendLength[phoneme1]
    //          inBlendFrames = inBlendLength[phoneme1]
    //      else
    //          // use lengths from the second phoneme
    //          // note that in and out are SWAPPED!
    //          outBlendFrames = inBlendLength[phoneme2]
    //          inBlendFrames = outBlendLength[phoneme2]
    //
    // Blend lengths can't be less than zero.
    //
    // Transitions are assumed to be symetrical, so if the transition
    // values for the second phoneme are used, the inBlendLength and
    // outBlendLength values are SWAPPED.
    //
    // For most of the parameters, SAM interpolates over the range of the last
    // outBlendFrames-1 and the first inBlendFrames.
    //
    // The exception to this is the Pitch[] parameter, which is interpolates the
    // pitch from the CENTER of the current phoneme to the CENTER of the next
    // phoneme.
    //
    // Here are two examples. First, For example, consider the word "SUN" (S AH N)
    //
    //    Phoneme   Duration    BlendWeight    OutBlendFrames    InBlendFrames
    //    S         2           18             1                 3
    //    AH        8           2              4                 4
    //    N         7           8              1                 2
    //
    // The formant transitions for the output frames are calculated as follows:
    //
    //     flags ampl1 freq1 ampl2 freq2 ampl3 freq3 pitch
    //    ------------------------------------------------
    // S
    //    241     0     6     0    73     0    99    61   Use S (weight 18) for transition instead of AH (weight 2)
    //    241     0     6     0    73     0    99    61   <-- (OutBlendFrames-1) = (1-1) = 0 frames
    // AH
    //      0     2    10     2    66     0    96    59 * <-- InBlendFrames = 3 frames
    //      0     4    14     3    59     0    93    57 *
    //      0     8    18     5    52     0    90    55 *
    //      0    15    22     9    44     1    87    53
    //      0    15    22     9    44     1    87    53
    //      0    15    22     9    44     1    87    53   Use N (weight 8) for transition instead of AH (weight 2).
    //      0    15    22     9    44     1    87    53   Since N is second phoneme, reverse the IN and OUT values.
    //      0    11    17     8    47     1    98    56 * <-- (InBlendFrames-1) = (2-1) = 1 frames
    // N
    //      0     8    12     6    50     1   109    58 * <-- OutBlendFrames = 1
    //      0     5     6     5    54     0   121    61
    //      0     5     6     5    54     0   121    61
    //      0     5     6     5    54     0   121    61
    //      0     5     6     5    54     0   121    61
    //      0     5     6     5    54     0   121    61
    //      0     5     6     5    54     0   121    61
    //
    // Now, consider the reverse "NUS" (N AH S):
    //
    //     flags ampl1 freq1 ampl2 freq2 ampl3 freq3 pitch
    //    ------------------------------------------------
    // N
    //     0     5     6     5    54     0   121    61
    //     0     5     6     5    54     0   121    61
    //     0     5     6     5    54     0   121    61
    //     0     5     6     5    54     0   121    61
    //     0     5     6     5    54     0   121    61
    //     0     5     6     5    54     0   121    61   Use N (weight 8) for transition instead of AH (weight 2)
    //     0     5     6     5    54     0   121    61   <-- (OutBlendFrames-1) = (1-1) = 0 frames
    // AH
    //     0     8    11     6    51     0   110    59 * <-- InBlendFrames = 2
    //     0    11    16     8    48     0    99    56 *
    //     0    15    22     9    44     1    87    53   Use S (weight 18) for transition instead of AH (weight 2)
    //     0    15    22     9    44     1    87    53   Since S is second phoneme, reverse the IN and OUT values.
    //     0     9    18     5    51     1    90    55 * <-- (InBlendFrames-1) = (3-1) = 2
    //     0     4    14     3    58     1    93    57 *
    // S
    //   241     2    10     2    65     1    96    59 * <-- OutBlendFrames = 1
    //   241     0     6     0    73     0    99    61

    let sum_length = {
        let mut idx = 0;
        let mut sum_length = 0;

        loop {
            let phoneme = phonemes.phoneme_index[idx];
            let next_phoneme = phonemes.phoneme_index[idx + 1];
            sum_length += phonemes.phoneme_length[idx];

            if phoneme == 255 {
                break;
            }

            // Used to decide which phoneme's blend lengths. The candidate with the lower score is selected.
            // tab45856
            const BLEND_RANK: [u8; 80] = [
                0, 0x1F, 0x1F, 0x1F, 0x1F, 2, 2, 2, 2, 2, 2, 2, 2, 2, 5, 5, 2, 0xA, 2, 8, 5, 5,
                0xB, 0xA, 9, 8, 8, 0xA0, 8, 8, 0x17, 0x1F, 0x12, 0x12, 0x12, 0x12, 0x1E, 0x1E,
                0x14, 0x14, 0x14, 0x14, 0x17, 0x17, 0x1A, 0x1A, 0x1D, 0x1D, 2, 2, 2, 2, 2, 2, 0x1A,
                0x1D, 0x1B, 0x1A, 0x1D, 0x1B, 0x1A, 0x1D, 0x1B, 0x1A, 0x1D, 0x1B, 0x17, 0x1D, 0x17,
                0x17, 0x1D, 0x17, 0x17, 0x1D, 0x17, 0x17, 0x1D, 0x17, 0x17, 0x17,
            ];
            const OUT_BLEND_LENGTH: [u8; 80] = [
                0, 2, 2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 2, 4, 4, 2, 2, 2, 2, 2, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 0, 1, 0, 1, 0, 5, 5, 5, 5, 5, 4, 4, 2, 0,
                1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 2, 2, 0, 1, 3, 0, 2, 3, 0, 2, 0xA0, 0xA0,
            ];

            const IN_BLEND_LENGTH: [u8; 80] = [
                0, 2, 2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 4, 4, 3, 3, 3, 3, 3, 1,
                2, 3, 2, 1, 3, 3, 3, 3, 1, 1, 3, 3, 3, 2, 2, 3, 2, 3, 0, 0, 5, 5, 5, 5, 4, 4, 2, 0,
                2, 2, 0, 3, 2, 0, 4, 2, 0, 3, 2, 0, 2, 2, 0, 2, 3, 0, 3, 3, 0, 3, 0xB0, 0xA0,
            ];

            let (r1, r2) = (
                BLEND_RANK[phoneme as usize],
                BLEND_RANK[next_phoneme as usize],
            );
            let (phase_1, phase_2) = match r1.cmp(&r2) {
                Ordering::Less => (
                    IN_BLEND_LENGTH[phoneme as usize],
                    OUT_BLEND_LENGTH[phoneme as usize],
                ),
                Ordering::Equal => (
                    OUT_BLEND_LENGTH[phoneme as usize],
                    OUT_BLEND_LENGTH[next_phoneme as usize],
                ),
                Ordering::Greater => (
                    OUT_BLEND_LENGTH[next_phoneme as usize],
                    IN_BLEND_LENGTH[next_phoneme as usize],
                ),
            };

            let start_frame = sum_length.wrapping_sub(phase_1);
            let end_frame = sum_length.wrapping_add(phase_2);

            // ???
            let do_interpolation = (phase_1.wrapping_add(phase_2).wrapping_sub(2)) & 128 == 0;

            if do_interpolation {
                for (is_pitch, current_table) in [
                    (true, &mut frames.pitches),
                    (false, &mut frames.frequency1),
                    (false, &mut frames.frequency2),
                    (false, &mut frames.frequency3),
                    (false, &mut frames.amplitude1),
                    (false, &mut frames.amplitude2),
                    (false, &mut frames.amplitude3),
                ] {
                    let (value, length) = if is_pitch {
                        let curr_halfwidth = phonemes.phoneme_length[idx] >> 1;
                        let next_halfwidth = phonemes.phoneme_length[idx + 1] >> 1;
                        let center_next = next_halfwidth + sum_length;
                        let center_curr = sum_length - curr_halfwidth;

                        let value = current_table[center_next as usize]
                            - current_table[center_curr as usize];
                        let length = curr_halfwidth + next_halfwidth;
                        (value, length)
                    } else {
                        let value =
                            current_table[end_frame as usize] - current_table[start_frame as usize];
                        let length = phase_1 + phase_2;
                        (value, length)
                    };

                    let value_sign = value & 128;

                    // Change per frame
                    let (change_remainder, change_per_frame) = {
                        let m53 = value as i8;
                        let m53abs = m53.abs() as u8;
                        let change_remainder = m53abs % length;
                        let change_per_frame = (m53 / length as i8) as u8;
                        (change_remainder, change_per_frame)
                    };

                    let mut carry = 0;
                    for idx in start_frame..(start_frame + length) {
                        let mut set_value = current_table[idx as usize] + change_per_frame;
                        carry += change_remainder;
                        if carry >= length {
                            carry -= length;
                            if value_sign % 128 == 0 {
                                if set_value != 0 {
                                    set_value += 1;
                                }
                            } else {
                                set_value -= 1;
                            }
                        }

                        current_table[idx as usize] = set_value;
                    }
                }
            }
            idx += 1;
        }
        sum_length
    };

    // ASSIGN PITCH CONTOUR
    //
    // This subtracts the F1 frequency from the pitch to create a
    // pitch contour. Without this, the output would be at a single
    // pitch level (monotone).
    if !params.singmode {
        for i in 0..256 {
            frames.pitches[i] -= frames.frequency1[i] >> 1;
        }
    }

    // RESCALE AMPLITUDE
    //
    // Rescale volume from a linear scale to decibels.
    //
    const AMPLITUDE_RESCALE: [u8; 17] = [
        0, 1, 2, 2, 2, 3, 3, 4, 4, 5, 6, 8, 9, 0xB, 0xD, 0xF, 0, //17 elements?
    ];

    for i in (0..=255).rev() {
        frames.amplitude1[i] = AMPLITUDE_RESCALE[frames.amplitude1[i] as usize];
        frames.amplitude2[i] = AMPLITUDE_RESCALE[frames.amplitude2[i] as usize];
        frames.amplitude3[i] = AMPLITUDE_RESCALE[frames.amplitude3[i] as usize];
    }

    // TODO implement PrintOutput to debug frame specs?

    // PROCESS THE FRAMES
    //
    // In traditional vocal synthesis, the glottal pulse drives filters, which
    // are attenuated to the frequencies of the formants.
    //
    // SAM generates these formants directly with sin and rectangular waves.
    // To simulate them being driven by the glottal pulse, the waveforms are
    // reset at the beginning of each glottal pulse.
    let mut output_buffer = C64SoundBuffer::default();

    {
        let mut frame_idx = 0;
        let mut phase_1 = 0;
        let mut phase_2 = 0;
        let mut phase_3 = 0;
        let mut sum_length = sum_length;
        let mut speed_counter = 72;
        let mut glottal_pulse_length = frames.pitches[frame_idx];
        let mut voiced_length = glottal_pulse_length - (glottal_pulse_length >> 2);
        loop {
            let consonant_flag = frames.sampled_consonant_flag[frame_idx];
            if consonant_flag & 248 != 0 {
                render_sample(frame_idx, &mut frames, &mut output_buffer);
                frame_idx += 2;
                sum_length -= 2;
            } else {
                let mut ary = [0u8; 5];
                let mut p1 = phase_1 as i32 * 256;
                let mut p2 = phase_2 as i32 * 256;
                let mut p3 = phase_3 as i32 * 256;
                for item in &mut ary {
                    let sp1: i8 = SINE[(0xff & (p1 >> 8)) as usize];
                    let sp2: i8 = SINE[(0xff & (p2 >> 8)) as usize];
                    let rp3: i8 = RECTANGLE[(0xff & (p3 >> 8)) as usize] as i8;

                    let sin1 = sp1 as i32 * (frames.amplitude1[frame_idx] as i32 & 0x0f);
                    let sin2 = sp2 as i32 * (frames.amplitude2[frame_idx] as i32 & 0x0f);
                    let rect = rp3 as i32 * (frames.amplitude3[frame_idx] as i32 & 0x0f);
                    let sum = sin1 + sin2 + rect;
                    let mux = (sum / 32) + 128;
                    *item = mux as u8;

                    p1 += frames.frequency1[frame_idx] as i32 * 256 / 4;
                    p2 += frames.frequency2[frame_idx] as i32 * 256 / 4;
                    p3 += frames.frequency3[frame_idx] as i32 * 256 / 4;
                }

                output_buffer.output_5(0, &ary);
                speed_counter -= 1;
                if speed_counter != 0 {}

                frame_idx += 1;
                sum_length -= 1;
            }

            if sum_length == 0 {
                return output_buffer.buffer;
            }

            if glottal_pulse_length == 0 {
                glottal_pulse_length = frames.pitches[frame_idx];
                voiced_length = glottal_pulse_length - (glottal_pulse_length >> 2);
                phase_1 = 0;
                phase_2 = 0;
                phase_3 = 0;
                continue;
            }

            voiced_length -= 1;
            if voiced_length != 0 && consonant_flag == 0 {
                phase_1 += frames.frequency1[frame_idx];
                phase_2 += frames.frequency2[frame_idx];
                phase_3 += frames.frequency3[frame_idx];
                continue;
            }

            render_sample(frame_idx, &mut frames, &mut output_buffer);
            glottal_pulse_length = frames.pitches[frame_idx];
            voiced_length = glottal_pulse_length - (glottal_pulse_length >> 2);
            phase_1 = 0;
            phase_2 = 0;
            phase_3 = 0;
            continue;
        }
    }
}

#[derive(Default)]
struct C64SoundBuffer {
    buffer: Vec<u8>,
    idx: usize,
    prev_timetable_idx: usize,
}

impl C64SoundBuffer {
    pub fn output_5(&mut self, timetable_idx: usize, array: &[u8; 5]) {
        self.idx += C64_TIMETABLE[self.prev_timetable_idx][timetable_idx] as usize;
        self.prev_timetable_idx = timetable_idx;
        self.buffer
            .resize(self.buffer.len().max(self.idx / 50 + array.len()), 0);
        self.buffer[(self.idx / 50)..(self.idx / 50 + array.len())].copy_from_slice(array);
    }
}

const C64_TIMETABLE: [[u8; 5]; 5] = [
    [162, 167, 167, 127, 128],
    [226, 60, 60, 0, 0],
    [225, 60, 59, 0, 0],
    [200, 0, 0, 54, 55],
    [199, 0, 0, 54, 54],
];

fn add_inflection(frame_idx: usize, frames: &mut FramesTables, inflection_direction: u8) {
    let mut idx = frame_idx.saturating_sub(30);
    while frames.pitches[idx] == 127 {
        idx += 1;
    }

    let mut pitch = frames.pitches[idx];
    loop {
        pitch = pitch.wrapping_add(inflection_direction);

        frames.pitches[idx] = pitch;
        loop {
            idx += 1;
            if idx == frame_idx {
                return;
            }
            if frames.pitches[idx] != 255 {
                break;
            }
        }
    }
}

fn render_sample(frame_idx: usize, frames: &mut FramesTables, output_buffer: &mut C64SoundBuffer) {
    let consonant_flag = frames.sampled_consonant_flag[frame_idx];
    let mem56 = (consonant_flag & 7) - 1;

    const TABLE: [u8; 5] = [0x18, 0x1A, 0x17, 0x17, 0x17];

    let table_value = TABLE[mem56 as usize];
    let mem47 = mem56;

    if consonant_flag & 248 == 0 {
        // VOICED

        let pitch = frames.pitches[frame_idx] >> 4;

        let mut phase1 = pitch ^ 255;
        let mut sample_table_idx = 0;
        while phase1 != 0 {
            let sample = SAMPLE_TABLE[mem47 as usize * 256 + sample_table_idx];

            for _mem56 in [8, 7, 6, 5, 4, 3, 2, 1] {
                if (sample << 1) & 128 != 0 {
                    output_buffer.output_5(3, &[(26 & 0xf) * 16; 5]);
                } else {
                    output_buffer.output_5(4, &[(6 & 0xf) * 16; 5]);
                }
            }

            phase1 = phase1.wrapping_add(1);
            sample_table_idx += 1;
        }
    } else {
        // UNVOICED
        let mut offset = (consonant_flag & 248) ^ 255;
        while offset != 0 {
            for _mem56 in [8, 7, 6, 5, 4, 3, 2, 1] {
                let sample = SAMPLE_TABLE[mem47 as usize * 256 + offset as usize];
                if (sample << 1) & 128 == 0 {
                    output_buffer.output_5(1, &[table_value & 0x0f; 5]);
                }

                if (sample << 1) & 128 != 0 || table_value != 0 {
                    output_buffer.output_5(2, &[5 * 16; 5]);
                }
            }
            offset = offset.wrapping_add(1);
        }
    }
}
