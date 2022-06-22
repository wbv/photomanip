use std::io;

use std::io::Read;
use std::fs::File;

use std::convert::{TryFrom, TryInto};

#[cfg(test)]
mod tests;

#[cfg_attr(test, derive(Debug, Clone, PartialEq))]
pub struct ColorImage<Depth> {
    width: usize,
    height: usize,
    maxval: usize,
    rpixels: Vec<Depth>,
    gpixels: Vec<Depth>,
    bpixels: Vec<Depth>,
}

#[cfg_attr(test, derive(Debug, Clone, PartialEq))]
pub struct GrayImage<Depth> {
    width: usize,
    height: usize,
    maxval: usize,
    pixels: Vec<Depth>,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Image {
    Grayscale8(Box<GrayImage<u8>>),
    Grayscale16(Box<GrayImage<u16>>),
    Color8(Box<ColorImage<u8>>),
    Color16(Box<ColorImage<u16>>),
}


enum ImageKind {
    Grayscale,
    Color
}

enum RasterKind {
    Ascii,
    Raw
}

struct ImageHeader<'a> {
    is_color: bool,
    is_ascii_raster: bool,
    width: usize,
    height: usize,
    maxval: usize,
    raster: &'a [u8]
}


/// Tries to construct an [`Image`] from the file located at `path`.
pub fn load_from_file(path: &str) -> io::Result<Image> {
    let mut data = Vec::<u8>::new();
    let mut file = File::open(path)?;
    let _ = file.read_to_end(&mut data)?;

    // interpret the kind of PPM from the magic sequence
    let hdr = get_image_header(&data)?;

    match (hdr.is_color, hdr.maxval > 255) {
        (true, true) => {
            let img: ColorImage<u16> = hdr.try_into()?;
            Ok(Image::Color16(Box::new(img)))
        }
        (true, false) => {
            let img: ColorImage<u8> = hdr.try_into()?;
            Ok(Image::Color8(Box::new(img)))
        }
        (false, true) => {
            let img: GrayImage<u16> = hdr.try_into()?;
            Ok(Image::Grayscale16(Box::new(img)))
        }
        (false, false) => {
            let img: GrayImage<u8> = hdr.try_into()?;
            Ok(Image::Grayscale8(Box::new(img)))
        }
    }
}

// Parsing rules:
//
// after the first two characters "P#", there's
// 1) Whitespace
// 2) Width (ASCII Decimal)
// 3) Whitespace
// 4) Height (ASCII Decimal)
// 5) Whitespace
// 6) Maxval (ASCII Decimal, must be < 65536)
// 7) A single whitespace
// 8) RASTER DATA
//
// but any line (something followed by '\n' or '\r') that begins with a '#' is a comment and gets
// ignored until the next newline.
fn get_image_header(filedata: &[u8]) -> io::Result<ImageHeader> {

    // first determine magic sequence
    let (color_kind, raster_kind) = get_kind(filedata)?;

    // Finite State Machine for parsing
    enum State {
        Newline,
        Whitespace,
        Comment,
        Value,
    }

    // iterator over the data
    let mut scanner = filedata.iter().enumerate().skip(2);

    // stores values extracted from the image header:
    // [width, height, maxval, raster_start]
    let mut params = Vec::<usize>::with_capacity(4);

    // index into data where we start interpreting a value as a string
    let mut param_start: usize = 2;

    // the FSM parser's state
    let mut state = State::Newline;

    while params.len() < 4 {
        match scanner.next() {
            Some((i, &ch)) => {
                match state {
                    State::Newline => {
                        if ch == b'#' {
                            state = State::Comment;
                        } else if ch == b'\n' || ch == b'\r' {
                            state = State::Newline;
                        } else if ch.is_ascii_whitespace() {
                            state = State::Whitespace;
                        } else {
                            // this character is the start of a numeric parameter
                            param_start = i;
                            state = State::Value;
                        }
                    }
                    State::Comment => {
                        if ch == b'\n' || ch == b'\r' {
                            state = State::Newline;
                        }
                    }
                    State::Whitespace => {
                        if ch == b'\n' || ch == b'\r' {
                            state = State::Newline;
                        }
                        else if !ch.is_ascii_whitespace() {
                            // this character is the start of a numeric parameter
                            param_start = i;
                            state = State::Value;
                        }
                    }
                    State::Value => {
                        if ch.is_ascii_whitespace() {
                            // this character is the non-inclusive end of a numeric parameter, so
                            // start parsing it
                            let value_string = std::str::from_utf8(&filedata[param_start..i]);
                            match value_string {
                                Ok(value) => {
                                    match value.parse::<usize>() {
                                        Ok(v) => {
                                            // save the succesffully interpreted value
                                            params.push(v);

                                            // if we've found width, length, maxval
                                            // then mark where the raster starts
                                            if params.len() == 3 {
                                                params.push(i + 1);
                                            }
                                        }
                                        Err(_) => {
                                            return Err(io::Error::new(
                                                io::ErrorKind::InvalidData,
                                                "Invalid image param"
                                            ));
                                        }
                                    }
                                },
                                Err(_) => {
                                    return Err(io::Error::new(
                                        io::ErrorKind::InvalidData,
                                        "Invalid image param (non-unicode data)"
                                    ));
                                },
                            }

                            // advance state from here based on kind of whitespace
                            if ch == b'\n' || ch == b'\r' {
                                state = State::Newline;
                            } else {
                                state = State::Whitespace;
                            }
                        }
                    }
                }
            }
            None => {
               return Err(io::Error::new(
                   io::ErrorKind::InvalidData,
                   format!(
                       "Reached end of file before finding all image parameters (found {:?})",
                       params
                   )
               ));
            }
        }
    }

    Ok(ImageHeader {
        is_color:
            match color_kind {
                ImageKind::Color => true,
                ImageKind::Grayscale => false
            },
        is_ascii_raster:
            match raster_kind {
                RasterKind::Ascii => true,
                RasterKind::Raw => false
            },
        width: params[0],
        height: params[1],
        maxval: params[2],
        raster: &filedata[params[3]..]
    })
}

fn get_kind(filedata: &[u8]) -> io::Result<(ImageKind, RasterKind)> {
    match &filedata[0..2] {
        b"P2" => Ok((ImageKind::Grayscale, RasterKind::Ascii)),
        b"P3" => Ok((ImageKind::Color,     RasterKind::Ascii)),
        b"P5" => Ok((ImageKind::Grayscale, RasterKind::Raw)),
        b"P6" => Ok((ImageKind::Color,     RasterKind::Raw)),
        [one, two] => {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Not a PPM/PGM file (found magic sequence {:?})",
                    [one, two]
                )
            ))
        },
        _ => {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Not a PPM/PGM file"))
        }
    }
}

///////////////////////////////
// Raster Extraction Methods //
///////////////////////////////

impl TryFrom<ImageHeader<'_>> for ColorImage<u8> {
    type Error = io::Error;
    fn try_from(hdr: ImageHeader) -> Result<Self, Self::Error> {

        if hdr.is_ascii_raster {
            unimplemented!();
        } else {
            // for raw files, make sure raster size is consistent w/ header
            let expected_len = hdr.width * hdr.height * 3;
            if hdr.raster.len() != expected_len {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Inconsistent raster size '{}' (expected '{}')",
                        hdr.raster.len(),
                        expected_len
                    )
                ));
            }

            // each channel holds `size` pixels, so just preallocate enough heap
            let size = hdr.width * hdr.height;
            let mut rs: Vec<u8> = Vec::with_capacity(size);
            let mut gs: Vec<u8> = Vec::with_capacity(size);
            let mut bs: Vec<u8> = Vec::with_capacity(size);

            // iterate through in chunks of three,
            // putting pixels in their correct channel
            hdr.raster.chunks(3).for_each(
                |x| match x {
                    [r,g,b] => {
                        rs.push(*r);
                        gs.push(*g);
                        bs.push(*b);
                    },
                    _ => {
                        // we verified expected_len so we shouldn't get here
                        panic!("Bad raster chunk");
                    }
                },
            );

            Ok(Self {
                width: hdr.width,
                height: hdr.height,
                maxval: hdr.maxval,
                rpixels: rs,
                gpixels: gs,
                bpixels: bs
            })
        }
    }
}

impl TryFrom<ImageHeader<'_>> for ColorImage<u16> {
    type Error = io::Error;
    fn try_from(hdr: ImageHeader) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

impl TryFrom<ImageHeader<'_>> for GrayImage<u8> {
    type Error = io::Error;
    fn try_from(hdr: ImageHeader) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

impl TryFrom<ImageHeader<'_>> for GrayImage<u16> {
    type Error = io::Error;
    fn try_from(hdr: ImageHeader) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}


////////////////////////////////
// Image Manipulation Methods //
////////////////////////////////

pub trait ImageManip {
    type Depth;
    fn brighten(&self, amount: i32) -> ColorImage<Self::Depth>;
    fn contrast(&self) -> ColorImage<Self::Depth>;
    fn grayscale(&self) -> ColorImage<Self::Depth>;
    fn negate(&self) -> ColorImage<Self::Depth>;
    fn sharpen(&self) -> ColorImage<Self::Depth>;
    fn smooth(&self) -> ColorImage<Self::Depth>;
}

impl ImageManip for ColorImage<u8> {
    type Depth = u8;
    fn brighten(&self, amount: i32) -> ColorImage<u8> {
        unimplemented!()
    }
    fn contrast(&self) -> ColorImage<u8> {
        unimplemented!()
    }
    fn grayscale(&self) -> ColorImage<u8> {
        unimplemented!()
    }
    fn negate(&self) -> ColorImage<u8> {
        unimplemented!()
    }
    fn sharpen(&self) -> ColorImage<u8> {
        unimplemented!()
    }
    fn smooth(&self) -> ColorImage<u8> {
        unimplemented!()
    }
}

impl ImageManip for ColorImage<u16> {
    type Depth = u16;
    fn brighten(&self, amount: i32) -> ColorImage<u16> {
        unimplemented!()
    }
    fn contrast(&self) -> ColorImage<u16> {
        unimplemented!()
    }
    fn grayscale(&self) -> ColorImage<u16> {
        unimplemented!()
    }
    fn negate(&self) -> ColorImage<u16> {
        unimplemented!()
    }
    fn sharpen(&self) -> ColorImage<u16> {
        unimplemented!()
    }
    fn smooth(&self) -> ColorImage<u16> {
        unimplemented!()
    }
}

impl ImageManip for GrayImage<u8> {
    type Depth = u8;
    fn brighten(&self, amount: i32) -> ColorImage<u8> {
        unimplemented!()
    }
    fn contrast(&self) -> ColorImage<u8> {
        unimplemented!()
    }
    fn grayscale(&self) -> ColorImage<u8> {
        unimplemented!()
    }
    fn negate(&self) -> ColorImage<u8> {
        unimplemented!()
    }
    fn sharpen(&self) -> ColorImage<u8> {
        unimplemented!()
    }
    fn smooth(&self) -> ColorImage<u8> {
        unimplemented!()
    }
}

impl ImageManip for GrayImage<u16> {
    type Depth = u16;
    fn brighten(&self, amount: i32) -> ColorImage<u16> {
        unimplemented!()
    }
    fn contrast(&self) -> ColorImage<u16> {
        unimplemented!()
    }
    fn grayscale(&self) -> ColorImage<u16> {
        unimplemented!()
    }
    fn negate(&self) -> ColorImage<u16> {
        unimplemented!()
    }
    fn sharpen(&self) -> ColorImage<u16> {
        unimplemented!()
    }
    fn smooth(&self) -> ColorImage<u16> {
        unimplemented!()
    }
}



