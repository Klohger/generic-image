use crate::{
    error::{IndexOutOfRange, PositionOutOfRange},
    Image,
};
#[const_trait]
pub trait ImageIndex: Copy {
    fn index<Pixel>(
        self,
        image: &Image<impl AsRef<[Pixel]>, Pixel>,
    ) -> Result<usize, PositionOutOfRange>;
    fn pos<Pixel>(
        &self,
        image: &Image<impl AsRef<[Pixel]>, Pixel>,
    ) -> Result<[usize; 2], IndexOutOfRange>;
}

impl const ImageIndex for usize {
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

impl const ImageIndex for (usize, usize) {
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

impl const ImageIndex for [usize; 2] {
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
