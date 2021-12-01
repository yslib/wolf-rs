extern crate sdl2;
extern crate stb_image;
use std::{borrow::{Borrow, BorrowMut}, alloc::dealloc};

use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, surface::Surface};

mod canvas;
use canvas::{Canvas, Color, From2DData, Texture2D};

mod application;
mod map;
mod math;
mod trait_def;

mod wolf_asset;

fn main() {
    let cur_dir = std::env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let cur_exe = std::env::current_exe()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let vid = sdl_context.video().unwrap();

    //let mut screen = Canvas::new(res);

    let mut tex_2d: Texture2D<u8> = match stb_image::image::load_with_depth(
        wolf_asset::asset_filename(&["resources", "test.jpg"]),
        4,
        false,
    ) {
        stb_image::image::LoadResult::Error(err) => {
            panic!("{}", err);
        }
        stb_image::image::LoadResult::ImageU8(image) => Texture2D::<u8>::from_data(
            image.data,
            image.width as u32,
            image.height as u32,
            image.depth as u8,
        ),
        stb_image::image::LoadResult::ImageF32(_image) => {
            panic!("float image is not supported");
        }
    };

    let window = vid
        .window("Wolf", tex_2d.width, tex_2d.height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let default_format = canvas.default_pixel_format();
    'running: loop {
        let mut b = tex_2d.buffer();
        let a = b.borrow_mut();
        let surface = Surface::from_data(
            a.as_mut_slice(),
            tex_2d.width as u32,
            tex_2d.height as u32,
            default_format.byte_size_of_pixels(tex_2d.width as usize) as u32,
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
                _ => {}
            }
        }
    }
}
