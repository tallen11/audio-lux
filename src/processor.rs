use crate::processor_module::ProcessorModule;

pub type ProcessorOutput = Vec<f32>;

pub struct Processor {
    resolution: u32,
    window_size: u32,
    modules: Vec<Box<ProcessorModule>>,
}

impl Processor {
    pub fn new(window_size: u32, resolution: u32) -> Self {
        Self {
            resolution: resolution,
            window_size: window_size,
            modules: Vec::new(),
        }
    }

    pub fn add_module(&mut self, module: Box<ProcessorModule>) {
        self.modules.push(module);
    }

    pub fn process(&mut self, buffer: &[f32]) -> ProcessorOutput {
        let mut data: Vec<f32> = buffer.to_vec();
        for module in self.modules.iter_mut() {
            data = module.process(&data);
        }

        data
    }
}
