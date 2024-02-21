use core::mem::size_of;
use generic_image::Image;
use std::fmt::Debug;
fn main() {
    {
        let mut image = unsafe { Image::uninit(16, 16) };
        image.fill([255, 0, 0]);
        image[[1, 0]] = [0, 0, 0];
        image[(0, 1)][1] = 255;

        tast_png(image.region([0, 0]..[7, 3]).unwrap());
    }
}

fn tast_png<Source>(image: Image<Source, [u8; 3]>)
where
    Source: AsRef<[[u8; 3]]> + Debug,
{
    let mut png_buf = Vec::new();

    let mut encoder = png::Encoder::new(&mut png_buf, image.width() as u32, image.height() as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_compression(png::Compression::Best);
    encoder.set_adaptive_filter(png::AdaptiveFilterType::Adaptive);
    encoder.set_filter(png::FilterType::Paeth);
    encoder.set_depth(png::BitDepth::Eight);
    let mut encoder = encoder.write_header().unwrap();
    let buf = unsafe { image.cursor().read_into_box().unwrap() };
    println!("original: {buf:02X?}");
    encoder.write_image_data(&buf).unwrap();
    drop(buf);
    encoder.finish().unwrap();
    println!("encoded:  {png_buf:02X?}");
    let mut imported = unsafe { Image::uninit(image.width(), image.height()) };

    let mut decoder = png::Decoder::new(png_buf.as_slice()).read_info().unwrap();

    let imported_slice = unsafe {
        std::slice::from_raw_parts_mut(
            imported.source_mut().as_mut_ptr() as *mut u8,
            imported.source().len() * size_of::<[u8; 3]>(),
        )
    };
    decoder.next_frame(imported_slice).unwrap();
    println!("decoded:  {imported_slice:02X?}");
    decoder.finish().unwrap();
    drop(decoder);
    assert_eq!(image, imported);
}
