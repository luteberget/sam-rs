use crate::{phonemes::{convert_phonemes, print_phonemes}, render::FormantTables};

mod phonemes;
mod render;
mod rendertables;

pub struct Params {
    pub speed: u8,
    pub pitch: u8,
    pub mouth: u8,
    pub throat: u8,
    pub singmode: bool,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            speed: 72,
            pitch: 64,
            mouth: 128,
            throat: 128,
            singmode: false,
        }
    }
}

fn main() {
    let input = "/HAALAOAO MAYN NAAMAEAE IHSTT SAEBAASTTIHAAN";
    println!("phonetic input: {}", input);

    let params = Params::default();

    
    let phonemes = convert_phonemes(input.as_bytes());
    print_phonemes(&phonemes);
    
    let formant_tables = FormantTables::from_params(&params);
    let buffer: Vec<u8> = render::render(&params, &phonemes, &formant_tables);

    println!("{:?}", buffer);
}
