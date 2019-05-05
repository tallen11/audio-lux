use crate::processor_module::ProcessorModule;

pub enum OutputModuleMode {
    Average,
    Max,
    Min,
}

pub struct OutputModule {
    mode: OutputModuleMode,
    bucket_len: u32,
}

impl OutputModule {
    pub fn new(mode: OutputModuleMode, bucket_len: u32) -> Self {
        Self {
            mode: mode,
            bucket_len: bucket_len,
        }
    }
}

impl ProcessorModule for OutputModule {
    fn process(&mut self, input: &Vec<f32>) -> Vec<f32> {
        match self.mode {
            OutputModuleMode::Average => {
                let input_len = input.len() as u32;
                let mut output: Vec<f32> = Vec::new();

                let mut bucket_sum = 0.0;
                for (index, s) in input.iter().enumerate() {
                    if (index + 1) as u32 % self.bucket_len == 0 {
                        let o = bucket_sum / self.bucket_len as f32;
                        output.push(o);
                        bucket_sum = 0.0;
                    } else {
                        bucket_sum += s;
                    }
                }

                output
            }

            OutputModuleMode::Max => {
                let input_len = input.len() as u32;
                let mut output: Vec<f32> = Vec::new();

                let mut bucket_max = 0.0;
                for (index, s) in input.iter().enumerate() {
                    if (index + 1) as u32 % self.bucket_len == 0 {
                        output.push(bucket_max);
                        bucket_max = 0.0;
                    } else {
                        if *s > bucket_max {
                            bucket_max = *s;
                        }
                    }
                }

                output
            }

            OutputModuleMode::Min => {
                vec![0.0]
            }
        }
    }
}
