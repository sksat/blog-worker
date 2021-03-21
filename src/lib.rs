mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

use image::{Rgb, RgbImage};
use imageproc::{drawing, rect};
use rusttype::{Font, Scale};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn ogp_image(font: &[u8]) -> Vec<u8> {
    //let mut v = Vec::new();
    //v.push(font[0]);
    //return v;
    //let font = Vec::from(include_bytes!("../font/DejaVuSans.ttf") as &[u8]);
    let font = Font::try_from_bytes(font).unwrap();

    let height = 12.4;
    let scale = Scale {
        x: height * 2.0,
        y: height * 2.0,
    };

    let mut image = RgbImage::new(200, 200);
    drawing::draw_filled_rect_mut(
        &mut image,
        rect::Rect::at(0, 0).of_size(200, 200),
        Rgb([255u8, 255u8, 255u8]),
    );
    drawing::draw_text_mut(
        &mut image,
        Rgb([0u8, 0u8, 255u8]),
        10,
        10,
        scale,
        &font,
        "お〜じ〜ぴ〜",
    );

    let image = image::DynamicImage::ImageRgb8(image);

    let mut bytes: Vec<u8> = Vec::new();
    let _ = image
        .write_to(&mut bytes, image::ImageOutputFormat::Png)
        .unwrap();

    bytes
}
