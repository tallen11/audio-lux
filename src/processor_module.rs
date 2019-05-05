pub trait ProcessorModule {
    fn process(&mut self, input: &Vec<f32>) -> Vec<f32>;
}
