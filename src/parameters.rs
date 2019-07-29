#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FilterType {
    Lowpass,
    Highpass,
    Bandpass,
    Notch,
}

impl From<f32> for FilterType {
    fn from(item: f32) -> Self {
        let i : u32 = ((item * 3.0) as u32) % 4;
        match i {
            0 => FilterType::Lowpass,
            1 => FilterType::Highpass,
            2 => FilterType::Bandpass,
            3 => FilterType::Notch,
            _ => FilterType::Lowpass,
        }
    }
}

impl From<FilterType> for f32 {
    fn from(item: FilterType) -> f32 {
        match item {
            FilterType::Lowpass  => 0.0,
            FilterType::Highpass => 1.0,
            FilterType::Bandpass => 2.0,
            FilterType::Notch    => 3.0,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VoiceMode {
    Polyphonic,
    MonoLegatoTrill,
}

impl From<f32> for VoiceMode {
    fn from(item: f32) -> Self {
        let i = item as i32;
        match i {
            0 => VoiceMode::Polyphonic,
            1 => VoiceMode::MonoLegatoTrill,
            _ => VoiceMode::Polyphonic,
        }
    }
}

impl From<VoiceMode> for f32 {
    fn from(item: VoiceMode) -> f32 {
        match item {
            VoiceMode::Polyphonic      => 0.0,
            VoiceMode::MonoLegatoTrill => 1.0,
        }
    }
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Spread {
    Mono,
    FullInvert,
    ModInvert,
}

impl From<f32> for Spread {
    fn from(item: f32) -> Self {
        let i = (item * 2.0) as i32;
        match i {
            0 => Spread::Mono,
            1 => Spread::FullInvert,
            2 => Spread::ModInvert,
            _ => Spread::Mono,
        }
    }
}

impl From<Spread> for f32 {
    fn from(item: Spread) -> f32 {
        match item {
            Spread::Mono       => 0.0,
            Spread::FullInvert => 0.5,
            Spread::ModInvert  => 1.0,
        }
    }
}

pub enum Parameter {
    StateVariableFilterType(FilterType),
    Freq(f32),
    Q(f32),
}

pub struct ParameterData {
}
