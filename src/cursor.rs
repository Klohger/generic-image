use crate::Image;
use core::{cmp, marker::PhantomData, mem::size_of};
use std::io::{self, Read, Seek, Write};

pub struct ImageCursor<Source, Pixel, I: AsRef<Image<Source, Pixel>>>
where
    Source: AsRef<[Pixel]>,
{
    image: I,
    index: usize,
    _p: PhantomData<(Source, Pixel)>,
}

impl<Source, Pixel, I: AsRef<Image<Source, Pixel>>> ImageCursor<Source, Pixel, I>
where
    Source: AsRef<[Pixel]>,
{
    pub fn read_into_box(&mut self) -> io::Result<Box<[u8]>> {
        let image = self.image.as_ref();
        let mut buf = unsafe {
            Box::new_uninit_slice(image.width() * image.height() * size_of::<Pixel>()).assume_init()
        };
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
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
        if self.index < image.stride() * image.height() * size_of::<Pixel>() {
            let current_row_index =
                (self.index / (image.stride() * size_of::<Pixel>())) * image.stride();
            let last_element_of_row = (current_row_index + image.width()) * size_of::<Pixel>();
            let rest_of_row = unsafe {
                std::slice::from_raw_parts_mut(
                    image
                        .source_mut()
                        .as_mut()
                        .as_mut_ptr()
                        .cast::<u8>()
                        .add(self.index),
                    last_element_of_row - self.index,
                )
            };

            match buf.len().cmp(&rest_of_row.len()) {
                cmp::Ordering::Less => {
                    self.index += buf.len();
                    rest_of_row[..buf.len()].copy_from_slice(buf);
                    Ok(buf.len())
                }
                cmp::Ordering::Equal | cmp::Ordering::Greater => {
                    rest_of_row.copy_from_slice(&buf[..rest_of_row.len()]);
                    self.index += image.stride() * size_of::<Pixel>();
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
    pub const unsafe fn new(image: I) -> Self {
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
        if self.index < image.stride() * image.height() * size_of::<Pixel>() {
            let current_row_index =
                (self.index / (image.stride() * size_of::<Pixel>())) * image.stride();
            let last_element_of_row = (current_row_index + image.width()) * size_of::<Pixel>();
            let rest_of_row = unsafe {
                std::slice::from_raw_parts(
                    image
                        .source()
                        .as_ref()
                        .as_ptr()
                        .cast::<u8>()
                        .add(self.index),
                    last_element_of_row - self.index,
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
                    self.index += image.stride() * size_of::<Pixel>();
                    Ok(rest_of_row.len())
                }
            }
        } else {
            Ok(0)
        }
    }
}
