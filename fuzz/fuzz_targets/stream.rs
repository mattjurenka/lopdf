#![no_main]
use libfuzzer_sys::fuzz_target;
use lopdf::{Document, Object};
use lopdf::Dictionary as LoDictionary;
use lopdf::Stream as LoStream;

fuzz_target!(|data: &[u8]| {
    match std::str::from_utf8(data) {
        Ok (s) => {
            let mut doc = Document::new();
            let id = doc.add_object(Object::string_literal(s));
            let id2 = doc.add_object(Object::Stream(LoStream::new(
                LoDictionary::new(),
                s.as_bytes().to_vec(),
            )));
            assert!(doc.get_object(id).is_ok());
            assert!(doc.get_object(id2).is_ok());
        },
        _ => {},
    }
});
