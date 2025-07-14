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

    let polygon2 = vec![
        Vector2::new(321.0, 335.0),
        Vector2::new(288.0, 286.0),
        Vector2::new(339.0, 251.0),
        Vector2::new(374.0, 302.0),
    ];

    framebuffer.set_color(Color::BLUE);
    fill_polygon(&mut framebuffer, &polygon2);

    framebuffer.set_color(Color::WHITE);
    for i in 0..polygon2.len() {
        let start = polygon2[i];
        let end = polygon2[(i + 1) % polygon2.len()];
        line(&mut framebuffer, start, end);
    };
    
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
    };
        
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

    let output_file = "out.bmp";
    framebuffer
        .render_to_file(output_file)
        .expect("Failed to save framebuffer to file");
}
