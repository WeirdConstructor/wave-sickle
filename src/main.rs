// Taken from xoroshiro128 crate under MIT License
// Implemented by Matthew Scharley (Copyright 2016)
// https://github.com/mscharley/rust-xoroshiro128
pub fn next_xoroshiro128(state: &mut [u64; 2]) -> u64 {
    let s0: u64     = state[0];
    let mut s1: u64 = state[1];
    let result: u64 = s0.wrapping_add(s1);

    s1 ^= s0;
    state[0] = s0.rotate_left(55) ^ s1 ^ (s1 << 14); // a, b
    state[1] = s1.rotate_left(36); // c

    result
}

// Taken from rand::distributions
// Licensed under the Apache License, Version 2.0
// Copyright 2018 Developers of the Rand project.
pub fn u64_to_open01(u: u64) -> f64 {
    use core::f64::EPSILON;
    let float_size         = std::mem::size_of::<f64>() as u32 * 8;
    let fraction           = u >> (float_size - 52);
    let exponent_bits: u64 = (1023 as u64) << 52;
    f64::from_bits(fraction | exponent_bits) - (1.0 - EPSILON / 2.0)
}

fn audio() {
    std::thread::spawn(move || {
        use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
        let host = cpal::default_host();
        let event_loop = host.event_loop();
        let device = host.default_output_device().expect("no output device available");
        let format = device.default_output_format().expect("proper default format");
        println!("FORMAT: {:?}", format);
        let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
        event_loop.play_stream(stream_id).expect("failed to play_stream");

        let a : u64 = 0x193a6754a8a7d469;
        let b : u64 = 0x97830e05113ba7bb;
        let mut ss = [a, b];

        use cpal::{StreamData, UnknownTypeOutputBuffer};
        event_loop.run(move |stream_id, stream_result| {
            let stream_data = match stream_result {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                    return;
                }
                _ => return,
            };

            match stream_data {
                StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
                    println!("FOFOE3");
                    for elem in buffer.iter_mut() {
                        *elem = u16::max_value() / 2;
                    }
                },
                StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
                    println!("FOFOE2");
                    for elem in buffer.iter_mut() {
                        *elem = 0;
                    }
                },
                StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
                    let mut last = 0.0;
                    for elem in buffer.iter_mut() {
                        let u = next_xoroshiro128(&mut ss);
                        *elem = 0.01 * u64_to_open01(u) as f32;
                        last = *elem;
                    }
                    println!("FOFOE5 {}", last);
                },
                _ => (),
            }
    //        println!("FIOFOFO: {}", stream_result.unwrap());
            // react to stream events and read or write stream data here
        });

    });
}


fn main() {
    audio();
    println!("TST");
    loop { }
}
