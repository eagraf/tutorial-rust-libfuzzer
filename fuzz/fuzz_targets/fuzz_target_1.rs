#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate tutorial_rust;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    tutorial_rust::broken_method(data);
});
