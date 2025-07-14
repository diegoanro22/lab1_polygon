mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::FrameBuffer;
use line::line;
use polygon::fill_polygon;

fn main() {
    let width = 800;
    let height = 600;
    let background_color = Color::new(50, 50, 100, 255);
    let mut framebuffer = FrameBuffer::new(width,height, background_color);

    framebuffer.clear();



    let output_file = "out.bmp";
    framebuffer.render_to_file(output_file)
        .expect("Failed to save framebuffer to file");
}