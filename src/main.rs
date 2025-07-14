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
    let background_color = Color::BLACK;
    let mut framebuffer = FrameBuffer::new(width,height, background_color);


    let polygon3 = vec![
        Vector2::new(377.0,249.0),
        Vector2::new(411.0,197.0),
        Vector2::new(436.0, 249.0),
    ];

    framebuffer.set_color(Color::RED);
    fill_polygon(&mut framebuffer, &polygon3);

    framebuffer.set_color(Color::WHITE);
    for i in 0..polygon3.len() {
        let start = polygon3[i];
        let end = polygon3[(i + 1) % polygon3.len()];
        line(&mut framebuffer, start, end);
    }


    let output_file = "out.png";
    framebuffer.render_to_file(output_file)
        .expect("Failed to save framebuffer to file");
}