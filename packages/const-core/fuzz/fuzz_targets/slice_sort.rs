#![no_main]

use libfuzzer_sys::fuzz_target;

use const_core::slice_sort;

fn test(mut data: Vec<u8>) {
    let data1: &mut [u8] = &mut data.clone();
    let data2: &mut [u8] = &mut data;
    data1.sort();
    slice_sort!(data2);

    assert_eq!(data1, data2);
}

fuzz_target!(|data: Vec<u8>| test(data));
