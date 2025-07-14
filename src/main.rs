mod framebuffer;
mod line;
mod polygon;

use framebuffer::FrameBuffer;
use line::line;
use polygon::fill_polygon;
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;
    let background_color = Color::BLACK;
    let mut framebuffer = FrameBuffer::new(width, height, background_color);

    
    let polygon1 = vec![
        Vector2::new(165.0, 380.0),
        Vector2::new(185.0, 360.0),
        Vector2::new(180.0, 330.0),
        Vector2::new(207.0, 345.0),
        Vector2::new(233.0, 330.0),
        Vector2::new(230.0, 360.0),
        Vector2::new(250.0, 380.0),
        Vector2::new(220.0, 385.0),
        Vector2::new(205.0, 410.0),
        Vector2::new(193.0, 383.0),
    ];

    framebuffer.set_color(Color::YELLOW);
    fill_polygon(&mut framebuffer, &polygon1);

    framebuffer.set_color(Color::WHITE);
    for i in 0..polygon1.len() {
        let start = polygon1[i];
        let end = polygon1[(i + 1) % polygon1.len()];
        line(&mut framebuffer, start, end);
    }




    let output_file = "out.png";
    framebuffer
        .render_to_file(output_file)
        .expect("Failed to save framebuffer to file");
}
