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

    let polygon4 = vec![
        Vector2::new(413.00, 177.00),
        Vector2::new(448.00, 159.00),
        Vector2::new(502.00, 88.00),
        Vector2::new(553.00, 53.00),
        Vector2::new(535.00, 36.00),
        Vector2::new(676.00, 37.00),
        Vector2::new(660.00, 52.00),
        Vector2::new(750.00, 145.00),
        Vector2::new(761.00, 179.00),
        Vector2::new(672.00, 192.00),
        Vector2::new(659.00, 214.00),
        Vector2::new(615.00, 214.00),
        Vector2::new(632.00, 230.00),
        Vector2::new(580.00, 230.00),
        Vector2::new(597.00, 215.00),
        Vector2::new(552.00, 214.00),
        Vector2::new(517.00, 144.00),
        Vector2::new(466.00, 180.00),
    ];

    let polygon5 = vec![
        Vector2::new(682.00, 175.00),
        Vector2::new(708.00, 120.00),
        Vector2::new(735.00, 148.00),
        Vector2::new(739.00, 170.00),
    ];

    framebuffer.set_color(Color::GREEN);
    fill_polygon(&mut framebuffer, &polygon4);

    framebuffer.set_color(background_color);
    fill_polygon(&mut framebuffer, &polygon5);

    framebuffer.set_color(Color::WHITE);
    for i in 0..polygon4.len() {
        let start = polygon4[i];
        let end = polygon4[(i + 1) % polygon4.len()];
        line(&mut framebuffer, start, end);
    }
    for i in 0..polygon5.len() {
        let start = polygon5[i];
        let end = polygon5[(i + 1) % polygon5.len()];
        line(&mut framebuffer, start, end);
    }

    let output_file = "out.png";
    framebuffer
        .render_to_file(output_file)
        .expect("Failed to save framebuffer to file");
}
