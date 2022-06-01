#![no_main]
use libfuzzer_sys::fuzz_target;
use lopdf::{Document};

fuzz_target!(|data: &[u8]| {
    match std::str::from_utf8(data) {
        Ok (s) => {
            let mut doc = Document::load("./example.pdf").unwrap();
            let _ = doc.replace_text(1, "Hello World!", s);
        },
        _ => {},
    }
});
