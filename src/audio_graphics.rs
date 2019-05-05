use std::sync::{Mutex, Arc};

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use crate::processor::ProcessorOutput;

pub struct AudioGraphics {
    width: u32,
    height: u32,
    gl: GlGraphics,
    output: Option<Vec<f32>>,
}

impl AudioGraphics {
    pub fn new(width: u32, height: u32) -> Self {        
        Self {
            width: width,
            height: height,
            gl: GlGraphics::new(OpenGL::V3_2),
            output: None,
        }
    }

    pub fn post(&mut self, buffer: &Vec<f32>) {
        self.output = Some(buffer.iter().map(|sample| *sample).collect());
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let output = &self.output;
        let screen_width = self.width as f64;
        let screen_height = self.height as f64;

        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);

            if let Some(output) = output {
                let width = screen_width / output.len() as f64;
                let space = 1.0;

                for (index, value) in output.iter().enumerate() {
                    let color = [1.0, 1.0, 1.0, 1.0];

                    let rect = rectangle::rectangle_by_corners(width * (index as f64),
                                                               screen_height,
                                                               width * (index as f64) + (width - space),
                                                               screen_height - (*value as f64) * 10.0);
                        
                    let r = Rectangle::new(color);
                    r.draw(rect, &draw_state::DrawState::default(), c.transform, gl);
                }
            }
        });
    }
}
