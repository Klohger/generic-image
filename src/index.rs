use crate::{
    error::{IndexOutOfRange, PositionOutOfRange},
    Image,
};

pub trait ImageIndex<Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    fn index(self, image: &Image<Source, Pixel>) -> Result<usize, PositionOutOfRange>;
    fn pos(&self, image: &Image<Source, Pixel>) -> Result<[usize; 2], IndexOutOfRange>;
}

impl<Source, Pixel> ImageIndex<Source, Pixel> for usize
where
    Source: AsRef<[Pixel]>,
{
    fn index(self, _: &Image<Source, Pixel>) -> Result<usize, PositionOutOfRange> {
        Ok(self)
    }

    fn pos(&self, image: &Image<Source, Pixel>) -> Result<[usize; 2], IndexOutOfRange> {
        image.index_to_pos(*self)
    }
}

impl<Source, Pixel> ImageIndex<Source, Pixel> for (usize, usize)
where
    Source: AsRef<[Pixel]>,
{
    fn index(self, image: &Image<Source, Pixel>) -> Result<usize, PositionOutOfRange> {
        image.pos_to_index(self.0, self.1)
    }

    fn pos(&self, _: &Image<Source, Pixel>) -> Result<[usize; 2], IndexOutOfRange> {
        Ok((*self).into())
    }
}

impl<Source, Pixel> ImageIndex<Source, Pixel> for [usize; 2]
where
    Source: AsRef<[Pixel]>,
{
    fn index(self, image: &Image<Source, Pixel>) -> Result<usize, PositionOutOfRange> {
        image.pos_to_index(self[0], self[1])
    }
    fn pos(&self, _: &Image<Source, Pixel>) -> Result<[usize; 2], IndexOutOfRange> {
        Ok(*self)
    }
}
