#![feature(new_uninit)]
/// IMPROVEMENTS:
/// proper error handling
/// ability to resize
/// png feature gate
/// std feature gate
/// new_uninit feature gate
/// mutable iterator
use core::{
    cmp,
    fmt::Debug,
    marker::PhantomData,
    mem::size_of,
    ops::{Index, IndexMut, Range},
};
use std::{
    io::{self, Read, Seek, Write},
    rc::Rc,
    sync::Arc,
};

pub trait ImageIndex<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    fn index(self, image: &Image<Source, Pixel>) -> usize;
    fn pos(&self, image: &Image<Source, Pixel>) -> [usize; 2];
}

impl<Source, Pixel> ImageIndex<Source, Pixel> for usize
where
    Source: AsRef<[Pixel]>,
{
    fn index(self, image: &Image<Source, Pixel>) -> usize {
        self
    }

    fn pos(&self, image: &Image<Source, Pixel>) -> [usize; 2] {
        image.index_to_pos(*self)
    }
}
impl<Source, Pixel> ImageIndex<Source, Pixel> for (usize, usize)
where
    Source: AsRef<[Pixel]>,
{
    fn index(self, image: &Image<Source, Pixel>) -> usize {
        image.pos_to_index(self.0, self.1)
    }

    fn pos(&self, image: &Image<Source, Pixel>) -> [usize; 2] {
        (*self).into()
    }
}
impl<Source, Pixel> ImageIndex<Source, Pixel> for [usize; 2]
where
    Source: AsRef<[Pixel]>,
{
    fn index(self, image: &Image<Source, Pixel>) -> usize {
        image.pos_to_index(self[0], self[1])
    }
    fn pos(&self, image: &Image<Source, Pixel>) -> [usize; 2] {
        *self
    }
}

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

impl<Source, Pixel> Debug for Image<Source, Pixel>
where
    Source: AsRef<[Pixel]> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("stride", &self.stride)
            .field("source", &self.source)
            .finish()
    }
}

impl<LSource, RSource, Pixel> PartialEq<Image<RSource, Pixel>> for Image<LSource, Pixel>
where
    Pixel: PartialEq,
    LSource: AsRef<[Pixel]>,
    RSource: AsRef<[Pixel]>,
{
    fn eq(&self, other: &Image<RSource, Pixel>) -> bool {
        self.iter().eq(other.iter())
    }
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
    pub unsafe fn zeroed(width: usize, height: usize) -> Self {
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
    pub unsafe fn zeroed_rc(width: usize, height: usize) -> Self {
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
    pub unsafe fn zeroed_arc(width: usize, height: usize) -> Self {
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
    pub unsafe fn zeroed_vec(width: usize, height: usize) -> Self {
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
impl<Source, Pixel, I: ImageIndex<Source, Pixel>> Index<I> for Image<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    type Output = Pixel;

    fn index(&self, index: I) -> &Pixel {
        self.source.as_ref().index(index.index(self))
    }
}
impl<Source, Pixel, I: ImageIndex<Source, Pixel>> IndexMut<I> for Image<Source, Pixel>
where
    Source: AsMut<[Pixel]> + AsRef<[Pixel]>,
{
    fn index_mut(&mut self, index: I) -> &mut Pixel {
        let index = index.index(self);

        self.source.as_mut().index_mut(index)
    }
}
impl<Source, Pixel> Image<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
    Pixel: Copy,
{
    pub fn reallocated(&self) -> Image<Box<[Pixel]>, Pixel> {
        let mut source = Box::new_uninit_slice(self.width * self.height);
        for (i, old_i) in (0..self.width).map(|i| (i * self.height, i * self.stride)) {
            unsafe {
                std::ptr::copy(
                    (&self.source.as_ref()[old_i]) as *const _,
                    source[i].as_mut_ptr(),
                    self.width,
                )
            };
        }
        Image {
            width: self.width,
            height: self.height,
            stride: self.width,
            source: unsafe { source.assume_init() },
            _p: PhantomData,
        }
    }
}
impl<Source, Pixel> Image<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    pub const fn pos_to_index(&self, x: usize, y: usize) -> usize {
        if !(x < self.width) {
            panic!("x coordinate was outside of image");
        }
        if !(y < self.height) {
            panic!("y coordinate was outside of image");
        }
        (x) + (y * self.stride)
    }
    pub const fn index_to_pos(&self, i: usize) -> [usize; 2] {
        [i % self.width, i / self.width]
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
    pub fn region(&self, range: Range<impl ImageIndex<Source, Pixel>>) -> Image<&[Pixel], Pixel> {
        let start_pos = range.start.pos(self);
        let end_pos = range.end.pos(self);
        let start_i = range.start.index(self);
        let end_i = range.end.index(self);
        Image {
            width: end_pos[0] - start_pos[0],
            height: end_pos[0] - start_pos[1],
            stride: self.stride,
            source: &self.source.as_ref().index(start_i..end_i),
            _p: PhantomData,
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

    pub fn iter<'a>(&'a self) -> std::iter::Map<Range<usize>, impl Fn(usize) -> &'a [Pixel]> {
        let map = |i| &self.source.as_ref()[(i * self.stride)..((i * self.stride) + self.width)];
        (0..self.height).map(map)
    }
}

impl<Source, Pixel> Image<Source, Pixel>
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
impl<Source, Pixel> Image<Source, Pixel>
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
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let image = self.image.as_ref();
        let new_index = match pos {
            io::SeekFrom::Start(i) => i,
            io::SeekFrom::End(offset) => u64::try_from(image.width() * image.height())
                .map_err(|err| io::Error::other(err))?
                .checked_add_signed(offset)
                .unwrap(),
            io::SeekFrom::Current(offset) => u64::try_from(self.index)
                .map_err(|error| io::Error::other(error))?
                .checked_add_signed(offset)
                .unwrap(),
        };

        self.index = usize::try_from(new_index).map_err(|error| io::Error::other(error))?;
        Ok(new_index)
    }
}

impl<Source, Pixel, I: AsRef<Image<Source, Pixel>> + AsMut<Image<Source, Pixel>>> Write
    for ImageCursor<Source, Pixel, I>
where
    Source: AsRef<[Pixel]> + AsMut<[Pixel]>,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
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
                cmp::Ordering::Less => {
                    self.index += buf.len();
                    rest_of_row[..buf.len()].copy_from_slice(buf);
                    Ok(buf.len())
                }
                cmp::Ordering::Equal | cmp::Ordering::Greater => {
                    rest_of_row.copy_from_slice(&buf[..rest_of_row.len()]);
                    self.index = (current_row_index + image.stride) * size_of::<Pixel>();
                    Ok(rest_of_row.len())
                }
            }
        } else {
            Ok(0)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
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

impl<Source, Pixel, I: AsRef<Image<Source, Pixel>>> Read for ImageCursor<Source, Pixel, I>
where
    Source: AsRef<[Pixel]>,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
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
                cmp::Ordering::Less => {
                    self.index += buf.len();
                    buf.copy_from_slice(&rest_of_row[..buf.len()]);
                    Ok(buf.len())
                }
                cmp::Ordering::Equal | cmp::Ordering::Greater => {
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
