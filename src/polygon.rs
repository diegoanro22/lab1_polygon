use raylib::prelude::*;
use crate::framebuffer::FrameBuffer;

pub fn fill_polygon(framebuffer: &mut FrameBuffer, verts: &[Vector2]) {
    // 1) Encuentra el mínimo y máximo Y enteros
    let ys: Vec<i32> = verts.iter().map(|v| v.y as i32).collect();
    let y_min = *ys.iter().min().unwrap();
    let y_max = *ys.iter().max().unwrap();

    // 2) Por cada scanline
    for y in y_min..=y_max {
        // recolecta intersecciones X
        let mut xints: Vec<f32> = Vec::new();

        for i in 0..verts.len() {
            let a = verts[i];
            let b = verts[(i + 1) % verts.len()];
            let y1 = a.y as i32;
            let y2 = b.y as i32;

            // ignorar horizontales
            if y1 == y2 { continue; }

            // ¿this scanline intersecta el segmento?
            if (y >= y1.min(y2)) && (y < y1.max(y2)) {
                let t = (y as f32 - a.y) / (b.y - a.y);
                let x = a.x + t * (b.x - a.x);
                xints.push(x);
            }
        }

        // 3) ordenar e iterar en pares
        xints.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut i = 0;
        while i + 1 < xints.len() {
            let x_start = xints[i].ceil() as i32;
            let x_end   = xints[i+1].floor() as i32;
            for x in x_start..=x_end {
                framebuffer.set_pixel(x, y);
            }
            i += 2;
        }
    }
}
