#![feature(new_uninit)]
#![feature(const_trait_impl)]
#![feature(const_refs_to_cell)]
#![feature(effects)]
#![feature(generic_const_exprs)]
#![feature(maybe_uninit_uninit_array_transpose)]

/// TODO: (in subjective order of importance)
/// documentation
/// unit testing
/// mapping
/// map self
/// ability to resize
/// insert images onto other images
/// ability to remove stride
/// generic_const_exprs feature gate
/// new_uninit feature gate
/// const_trait feature gate (no need for const_refs_to_cell)
/// POST 1.0:
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
