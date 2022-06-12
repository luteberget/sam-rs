mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct SamDebugInfo {
    pub buffer: Vec<u8>,
}

pub fn synthesize_phonetic_debug(input: &str) -> SamDebugInfo {
    let buffer = synthesize_phonetic(input);

    SamDebugInfo { buffer }
}

pub fn synthesize_phonetic(input: &str) -> Vec<u8> {
    fn set_input(input: &str) {
        let mut bytes = input.as_bytes().to_vec();
        bytes.push(0x9b);
        let input = std::ffi::CString::new(bytes).unwrap();
        unsafe { sys::SetInput(input.as_ptr()) };
    }

    set_input(input);

    unsafe { sys::SAMMain() };

    let buffer_ptr = unsafe { sys::GetBuffer() } as *const u8;
    let len = unsafe { sys::GetBufferLength() };

    assert!(!buffer_ptr.is_null());
    assert!(len / 50 != 0);

    let buffer = unsafe { std::slice::from_raw_parts(buffer_ptr, len as usize / 50) }.to_vec();

    buffer
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::{sys, synthesize_phonetic, synthesize_phonetic_debug};

    #[test]
    #[serial]
    fn simple() {
        dbg!(unsafe { sys::GetBuffer() });
        assert!(dbg!(unsafe { sys::GetBufferLength() }) == 0);
    }


    #[test]
    #[serial]
    fn complete() {
        synthesize_phonetic("/HAALAOAO ");
    }


    #[test]
    #[serial]
    fn complete_debug() {
        synthesize_phonetic_debug("/HAALAOAO ");
    }
}
