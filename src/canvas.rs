use crate::css::Color;
use crate::display::{DisplayCommand, DisplayList};
use crate::layout::Rect;
use std::iter::repeat;

#[derive(Debug)]
pub struct Canvas {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        let white: Color = "#FFFFFF".to_string();
        return Canvas {
            pixels: repeat(white).take(width * height).collect(),
            width: width,
            height: height,
        };
    }
    pub fn paint_item(&mut self, item: &DisplayCommand) {
        match item {
            DisplayCommand::SolidColor(color, rect) => {
                // Clip the rectangle to the canvas boundaries.
                let x0 = rect.x.clamp(0.0, self.width as f32) as usize;
                let y0 = rect.y.clamp(0.0, self.height as f32) as usize;
                let x1 = (rect.x + rect.width).clamp(0.0, self.width as f32) as usize;
                let y1 = (rect.y + rect.height).clamp(0.0, self.height as f32) as usize;

                for y in y0..=y1 {
                    for x in x0..=x1 {
                        // TODO: alpha compositing with existing pixel
                        self.pixels[x + y * self.width] = color.clone();
                    }
                }
            }
            _ => (),
        }
    }
}

pub fn paint(display_list: &DisplayList, bounds: Rect) -> Canvas {
    let mut canvas = Canvas::new(bounds.width as usize, bounds.height as usize);
    for item in display_list {
        canvas.paint_item(&item);
    }
    return canvas;
}
