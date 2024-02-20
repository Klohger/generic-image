use core::{
    fmt::{Debug, Display, Pointer},
    marker::PhantomData,
};

#[derive(Debug)]
pub struct IndexOutOfRange {
    pub value: usize,
    pub reason: IndexOutOfRangeReason,
}
#[derive(Debug)]
pub enum IndexOutOfRangeReason {
    PastEnd,
    OutsideStride,
}

#[derive(Debug)]
pub struct PositionOutOfRange {
    pub pos: [usize; 2],
    pub which_axes: WhichAxes,
}
#[derive(Debug)]
pub struct SourceTooSmall<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    pub source: Source,
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    _p: PhantomData<Pixel>,
}

impl<Source, Pixel> SourceTooSmall<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    pub fn new(source: Source, width: usize, height: usize, stride: usize) -> Self {
        Self {
            source,
            width,
            height,
            stride,
            _p: PhantomData,
        }
    }
}
#[derive(Debug)]
pub enum WhichAxes {
    X,
    Y,
    Both,
}
#[derive(Debug)]
pub enum Error<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    IndexOutOfRange(IndexOutOfRange),
    PositionOutOfRange(PositionOutOfRange),
    SourceTooSmall(SourceTooSmall<Source, Pixel>),
}

impl<Source, Pixel> From<IndexOutOfRange> for Error<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    fn from(value: IndexOutOfRange) -> Self {
        Self::IndexOutOfRange(value)
    }
}

impl<Source, Pixel> From<PositionOutOfRange> for Error<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    fn from(value: PositionOutOfRange) -> Self {
        Self::PositionOutOfRange(value)
    }
}

impl<Source, Pixel> From<SourceTooSmall<Source, Pixel>> for Error<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    fn from(value: SourceTooSmall<Source, Pixel>) -> Self {
        Self::SourceTooSmall(value)
    }
}

impl<Source, Pixel> Display for Error<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SourceTooSmall(source_too_small) => source_too_small.fmt(f),
            Error::PositionOutOfRange(position_out_of_range) => position_out_of_range.fmt(f),
            Error::IndexOutOfRange(index_out_of_range) => index_out_of_range.fmt(f),
        }
    }
}

impl<Source, Pixel> std::error::Error for Error<Source, Pixel>
where
    Source: AsRef<[Pixel]> + Debug,
    Pixel: Debug,
{
}
