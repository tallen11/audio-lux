extern crate portaudio;
extern crate rustfft;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

pub mod processor;
pub mod audio_graphics;
pub mod processor_module;
pub mod fft_module;
pub mod gamma_module;
pub mod smoothing_module;
pub mod output_module;

use std::sync::{Mutex, Arc};

use portaudio as pa;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use processor::Processor;
use audio_graphics::AudioGraphics;

use fft_module::FFTModule;
use gamma_module::GammaModule;
use smoothing_module::SmoothingModule;
use output_module::{OutputModule, OutputModuleMode};

const SAMPLE_RATE: f64 = 44100.0;
const WINDOW_SIZE: u32 = 512;
const CHANNELS: i32 = 1;
const INTERLEAVED: bool = true;

const OUTPUT_RESOLUTION: u32 = 256;

fn main() {
    let pa = pa::PortAudio::new().unwrap();

    let sound_siphon_device = pa.devices().unwrap().find_map(|device| {
        let (idx, info) = device.unwrap();
        if info.name == "Spotify" {
            return Some(idx);
        }

        None
    }).unwrap();

    let device_info = pa.device_info(sound_siphon_device).unwrap();
    let latency = device_info.default_low_input_latency;
    let input_params = pa::StreamParameters::<f32>::new(sound_siphon_device, CHANNELS, INTERLEAVED, latency);

    pa.is_input_format_supported(input_params, SAMPLE_RATE).unwrap();

    let settings = pa::InputStreamSettings::new(input_params, SAMPLE_RATE, WINDOW_SIZE);

    let gl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Audio Visualizer", [512, 480]).opengl(gl)
                                                                                .exit_on_esc(true)
                                                                                .build()
                                                                                .unwrap();

    let mut ag_process = Arc::new(Mutex::new(AudioGraphics::new(512, 480)));
    let mut ag_render = Arc::clone(&ag_process);
    
    let mut processor = Processor::new(WINDOW_SIZE, OUTPUT_RESOLUTION);
    processor.add_module(Box::new(FFTModule::new(SAMPLE_RATE, WINDOW_SIZE)));
    // processor.add_module(Box::new(GammaModule::new(2.0, SAMPLE_RATE)));
    processor.add_module(Box::new(SmoothingModule::new(0.9)));
    processor.add_module(Box::new(OutputModule::new(OutputModuleMode::Max, 4)));

    let mut stream = pa.open_non_blocking_stream(settings, move |pa::stream::InputCallbackArgs { buffer, frames, .. }| {
        {
            let output = processor.process(buffer);
            let mut ag = ag_process.lock().unwrap();
            ag.post(&output);
        }
        
        pa::Continue
    }).unwrap();

    stream.start().unwrap();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            let mut ag = ag_render.lock().unwrap();
            ag.render(&r);
        }
    }

    stream.close();
}
