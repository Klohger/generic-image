#![feature(new_uninit)]
use core::{
    marker::PhantomData,
    mem::size_of,
    ops::{Index, IndexMut, Range},
};
/// IMPROVEMENTS:
/// proper error handling
/// ability to resize
/// png feature gate
/// std feature gate
///
use std::{
    io::{Read, Seek, Write},
    rc::Rc,
    sync::Arc,
};

pub struct Image<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    width: usize,
    height: usize,
    stride: usize,
    source: Source,
    _p: PhantomData<Pixel>,
}

impl<Source, Pixel> AsMut<Self> for Image<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<Source, Pixel> AsRef<Image<Source, Pixel>> for Image<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    fn as_ref(&self) -> &Image<Source, Pixel> {
        self
    }
}
impl<Pixel> Image<Box<[Pixel]>, Pixel> {
    pub fn empty_box(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            stride: width,
            source: unsafe { Box::new_zeroed_slice(width * height).assume_init() },
            _p: PhantomData,
        }
    }
    pub unsafe fn uninit(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            stride: width,
            source: Box::new_uninit_slice(width * height).assume_init(),
            _p: PhantomData,
        }
    }
}
impl<Pixel> Image<Rc<[Pixel]>, Pixel> {
    pub fn empty_rc(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            stride: width,
            source: unsafe { Rc::new_zeroed_slice(width * height).assume_init() },
            _p: PhantomData,
        }
    }
    pub unsafe fn uninit_rc(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            stride: width,
            source: Rc::new_uninit_slice(width * height).assume_init(),
            _p: PhantomData,
        }
    }
}
impl<Pixel> Image<Arc<[Pixel]>, Pixel> {
    pub fn empty_arc(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            stride: width,
            source: unsafe { Arc::new_zeroed_slice(width * height).assume_init() },
            _p: PhantomData,
        }
    }
    pub unsafe fn uninit_arc(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            stride: width,
            source: Arc::new_uninit_slice(width * height).assume_init(),
            _p: PhantomData,
        }
    }
}
impl<Pixel> Image<Vec<Pixel>, Pixel> {
    pub fn empty_vec(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            stride: width,
            source: unsafe { Box::new_zeroed_slice(width * height).assume_init() }.into_vec(),
            _p: PhantomData,
        }
    }
    pub unsafe fn uninit_vec(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            stride: width,
            source: Box::new_uninit_slice(width * height)
                .assume_init()
                .into_vec(),
            _p: PhantomData,
        }
    }
}
impl<Pixel, Source> Index<(usize, usize)> for Image<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    type Output = Pixel;

    fn index(&self, index: (usize, usize)) -> &Pixel {
        self.source.as_ref().index(self.calc_index(index))
    }
}
impl<Pixel, Source> IndexMut<(usize, usize)> for Image<Source, Pixel>
where
    Source: AsMut<[Pixel]> + AsRef<[Pixel]>,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Pixel {
        let index = self.calc_index(index);

        self.source.as_mut().index_mut(index)
    }
}

impl<Pixel, Source> Image<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    const fn calc_index(&self, pos: (usize, usize)) -> usize {
        (pos.0 % self.width) + (pos.1 * self.stride)
    }

    pub fn from_source(width: usize, height: usize, source: Source) -> Option<Self> {
        if source.as_ref().len() < width * height {
            None
        } else {
            Some(Self {
                width,
                height,
                stride: width,
                source,
                _p: PhantomData,
            })
        }
    }
    pub fn from_source_with_stride(
        width: usize,
        height: usize,
        stride: usize,
        source: Source,
    ) -> Option<Self> {
        if source.as_ref().len() < stride * height {
            None
        } else {
            Some(Self {
                width,
                height,
                stride,
                source,
                _p: PhantomData,
            })
        }
    }
    pub fn region(&self, range: Range<(usize, usize)>) -> Option<Image<&[Pixel], Pixel>> {
        if range.start.0 < self.width
            && range.start.1 < self.width
            && range.end.0 < self.width
            && range.end.1 < self.height
            && range.start.0 <= range.end.0
            && range.start.1 <= range.end.1
        {
            let start_i = self.calc_index(range.start);
            let end_i = self.calc_index(range.end);
            Some(Image {
                width: range.end.0 - range.start.0,
                height: range.end.1 - range.start.1,
                stride: self.stride,
                source: &self.source.as_ref().index(start_i..end_i),
                _p: PhantomData,
            })
        } else {
            None
        }
    }
    pub const fn cursor(&self) -> ImageCursor<Source, Pixel, &Self> {
        ImageCursor::new(self)
    }
    pub const fn into_cursor(self) -> ImageCursor<Source, Pixel, Self> {
        ImageCursor::new(self)
    }
    pub const fn width(&self) -> usize {
        self.width
    }
    pub const fn height(&self) -> usize {
        self.height
    }
    pub const fn stride(&self) -> usize {
        self.stride
    }
    pub const fn source(&self) -> &Source {
        &self.source
    }
    pub fn into_source(self) -> Source {
        self.source
    }
}
impl<Pixel, Source> Image<Source, Pixel>
where
    Source: AsMut<[Pixel]> + AsRef<[Pixel]>,
{
    pub fn source_mut(&mut self) -> &mut Source {
        &mut self.source
    }
    pub fn region_mut(
        &mut self,
        range: Range<(usize, usize)>,
    ) -> Option<Image<&mut [Pixel], Pixel>> {
        if range.start.0 < self.width
            && range.start.1 < self.width
            && range.end.0 < self.width
            && range.end.1 < self.height
            && range.start.0 <= range.end.0
            && range.start.1 <= range.end.1
        {
            let start_i = (range.start.0 % self.width) + (range.start.1 * self.stride);
            let end_i = (range.end.0 % self.width) + (range.end.1 * self.stride);
            Some(Image {
                width: range.end.0 - range.start.0,
                height: range.end.1 - range.start.1,
                stride: self.stride,
                source: self.source.as_mut().index_mut(start_i..end_i),
                _p: PhantomData,
            })
        } else {
            None
        }
    }
    pub fn cursor_mut(&mut self) -> ImageCursor<Source, Pixel, &mut Self> {
        ImageCursor::new(self)
    }
}
impl<Pixel, Source> Image<Source, Pixel>
where
    Pixel: Clone,
    Source: AsMut<[Pixel]> + AsRef<[Pixel]>,
{
    pub fn fill(&mut self, pixel: Pixel) {
        self.source.as_mut().fill(pixel);
    }
}
pub struct ImageCursor<Source, Pixel, I: AsRef<Image<Source, Pixel>>>
where
    Source: AsRef<[Pixel]>,
{
    image: I,
    index: usize,
    _p: PhantomData<(Source, Pixel)>,
}

impl<Source, Pixel, I: AsRef<Image<Source, Pixel>>> Seek for ImageCursor<Source, Pixel, I>
where
    Source: AsRef<[Pixel]>,
{
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        let image = self.image.as_ref().width();
        let new_index = match pos {
            std::io::SeekFrom::Start(i) => i,
            std::io::SeekFrom::End(_) => self.image.as_ref().width(),
            std::io::SeekFrom::Current(_) => todo!(),
        };
        new_index
    }
}

impl<Source, Pixel, I: AsRef<Image<Source, Pixel>> + AsMut<Image<Source, Pixel>>> Write
    for ImageCursor<Source, Pixel, I>
where
    Source: AsRef<[Pixel]> + AsMut<[Pixel]>,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let image = self.image.as_mut();
        if self.index < image.stride * image.height * size_of::<Pixel>() {
            let current_row_index =
                (self.index / (image.stride * size_of::<Pixel>())) * image.stride;
            let last_element_of_row = current_row_index + image.width;
            let rest_of_row = image
                .source
                .as_mut()
                .index_mut(self.index..last_element_of_row * size_of::<Pixel>());
            let rest_of_row = unsafe {
                let len = rest_of_row.len() * size_of::<Pixel>();
                std::slice::from_raw_parts_mut(rest_of_row.as_mut_ptr().cast::<u8>(), len)
            };

            match buf.len().cmp(&rest_of_row.len()) {
                std::cmp::Ordering::Less => {
                    self.index += buf.len();
                    rest_of_row[..buf.len()].copy_from_slice(buf);
                    Ok(buf.len())
                }
                std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                    rest_of_row.copy_from_slice(&buf[..rest_of_row.len()]);
                    self.index = (current_row_index + image.stride) * size_of::<Pixel>();
                    Ok(rest_of_row.len())
                }
            }
        } else {
            Ok(0)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<Source, Pixel, I: AsRef<Image<Source, Pixel>>> ImageCursor<Source, Pixel, I>
where
    Source: AsRef<[Pixel]>,
{
    pub const fn image(&self) -> &I {
        &self.image
    }
    pub const fn index(&self) -> usize {
        self.index
    }
    pub const fn new(image: I) -> Self {
        Self {
            image,
            index: 0,
            _p: PhantomData,
        }
    }
}

impl<Pixel, Source, I: AsRef<Image<Source, Pixel>>> Read for ImageCursor<Source, Pixel, I>
where
    Source: AsRef<[Pixel]>,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let image = self.image.as_ref();
        if self.index < image.stride * image.height * size_of::<Pixel>() {
            let current_row_index =
                (self.index / (image.stride * size_of::<Pixel>())) * image.stride;
            let last_element_of_row = current_row_index + image.width;
            let rest_of_row = image
                .source
                .as_ref()
                .index(self.index..last_element_of_row * size_of::<Pixel>());
            let rest_of_row = unsafe {
                std::slice::from_raw_parts(
                    rest_of_row.as_ptr().cast::<u8>(),
                    rest_of_row.len() * size_of::<Pixel>(),
                )
            };
            match buf.len().cmp(&rest_of_row.len()) {
                std::cmp::Ordering::Less => {
                    self.index += buf.len();
                    buf.copy_from_slice(&rest_of_row[..buf.len()]);
                    Ok(buf.len())
                }
                std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                    buf[..rest_of_row.len()].copy_from_slice(rest_of_row);
                    self.index = (current_row_index + image.stride) * size_of::<Pixel>();
                    Ok(rest_of_row.len())
                }
            }
        } else {
            Ok(0)
        }
    }
}
