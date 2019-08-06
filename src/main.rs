mod helpers;
mod parameters;
mod state_variable_filter;
mod envelope;
mod synth_device;
mod sample_player;
mod sample_loader;
mod all_pass;

/*

double Helpers::FastCos(double x)
{
    // normalize range from 0..2PI to 1..2
    const auto phaseScale = 1.0 / (M_PI * 2);
    auto phase = 1.0 + x * phaseScale;

    auto phaseAsInt = *reinterpret_cast<unsigned long long *>(&phase);
    int exponent = (phaseAsInt >> 52) - 1023;

    const auto fractBits = 32 - fastCosTabLog2Size;
    const auto fractScale = 1 << fractBits;
    const auto fractMask = fractScale - 1;

    auto significand = (unsigned int)((phaseAsInt << exponent) >> (52 - 32));
    auto index = significand >> fractBits;
    int fract = significand & fractMask;

    auto left = fastCosTab[index];
    auto right = fastCosTab[index + 1];

    auto fractMix = fract * (1.0 / fractScale);
    return left + (right - left) * fractMix;
}
*/

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

        let sample_rate = if let cpal::SampleRate(r) = format.sample_rate {
            r
        } else {
            44100
        };

        let sample1 = sample_loader::load_wav("test_s1.wav");
        let lens = sample1.len();
        println!("LOADED SMAPLE {}", lens);

        let mut sp = sample_player::SamplePlayer::new(sample_rate as f64);
        sp.loop_mode = sample_player::LoopMode::PingPong;
        sp.sample_loop_start = 0;
        sp.sample_loop_length = lens as i32;
        sp.sample_data = sample1;
        sp.calc_pitch(0.0);
        sp.init_pos();
        sp.run_prep();

        let mut ap = all_pass::AllPass::new();
        ap.set_buffer_size(10);
        ap.set_feedback(0.7);

        println!("SMPL: {}", sample_rate);

        let mut fl = state_variable_filter::Filter::new(sample_rate as f64);
        fl.set_q(1.0);
        fl.set_type(parameters::FilterType::Bandpass);

        let mut phase : f64 = 0.0;
        use cpal::{StreamData, UnknownTypeOutputBuffer};
        event_loop.run(move |stream_id, stream_result| {
            let stream_data = match stream_result {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                    return;
                }
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
                    fl.set_freq(200.0 + (helpers::fast_sin(phase) + 1.0) as f32 *  200.0);
                    phase += 0.01;
                    for elem in buffer.iter_mut() {
                        let u = next_xoroshiro128(&mut ss);
                        let n = ap.process(sp.next());
                        *elem = n;

//                        *elem = 0.1 * fl.next(u64_to_open01(u) as f32);
//                        *elem = 0.01 * helpers::fast_sin(phase) as f32;
                        last = *elem;
                    }
//                    println!("FOFOE5 {}", last);
                },
                _ => (),
            }
    //        println!("FIOFOFO: {}", stream_result.unwrap());
            // react to stream events and read or write stream data here
        });

    });
}


fn main() {
    helpers::init_cos_tab();
    audio();
    println!("TST");
    loop { }
}
