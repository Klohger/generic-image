use generic_image::Image;
use std::{fs::File, io::{BufReader, BufWriter}, mem::size_of};

fn main() {
    let mut image = unsafe { Image::uninit(16, 16) };

    image.fill([255u8, 0, 0]);
    image[(1, 0)] = [0, 0, 0];
    image[(0, 1)][1] = 255;
    let region = image.region((0, 0)..(2, 2)).unwrap();
    {
        let mut encoder = png::Encoder::new(
            BufWriter::new(File::create("test.png").unwrap()),
            region.width() as u32,
            region.height() as u32,
        );
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_compression(png::Compression::Best);
        encoder.set_adaptive_filter(png::AdaptiveFilterType::Adaptive);
        encoder.set_filter(png::FilterType::Paeth);
        encoder.set_depth(png::BitDepth::Eight);
        let mut encoder = encoder.write_header().unwrap();

        let buf = region.reallocated().into_source();
        encoder
            .write_image_data(unsafe {
                std::slice::from_raw_parts(buf.as_ptr().cast(), buf.len() * size_of::<[u8; 3]>())
            })
            .unwrap();
        encoder.finish().unwrap();
    }
    {
        let mut imported = unsafe { Image::<Box<_>, [u8;3]>::uninit(2, 2) };
        let imported_source = imported.source_mut();

        let mut decoder = png::Decoder::new(BufReader::new(File::open("test.png").unwrap())).read_info().unwrap();
        decoder.next_frame(unsafe { std::slice::from_raw_parts_mut(imported_source.as_mut_ptr().cast(), imported_source.len() * size_of::<[u8;3]>()) }).unwrap();

        assert_eq!(region, imported);
    }
}
