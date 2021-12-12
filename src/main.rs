#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused)]
mod camera;
mod io;
mod math;
mod palette;
mod resource;
mod texture;
mod trait_def;
mod wolf_asset;
//mod canvas;

mod canvas;

fn main() -> Result<(), i32> {
    Ok(())
}

#[cfg(test)]
mod dda_test {

    extern crate sdl2;
    extern crate stb_image;

    use sdl2::keyboard::Scancode;
    use sdl2::sys::KeyCode;
    use sdl2::{event::Event, keyboard::Keycode, surface::Surface};
    use std::ops::Range;

    use crate::{camera::WolfCamera, math::Grid2, resource::MapCache, resource::TexturePool};

    use super::canvas;
    use super::math::{Bound2, Vec2};
    use canvas::{Canvas, Color};


    #[test]
    fn dda_test() {
        // drawing a test scene

        let sdl_context = sdl2::init().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let vid = sdl_context.video().unwrap();

        let canvas_res = (400, 300);
        let mut screen = Canvas::new(canvas_res);

        let window = vid
            .window("Wolf", 1920, 1080)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let default_format = canvas.default_pixel_format();

        let mut rs = MapCache::load();
        let map = rs.read_map(1, 1, 0);
        let min = Vec2::<i32>::new(0, 0);
        let max = Vec2::<i32>::new(map.width, map.height);
        let b = Bound2::<i32>::new(min, max);
        let g = Grid2::new(b);
        let mut cam = WolfCamera::new(Vec2::new(32_f32, 32_f32), Vec2::new(1.0, 1.0), 45f32);
        let mut wall_color_index = vec![0u8; canvas_res.1];

        let mut tp = TexturePool::open();

        'running: loop {
            //screen.clear();
            screen.draw_ceil_and_floor();

            let half_width = canvas_res.0 as f32 / 2_f32;
            for col in 0..canvas_res.0 {
                let angle = cam.get_view_angle()
                    + ((col as f32 - half_width) / canvas_res.0 as f32) * cam.fov.to_radians();

                for v in g.iter(cam.pos, angle) {
                    let cell_index = v.0;
                    let isect_pos = cam.dir * v.1 + cam.pos;

                    let dxdy = isect_pos - cam.pos;
                    let mut corrected = (dxdy.x * angle.cos() - dxdy.y * angle.sin());
                    let max_depth = 20_f32;
                    let a = Vec2::new(0f32, 200f32);
                    let b = Vec2::new(20f32, 1f32);
                    let k = (b.y - a.y) / (b.x - a.x);
                    let c = (b.x * a.y - a.x * b.y) / (b.x - a.x);
                    if corrected < 5f32 {
                        corrected = 5f32;
                    }
                    // let wall_height = if corrected < a.x {
                    //     a.y
                    // } else if corrected >= b.x {
                    //     b.y
                    // } else {
                    //     // ((1_f32 - corrected / max_depth) * canvas_res.1 as f32) as usize
                    //     // canvas_res.1 / corrected as usize
                    //     k * corrected + c
                    // } as usize;

                    let mut wall_height = (1000.0f32 / corrected) as usize;

                    if wall_height > canvas_res.1 {
                        wall_height = canvas_res.1
                    }

                    let isect_index = (cell_index.y * 64 + cell_index.x) as usize;
                    let a = map.data[isect_index];
                    if a <= 107 {
                        //draw column
                        let tex = tp.get_texture(a as usize);
                        //wall_color_index.fill(a as u8);
                        let u = v.2;
                        wall_color_index = (0..wall_height).map(|h|{
                            tex.sample_nearest(u,1f32/wall_height as f32 * h as f32)
                        }).collect();

                        screen.set_wall(col, &wall_color_index[0..wall_height]);
                        break;
                    }
                }
            }

            // minimap
            {
                (0..64).for_each(|x| {
                    (0..64).for_each(|y| {
                        let isect_index = (y * 64 + x) as usize;
                        let color = map.data[isect_index];
                        screen.set_pixel_by_color_index(x, y, color as u8);
                    })
                });
                // player
                let p = cam.pos;
                screen.set_pixel_by_color_index(p.x as u32, p.y as u32, 50);

                // view range
                for col in 0..canvas_res.0 {
                    let angle = cam.get_view_angle()
                        + ((col as f32 - half_width) / canvas_res.0 as f32) * cam.fov.to_radians();
                    for v in g.iter(cam.pos, angle) {
                        let cell_index = v.0;
                        let isect_index = (cell_index.y * 64 + cell_index.x) as usize;
                        let a = map.data[isect_index];
                        screen.set_pixel_by_color_index(v.0.x as u32, v.0.y as u32, 50);
                        if a <= 107 {
                            break;
                        }
                    }
                }
            }

            let surface = Surface::from_data(
                screen.buffer_as_mut(),
                canvas_res.0 as u32,
                canvas_res.1 as u32,
                default_format.byte_size_of_pixels(canvas_res.0) as u32,
                default_format,
            )
            .unwrap();

            let texture = texture_creator
                .create_texture_from_surface(surface)
                .unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();

            // handling events
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::W),
                        ..
                    } => {
                        println!("W");
                        cam.advance(1);
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::S),
                        ..
                    } => {
                        cam.advance(-1);
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::A),
                        ..
                    } => {
                        cam.rotate(-1f32);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::D),
                        ..
                    } => {
                        cam.rotate(1f32);
                    }
                    Event::MouseMotion {
                        x, y, xrel, yrel, ..
                    } => {
                        cam.advance(-yrel);
                        cam.rotate(xrel as f32);
                    }
                    _ => {}
                }
            }

            // rendering
        }
    }
}
