extern crate sdl2;
extern crate stb_image;
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, surface::Surface};

mod canvas;
use canvas::{Canvas, Color, From2DData, Texture2D};

mod math;
mod trait_def;

mod wolf_asset;

fn main() {
	let cur_dir = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
	let cur_exe = std::env::current_exe().unwrap().into_os_string().into_string().unwrap();
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let vid = sdl_context.video().unwrap();

    let res: (usize, usize) = (800, 600);

    let mut screen = Canvas::new(res);
    for y in 0..res.1 {
        for x in 0..res.0 {
            if y == 0 {
                screen.set_pixel(
                    x as u32,
                    y as u32,
                    Color {
                        r: 255,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                );
            } else {
                screen.set_pixel(
                    x as u32,
                    y as u32,
                    Color {
                        r: 0,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                );
            }
        }
    }

    let window = vid
        .window("Wolf", res.0 as u32, res.1 as u32)
        .position_centered()
        .build()
        .unwrap();

    // let tex_2d: Texture2D<f32> = match stb_image::image::load(
    //     "/home/ysl/Code/rusterizer/test/obj/diablo3_pose/diablo3_pose_diffuse.tga",
    // ) {
    //     stb_image::image::LoadResult::Error(err) => {
    //         panic!("{}", err);
    //     }
    //     stb_image::image::LoadResult::ImageU8(image) => Texture2D::<f32>::from_data(
    //         &image.data,
    //         image.width as u32,
    //         image.height as u32,
    //         image.depth as u8,
    //     ),
    //     stb_image::image::LoadResult::ImageF32(_image) => {
    //         panic!("float image is not supported");
    //     }
    // };

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    'running: loop {
        let surface = Surface::from_data(
            screen.buffer_as_mut(),
            res.0 as u32,
            res.1 as u32,
            1,
            PixelFormatEnum::RGB888,
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
                _ => {}
            }
        }
    }
}
