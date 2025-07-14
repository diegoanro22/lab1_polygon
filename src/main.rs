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

    
    let polygon2 = vec![
        Vector2::new(321.0,335.0),
        Vector2::new(288.0,286.0),
        Vector2::new(339.0, 251.0),
        Vector2::new(374.0,302.0),
    ];

    framebuffer.set_color(Color::BLUE);
    fill_polygon(&mut framebuffer, &polygon2);

    framebuffer.set_color(Color::WHITE);
    for i in 0..polygon2.len() {
        let start = polygon2[i];
        let end = polygon2[(i + 1) % polygon2.len()];
        line(&mut framebuffer, start, end);
    }



    let output_file = "out.png";
    framebuffer.render_to_file(output_file)
        .expect("Failed to save framebuffer to file");
}