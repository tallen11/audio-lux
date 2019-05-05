use crate::processor_module::ProcessorModule;

use rustfft::algorithm::Radix4;
use rustfft::FFT;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

pub struct FFTModule {
    sample_rate: f32,
    window_size: u32,
    fft_output: Vec<Complex<f32>>,
    fft: Radix4<f32>,
}

impl FFTModule {
    pub fn new(sample_rate: f64, window_size: u32) -> Self {
        Self {
            sample_rate: sample_rate as f32,
            window_size: window_size,
            fft_output: vec![Complex::zero(); window_size as usize],
            fft: Radix4::new(window_size as usize, false),
        }
    }

    fn A(&self, b: usize) -> f32 {
        let f = b as f32 * self.sample_rate / self.window_size as f32;
        let a = 12194.0*12194.0*f.powf(4.0) / ((f*f+20.6*20.6) * ((f*f+107.7*107.7)*(f*f+737.9*737.9)).sqrt() * (f*f+12194.0*12194.0));
        // println!("{}: {}", b, a);
        a
    }
}

impl ProcessorModule for FFTModule {
    fn process(&mut self, input: &Vec<f32>) -> Vec<f32> {
        let mut fft_input: Vec<Complex<f32>> = input.iter().enumerate().map(|(index, s)| {
            let a = 25.0 / 46.0;
            let window = a - (1.0 - a) * ((2.0 * std::f32::consts::PI * index as f32) / input.len() as f32).cos();
            
            Complex::new(*s * window, 0.0)
        }).collect();
        self.fft.process(&mut fft_input, &mut self.fft_output);
        self.fft_output[0..self.fft_output.len() / 4].iter().enumerate().map(|(index, c)| (10.0 * (c.re*c.re + c.im*c.im).log(10.0)).max(0.0)).collect()
    }
}
