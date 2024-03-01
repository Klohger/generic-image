use std::ops::{Index, IndexMut};

use crate::Image;

pub struct Iter<'a, Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    image: &'a Image<Source, Pixel>,
    idx: Option<usize>,
}

impl<'a, Source, Pixel> Iter<'a, Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    pub const fn new(image: &'a Image<Source, Pixel>) -> Self {
        Self {
            image,
            idx: Some(0),
        }
    }
}

impl<'a, Source, Pixel> Iterator for Iter<'a, Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    type Item = ([usize; 2], &'a Pixel);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(idx) = self.idx {
            let ret = (self.image.index_to_pos(idx).unwrap(), &self.image[idx]);
            self.idx = self.image.next_index(idx);
            Some(ret)
        } else {
            None
        }
    }
}

pub struct IterRows<'a, Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    image: &'a Image<Source, Pixel>,
    row: usize,
}

impl<'a, Source, Pixel> IterRows<'a, Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    pub const fn new(image: &'a Image<Source, Pixel>) -> Self {
        Self { image, row: 0 }
    }
}

impl<'a, Source, Pixel> Iterator for IterRows<'a, Source, Pixel>
where
    Source: AsRef<[Pixel]>,
{
    type Item = &'a [Pixel];

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.image.height() {
            None
        } else {
            let ret = unsafe {
                self.image.source().as_ref().index({
                    let start = self.row * self.image.stride();
                    let end = start + self.image.width();
                    start..end
                })
            };
            self.row += 1;
            Some(ret)
        }
    }
}

pub struct IterMut<'a, Source, Pixel>
where
    Source: AsRef<[Pixel]> + AsMut<[Pixel]>,
{
    image: &'a mut Image<Source, Pixel>,
    idx: Option<usize>,
}

impl<'a, Source, Pixel> IterMut<'a, Source, Pixel>
where
    Source: AsRef<[Pixel]> + AsMut<[Pixel]>,
{
    pub fn new(image: &'a mut Image<Source, Pixel>) -> Self {
        Self {
            image,
            idx: Some(0),
        }
    }
}

impl<'a, Source, Pixel> Iterator for IterMut<'a, Source, Pixel>
where
    Source: AsRef<[Pixel]> + AsMut<[Pixel]>,
{
    type Item = ([usize; 2], &'a mut Pixel);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(idx) = self.idx {
            let ret = (self.image.index_to_pos(idx).unwrap(), unsafe {
                &mut *((&mut self.image[idx]) as *mut _)
            });
            self.idx = self.image.next_index(idx);
            Some(ret)
        } else {
            None
        }
    }
}

pub struct IterRowsMut<'a, Source, Pixel>
where
    Source: AsRef<[Pixel]> + AsMut<[Pixel]>,
{
    image: &'a mut Image<Source, Pixel>,
    row: usize,
}

impl<'a, Source, Pixel> IterRowsMut<'a, Source, Pixel>
where
    Source: AsRef<[Pixel]> + AsMut<[Pixel]>,
{
    pub fn new(image: &'a mut Image<Source, Pixel>) -> Self {
        Self { image, row: 0 }
    }
}

impl<'a, Source, Pixel> Iterator for IterRowsMut<'a, Source, Pixel>
where
    Source: AsRef<[Pixel]> + AsMut<[Pixel]>,
{
    type Item = &'a mut [Pixel];

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.image.height() {
            None
        } else {
            let ret = {
                let start = self.row * self.image.stride();
                let end = start + self.image.width();
                unsafe { &mut *(self.image.source_mut().as_mut().index_mut(start..end) as *mut _) }
            };
            self.row += 1;
            Some(ret)
        }
    }
}
