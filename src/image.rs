use crate::{
    error::{IndexOutOfRange, IndexOutOfRangeReason, PositionOutOfRange, SourceTooSmall},
    iterator::{Iter, IterMut, IterRows, IterRowsMut},
    ImageCursor, ImageIndex,
};
use core::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Index, IndexMut, Range},
};
use std::{rc::Rc, sync::Arc};

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
        self.iter_rows().eq(other.iter_rows())
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
impl<Pixel> Image<Box<[Pixel]>, Pixel>
where
    Pixel: Clone,
{
    pub fn filled(width: usize, height: usize, pixel: Pixel) -> Self {
        let mut image = unsafe { Image::uninit(width, height) };
        unsafe { image.fill_source(pixel) }
        image
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
impl<Pixel> Image<Vec<Pixel>, Pixel>
where
    Pixel: Clone,
{
    pub fn filled_vec(width: usize, height: usize, pixel: Pixel) -> Self {
        let mut image = unsafe { Image::uninit_vec(width, height) };
        unsafe { image.fill_source(pixel) };
        image
    }
}
impl<Source, Pixel, I: ImageIndex> Index<I> for Image<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    type Output = Pixel;

    fn index(&self, index: I) -> &Pixel {
        self.source.as_ref().index(index.index(self).unwrap())
    }
}

impl<Source, Pixel, I: ImageIndex> IndexMut<I> for Image<Source, Pixel>
where
    Source: AsMut<[Pixel]> + AsRef<[Pixel]>,
{
    fn index_mut(&mut self, index: I) -> &mut Pixel {
        let index = index.index(self).unwrap();

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
                std::ptr::copy_nonoverlapping(
                    &self.source.as_ref()[old_i] as *const _,
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
    pub const fn pos_to_index(&self, x: usize, y: usize) -> Result<usize, PositionOutOfRange> {
        Err(PositionOutOfRange {
            pos: [x, y],
            which_axes: {
                let x_out_of_range = x >= self.width;
                let y_out_of_range = y >= self.height;
                if x_out_of_range && y_out_of_range {
                    crate::error::WhichAxes::Both
                } else if x_out_of_range {
                    crate::error::WhichAxes::X
                } else if y_out_of_range {
                    crate::error::WhichAxes::Y
                } else {
                    return Ok(x + y * self.stride);
                }
            },
        })
    }
    pub const fn max_index(&self) -> usize {
        ((self.height - 1) * self.stride) + (self.width - 1)
    }
    pub const fn index_to_pos(&self, i: usize) -> Result<[usize; 2], IndexOutOfRange> {
        Err(IndexOutOfRange {
            value: i,
            reason: {
                if i % self.stride >= self.width {
                    IndexOutOfRangeReason::OutsideStride
                } else if i >= self.max_index() {
                    IndexOutOfRangeReason::PastEnd
                } else {
                    return Ok([i % self.width, i / self.width]);
                }
            },
        })
    }
    pub const fn next_index(&self, idx: usize) -> Option<usize> {
        if idx == self.max_index() {
            None
        } else {
            Some(self.normalize_index(idx + 1))
        }
    }
    pub const fn normalize_index(&self, mut idx: usize) -> usize {
        if idx >= self.max_index() {
            idx = self.max_index();
        }
        let inc = {
            let inc = (idx) % self.stride;
            if inc >= self.width {
                0
            } else {
                inc
            }
        };
        inc + ((idx + 1 + self.width) / self.stride)
    }

    pub const fn from_source(
        width: usize,
        height: usize,
        source: Source,
    ) -> Result<Self, crate::error::SourceTooSmall<Source, Pixel>> {
        if source.as_ref().len() < width * height {
            Err(SourceTooSmall::new(source, width, height, width))
        } else {
            Ok(Self {
                width,
                height,
                stride: width,
                source,
                _p: PhantomData,
            })
        }
    }
    pub const fn from_source_with_stride(
        width: usize,
        height: usize,
        stride: usize,
        source: Source,
    ) -> Result<Self, crate::error::SourceTooSmall<Source, Pixel>> {
        if source.as_ref().len() < stride * height {
            Err(SourceTooSmall::new(source, width, height, stride))
        } else {
            Ok(Self {
                width,
                height,
                stride,
                source,
                _p: PhantomData,
            })
        }
    }
    pub const fn region<I: ~const ImageIndex>(
        &self,
        range: Range<I>,
    ) -> Result<Image<&[Pixel], Pixel>, crate::Error<Source, Pixel>> {
        let Range { start, end } = range;

        let start_pos = match start.pos(self) {
            Ok(pos) => pos,
            Err(err) => return Err(crate::Error::IndexOutOfRange(err)),
        };
        let end_pos = match end.pos(self) {
            Ok(pos) => pos,
            Err(err) => return Err(crate::Error::IndexOutOfRange(err)),
        };
        let start_i = match start.index(self) {
            Ok(index) => index,
            Err(err) => return Err(crate::Error::PositionOutOfRange(err)),
        };
        let end_i = match end.index(self) {
            Ok(index) => index,
            Err(err) => return Err(crate::Error::PositionOutOfRange(err)),
        };
        Ok(Image {
            width: end_pos[0] - start_pos[0],
            height: end_pos[1] - start_pos[1],
            stride: self.stride,
            source: &self.source.as_ref().index(start_i..end_i),
            _p: PhantomData,
        })
    }
    pub const unsafe fn cursor(&self) -> ImageCursor<Source, Pixel, &Self> {
        ImageCursor::new(self)
    }
    pub const fn into_cursor(self) -> ImageCursor<Source, Pixel, Self> {
        unsafe { ImageCursor::new(self) }
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
    pub const unsafe fn source(&self) -> &Source {
        &self.source
    }
    pub unsafe fn into_source(self) -> Source {
        self.source
    }
    pub const fn iter(&self) -> Iter<Source, Pixel> {
        Iter::new(self)
    }
    pub fn iter_rows(&self) -> IterRows<Source, Pixel> {
        IterRows::new(self)
    }
}

impl<Source, Pixel> Image<Source, Pixel>
where
    Source: AsMut<[Pixel]> + AsRef<[Pixel]>,
{
    pub unsafe fn source_mut(&mut self) -> &mut Source {
        &mut self.source
    }
    pub fn region_mut(
        &mut self,
        range: Range<impl ImageIndex>,
    ) -> Result<Image<&mut [Pixel], Pixel>, crate::Error<Source, Pixel>> {
        let start_pos = range.start.pos(self)?;
        let end_pos = range.end.pos(self)?;
        let start_i = range.start.index(self)?;
        let end_i = range.end.index(self)?;
        Ok(Image {
            width: end_pos[0] - start_pos[0],
            height: end_pos[1] - start_pos[1],
            stride: self.stride,
            source: self.source.as_mut().index_mut(start_i..end_i),
            _p: PhantomData,
        })
    }
    pub unsafe fn cursor_mut(&mut self) -> ImageCursor<Source, Pixel, &mut Self> {
        ImageCursor::new(self)
    }
    pub fn iter_mut(&mut self) -> IterMut<Source, Pixel> {
        IterMut::new(self)
    }
    pub fn iter_rows_mut(&mut self) -> IterRowsMut<Source, Pixel> {
        IterRowsMut::new(self)
    }
}

impl<Source, Pixel> Image<Source, Pixel>
where
    Pixel: Clone,
    Source: AsMut<[Pixel]> + AsRef<[Pixel]>,
{
    pub unsafe fn fill_source(&mut self, pixel: Pixel) {
        self.source_mut().as_mut().fill(pixel)
    }
    pub fn fill(&mut self, pixel: Pixel) {
        if self.width == self.stride {
            unsafe { self.fill_source(pixel) }
        } else {
            for row in self.iter_rows_mut() {
                row.fill(pixel.clone())
            }
        }
    }
}
