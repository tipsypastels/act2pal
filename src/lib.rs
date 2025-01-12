#![doc = include_str!("../README.md")]

use std::{error, fmt, ops::Deref};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b }
    }
}

impl PartialEq<(u8, u8, u8)> for Color {
    fn eq(&self, &rhs: &(u8, u8, u8)) -> bool {
        (self.r, self.g, self.b) == rhs
    }
}

#[derive(Debug, PartialEq)]
pub struct Palette {
    colors: Vec<Color>,
}

impl Palette {
    pub fn new(colors: Vec<Color>) -> Self {
        Self { colors }
    }

    pub fn from_act(bytes: &[u8]) -> Result<Self, ParseError> {
        const BYTES_PER_COLOR: usize = 3;
        const COUNT_OFFSET_FROM_END: usize = 3;

        let count = *bytes
            .get(bytes.len() - COUNT_OFFSET_FROM_END)
            .ok_or(ParseError)?;

        let colors = bytes
            .chunks_exact(BYTES_PER_COLOR)
            .take(count as _)
            .map(|chunk| {
                Ok(Color {
                    r: *chunk.first().ok_or(ParseError)?,
                    g: *chunk.get(1).ok_or(ParseError)?,
                    b: *chunk.get(2).ok_or(ParseError)?,
                })
            })
            .collect::<Result<Vec<_>, ParseError>>()?;

        if colors.len() as u8 != count {
            return Err(ParseError);
        }

        Ok(Self { colors })
    }
}

impl fmt::Display for Palette {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "JASC-PAL\n0100")?;
        writeln!(f, "{}", self.colors.len())?;

        for Color { r, g, b } in &self.colors {
            writeln!(f, "{r} {g} {b}")?;
        }

        Ok(())
    }
}

impl Deref for Palette {
    type Target = [Color];

    fn deref(&self) -> &Self::Target {
        &self.colors
    }
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unprocessable bytes")
    }
}

impl error::Error for ParseError {}
