#![no_main]

mod common;

use common::execute_bytes;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = execute_bytes(data);
});
