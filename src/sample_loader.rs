use hound;

pub fn load_wav(file: &str) -> Vec<f32> {
    let mut reader = hound::WavReader::open(file)
        .expect(&format!("Couldn't open file '{}'", file));
    // TODO: Add proper error reporting from WavSpec
    // TODO: Make sample conversion from any format to 44.1khz in f32
    // TODO: Resample different sample rates to the current sample rate
    //       which should be passed into this function.
    let samples : Vec<f32> =
        reader.samples::<f32>()
            .map(|s| s.expect("sample files need to be 44.1khz float pcm"))
            .collect();
    samples
}
