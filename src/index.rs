use crate::{
    error::{IndexOutOfRange, PositionOutOfRange},
    Image,
};

pub trait ImageIndex {
    fn index<Pixel>(
        self,
        image: &Image<impl AsRef<[Pixel]>, Pixel>,
    ) -> Result<usize, PositionOutOfRange>;
    fn pos<Pixel>(
        &self,
        image: &Image<impl AsRef<[Pixel]>, Pixel>,
    ) -> Result<[usize; 2], IndexOutOfRange>;
}

impl ImageIndex for usize {
    fn index<Pixel>(
        self,
        _: &Image<impl AsRef<[Pixel]>, Pixel>,
    ) -> Result<usize, PositionOutOfRange> {
        Ok(self)
    }

    fn pos<Pixel>(
        &self,
        image: &Image<impl AsRef<[Pixel]>, Pixel>,
    ) -> Result<[usize; 2], IndexOutOfRange> {
        image.index_to_pos(*self)
    }
}

impl ImageIndex for (usize, usize) {
    fn index<Pixel>(
        self,
        image: &Image<impl AsRef<[Pixel]>, Pixel>,
    ) -> Result<usize, PositionOutOfRange> {
        image.pos_to_index(self.0, self.1)
    }

    fn pos<Pixel>(
        &self,
        _: &Image<impl AsRef<[Pixel]>, Pixel>,
    ) -> Result<[usize; 2], IndexOutOfRange> {
        Ok((*self).into())
    }
}

impl ImageIndex for [usize; 2] {
    fn index<Pixel>(
        self,
        image: &Image<impl AsRef<[Pixel]>, Pixel>,
    ) -> Result<usize, PositionOutOfRange> {
        image.pos_to_index(self[0], self[1])
    }
    fn pos<Pixel>(
        &self,
        _: &Image<impl AsRef<[Pixel]>, Pixel>,
    ) -> Result<[usize; 2], IndexOutOfRange> {
        Ok(*self)
    }
}
