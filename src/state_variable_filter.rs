use crate::helpers::*;
use crate::parameters::*;

struct Filter {
    sample_rate: f64,
    recalculate: bool,
    filter_type: FilterType,
    freq:        f32,
    q:           f32,
    last_input:  f32,
    low:         f32,
    band:        f32,
    f:           f32,
}

impl Filter {
    fn new(sample_rate: f64) -> Self {
        Filter {
            sample_rate,
            recalculate: true,
            filter_type: FilterType::Lowpass,
            freq:        20.0,
            q:           1.0,
            last_input:  0.0,
            low:         0.0,
            band:        0.0,
            f:           0.0,
        }
    }

    fn set_parameter(p: Parameter) -> Result<(),()> {
        Ok(())
    }

    fn next(&mut self, input: f32) -> f32 {
        if self.recalculate {
            self.f = 1.5_f32
                     * (fast_sin(
                        3.141592_f64
                        * (self.freq as f64
                           / 2.0_f64
                           / self.sample_rate)) as f32);
            self.recalculate = false;
        }

        let ret =
             (self.run(self.last_input + input) / 2.0)
            + self.run(input)                   / 2.0;
        self.last_input = input;
        ret
    }

    fn run(&mut self, input: f32) -> f32 {
        self.low = self.low + self.f * self.band;
        let high = self.q * (input - self.band) - self.low;
        self.band = self.band + self.f * high;

        match self.filter_type {
            FilterType::Lowpass  => self.low,
            FilterType::Highpass => high,
            FilterType::Bandpass => self.band,
            FilterType::Notch    => self.low + high,
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_into_and_from() {
        let x : f32 = FilterType::Bandpass.into();
        assert_eq!(x, 2.0);
        let y : FilterType = (2.0).into();
        assert_eq!(y, FilterType::Bandpass);
    }
}
