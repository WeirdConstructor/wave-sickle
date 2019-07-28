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
