use crate::processor_module::ProcessorModule;

pub struct SmoothingModule {
    alpha: f32,
    previous_output: Option<Vec<f32>>,
}

impl SmoothingModule {
    pub fn new(alpha: f32) -> Self {
        Self {
            alpha: alpha,
            previous_output: None,
        }
    }
}

impl ProcessorModule for SmoothingModule {
    fn process(&mut self, input: &Vec<f32>) -> Vec<f32> {
        let mut has_infs = false;
        for s in input {
            if s.is_infinite() {
                has_infs = true;
                break;
            }
        }

        if has_infs {
            self.previous_output = Some(vec![0.0; input.len()]);
            return vec![0.0; input.len()];
        }

        if let Some(previous_output) = &self.previous_output {
            let mut output: Vec<f32> = input.iter().enumerate().map(|(index, s)| {
                let sp = previous_output[index];
                sp * self.alpha + s * (1.0 - self.alpha)
            }).collect();

            for i in 1..output.len() {
                output[i] = 0.35 * output[i-1] + 0.65 * output[i];
            }

            let mut sorted_output = output.to_vec();
                sorted_output.sort_by(|a,b| a.partial_cmp(b).unwrap());
                let med = sorted_output[sorted_output.len() / 2];

                for i in 0..output.len() {
                    output[i] = (output[i] - med).max(0.0);
                }

            // println!("{:?}", output);
            self.previous_output = Some(output.to_vec());

            return output;
        }

        self.previous_output = Some(input.to_vec());

        input.to_vec()
    }
}
