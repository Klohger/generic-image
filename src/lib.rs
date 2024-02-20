#![feature(new_uninit)]
/// IMPROVEMENTS: (in subjective order of importance)
/// documentation
/// ability to resize
/// mutable iterator
/// insert images onto other images
/// new_uninit feature gate
/// basic drawing maybe
/// png feature gate
/// std feature gate
pub mod cursor;
pub mod error;
mod image;
pub mod index;
pub use cursor::ImageCursor;
pub use error::Error;
pub use image::Image;
pub use index::ImageIndex;
