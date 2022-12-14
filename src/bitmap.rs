//! Bitmap storage and parsing.

use std::{ops::Index, str::FromStr};

use anyhow::{anyhow, bail};

pub struct Bitmap<T> {
    pub elements: Vec<T>,
    pub width: u32,
    pub height: u32,
    pub out_of_bounds: T,
}

impl<T> Bitmap<T> {
    pub fn flatten_index(&self, (x, y): (i32, i32)) -> usize {
        (x + y * self.width as i32) as usize
    }

    pub fn is_in_bounds(&self, (x, y): (i32, i32)) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }
}

impl<T> Index<(i32, i32)> for Bitmap<T> {
    type Output = T;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        if self.is_in_bounds(index) {
            &self.elements[self.flatten_index(index)]
        } else {
            &self.out_of_bounds
        }
    }
}

impl<T> FromStr for Bitmap<T>
where
    T: BitmapElement + Default,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width: Option<u32> = None;
        let mut height = 0;
        let mut elements = vec![];
        for line in s.lines() {
            if let Some(width) = width {
                if line.len() as u32 != width {
                    bail!(
                        "all lines must be the same width (first line's width was {width}): {line}"
                    );
                }
            }
            for c in line.chars() {
                elements.push(
                    T::from_char(c)
                        .ok_or_else(|| anyhow!("{c:?} is not a valid bitmap element"))?,
                );
            }
            width = Some(line.len() as u32);
            height += 1;
        }
        Ok(Bitmap {
            width: width.unwrap_or(0),
            height,
            elements,
            out_of_bounds: Default::default(),
        })
    }
}

pub trait BitmapElement: Sized {
    fn from_char(c: char) -> Option<Self>;
}
