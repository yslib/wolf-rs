mod camera;
mod io;
mod math;
mod palette;
mod resource;
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

    use crate::{
        camera::WolfCamera, math::Grid2, resource::ResoucesSystem, wolf_asset::WolfAssetCache,
    };

    use super::canvas;
    use super::math::{Bound2, Vec2};
    use canvas::{Canvas, Color, From2DData, Texture2D};

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

        let mut rs = WolfAssetCache::open();

        let map = rs.get_or_read_map(1, 1, 0);

        let min = Vec2::<i32>::new(0, 0);

        let max = Vec2::<i32>::new(64, 64);
        let b = Bound2::<i32>::new(min, max);
        let g = Grid2::new(b);

        let mut cam = WolfCamera::new();

        let mut wall_color_index = vec![0u8; canvas_res.1];

        'running: loop {
            screen.clear();
            for col in 0..canvas_res.0 {
                let half_width = canvas_res.0 as f32/ 2_f32;
                let angle =
                    cam.get_view_angle() + ((col as f32 - half_width) / canvas_res.0 as f32) * cam.fov.to_radians();
                for v in g.iter(cam.pos, angle) {
                    let cell_index = v.0;
                    let isect_pos = cam.dir * v.1 + cam.pos;

                    let dxdy = isect_pos - cam.pos;
                    let correction_depth = dxdy.x * angle.cos() - dxdy.y * angle.sin();
                    let max_depth = 15_f32;
                    let wall_height = if correction_depth < 0_f32 {
                        canvas_res.1
                    } else if correction_depth >= max_depth {
                        0_usize
                    } else {
                        ((1_f32 - correction_depth / max_depth) * canvas_res.1 as f32) as usize
                    };

                    let isect_index = (cell_index.y * 64 + cell_index.x) as usize;
                    let a = map[isect_index];

                    if a < 107 {
                        //draw column
                        wall_color_index.fill(30);
                        screen.set_wall(col, &wall_color_index[0..wall_height]);
                        break;
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
                        println!("S");
                        cam.advance(-1);
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::A),
                        ..
                    } => {
                        println!("A");
                        cam.rotate(-1f32);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::D),
                        ..
                    } => {
                        println!("D");
                        cam.rotate(1f32);
                    },
                    _ => {}
                }
            }

            // rendering
        }
    }
}
