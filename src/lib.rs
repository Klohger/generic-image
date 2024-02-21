#![feature(new_uninit)]
#![feature(const_trait_impl)]
#![feature(const_refs_to_cell)]
#![feature(effects)]
/// IMPROVEMENTS: (in subjective order of importance)
/// documentation
/// ability to resize
/// insert images onto other images
/// new_uninit feature gate
/// png feature gate
/// basic drawing maybe
/// improve cursor maybe.
/// chunks iterator maybe
/// std feature gate
pub mod cursor;
pub mod error;
pub mod image;
pub mod index;
pub mod iterator;
pub use cursor::ImageCursor;
pub use error::Error;
pub use image::Image;
pub use index::ImageIndex;
