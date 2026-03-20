#![no_main]

mod common;

use common::execute_bytes;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let first = execute_bytes(data);
    let second = execute_bytes(data);
    assert_eq!(first, second);
});
