use crate::processor_module::ProcessorModule;

pub struct GammaModule {
    gamma: f32,
    sample_rate: f32,
    f_max: f32,
}

impl GammaModule {
    pub fn new(gamma: f32, sample_rate: f64) -> Self {
        Self {
            gamma: gamma,
            sample_rate: sample_rate as f32,
            f_max: (sample_rate as f32) / 2.0,
        }
    }
}

impl ProcessorModule for GammaModule {
    fn process(&mut self, input: &Vec<f32>) -> Vec<f32> {
        input.iter().enumerate().map(|(index, s)| {
            let f_i = (index as f32) * self.sample_rate / input.len() as f32;
            let gc = (f_i / self.f_max).powf(1.0 / self.gamma) * input.len() as f32;
            // print!("{} ", (s * gc));
            (s * gc)
        }).collect()

        // input.to_vec()
    }
}
