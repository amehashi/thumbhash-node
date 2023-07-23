#![deny(clippy::all)]

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use thumbhash::{
  rgba_to_thumb_hash as rgba_to_thumb_hash_vendor,
  thumb_hash_to_approximate_aspect_ratio as thumb_hash_to_approximate_aspect_ratio_vendor,
  thumb_hash_to_average_rgba as thumb_hash_to_average_rgba_vendor,
  thumb_hash_to_rgba as thumb_hash_to_rgba_vendor,
};

#[napi(js_name = "rgbaToThumbHash")]
/// Encodes an RGBA image to a ThumbHash. RGB should not be premultiplied by A.
pub fn rgba_to_thumb_hash(w: u32, h: u32, rgba: Uint8Array) -> Uint8Array {
  rgba_to_thumb_hash_vendor(w as usize, h as usize, &rgba).into()
}

#[napi(js_name = "thumbHashToApproximateAspectRatio")]
/// Extracts the approximate aspect ratio of the original image.
/// An error will be returned if the input is too short.
pub fn thumb_hash_to_approximate_aspect_ratio(hash: Uint8Array) -> Result<f32> {
  thumb_hash_to_approximate_aspect_ratio_vendor(&hash)
    .map_err(|_| Error::new(Status::InvalidArg, String::from("invalid arg")))
}

#[napi(object)]
pub struct RGBA {
  pub r: f64,
  pub g: f64,
  pub b: f64,
  pub a: f64,
}

#[napi(js_name = "thumbHashToAverageRGBA")]
/// Extracts the average color from a ThumbHash.
/// Returns the RGBA values where each value ranges from 0 to 1.
/// RGB is not be premultiplied by A.
/// An error will be returned if the input is too short.
pub fn thumb_hash_to_average_rgba(hash: Uint8Array) -> Result<RGBA> {
  match thumb_hash_to_average_rgba_vendor(&hash) {
    Ok((r, g, b, a)) => Ok(RGBA {
      r: r as f64,
      g: g as f64,
      b: b as f64,
      a: a as f64,
    }),
    Err(_) => Err(Error::new(Status::InvalidArg, String::from("invalid arg"))),
  }
}

#[napi(object)]
pub struct Image {
  pub width: u32,
  pub height: u32,
  pub rgba: Uint8Array,
}

#[napi(js_name = "thumbHashToRGBA")]
/// Decodes a ThumbHash to an RGBA image.
/// RGB is not be premultiplied by A.
/// Returns the width, height, and pixels of the rendered placeholder image.
/// An error will be returned if the input is too short.
pub fn thumb_hash_to_rgba(hash: Uint8Array) -> Result<Image> {
  match thumb_hash_to_rgba_vendor(&hash) {
    Ok((w, h, rgba)) => Ok(Image {
      width: w as u32,
      height: h as u32,
      rgba: rgba.into(),
    }),
    Err(_) => Err(Error::new(Status::InvalidArg, String::from("invalid arg"))),
  }
}
