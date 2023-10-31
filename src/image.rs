use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io;
use std::io::Read;

#[cfg(test)]
mod tests;

/// The numeric type that pixel values are stored as, internally.
///
/// Note that raw images will still be read and written with the appropriate-size numeric type
/// according to the PPM/PGM specification.
type PxVal = u16;

#[cfg_attr(test, derive(Debug, Clone, PartialEq))]
struct ColorImage {
    width: usize,
    height: usize,
    maxval: usize,
    rpixels: Vec<PxVal>,
    gpixels: Vec<PxVal>,
    bpixels: Vec<PxVal>,
}

#[cfg_attr(test, derive(Debug, Clone, PartialEq))]
struct GrayImage {
    width: usize,
    height: usize,
    maxval: usize,
    pixels: Vec<PxVal>,
}

#[cfg_attr(test, derive(Debug, Clone, PartialEq))]
enum ImageType {
    Grayscale(GrayImage),
    Color(ColorImage),
}

enum ColorType {
    Grayscale,
    Color
}

enum RasterType {
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

impl<'a> ImageHeader<'a> {

    /// Attempts to parse a PGM/PPM image's metadata from its full contents.
    ///
    /// This function implements a state machine to read each byte at a time, parsing values as
    /// they apear and failing needed
    ///
    /// # Parsing rules
    ///
    /// After the first two characters "Px", we should see:
    /// 1. Whitespace
    /// 2. Width (ASCII Decimal)
    /// 3. Whitespace
    /// 4. Height (ASCII Decimal)
    /// 5. Whitespace
    /// 6. Maxval (ASCII Decimal, must be < 65536)
    /// 7. A single whitespace
    /// 8. (raster: the actual image pixel contents)
    ///
    /// Any line (something followed by '\n' or '\r') that begins with a '#' is a comment and
    /// gets ignored until the next newline.
    fn read(filedata: &'a [u8]) -> io::Result<ImageHeader> {

        // first determine magic sequence
        let (color_kind, raster_kind) = ImageHeader::get_kind(filedata)?;

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
                    ColorType::Color => true,
                    ColorType::Grayscale => false
                },
            is_ascii_raster:
                match raster_kind {
                    RasterType::Ascii => true,
                    RasterType::Raw => false
                },
            width: params[0],
            height: params[1],
            maxval: params[2],
            raster: &filedata[params[3]..]
        })
    }

    /// Gets the color and raster for an image.
    ///
    /// Returns a tuple indicating these based on the magic constant at the beginning of the file,
    /// or an error if the first two bytes of the file don't match any of the expected patterns.
    fn get_kind(filedata: &[u8]) -> io::Result<(ColorType, RasterType)> {
        match &filedata[0..2] {
            b"P2" => Ok((ColorType::Grayscale, RasterType::Ascii)),
            b"P3" => Ok((ColorType::Color,     RasterType::Ascii)),
            b"P5" => Ok((ColorType::Grayscale, RasterType::Raw)),
            b"P6" => Ok((ColorType::Color,     RasterType::Raw)),
            [one, two] => {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Not a PPM/PGM file (non-magic sequence: {:?})", [one, two])
                ))
            },
            _ => {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Not a PPM/PGM file (image too small: {} bytes)", filedata.len())
                ))
            }
        }
    }
}


#[cfg_attr(test, derive(Debug, Clone, PartialEq))]
pub struct Image(ImageType);

impl Image {

    /// Tries to construct an [`ImageType`] from the file located at `path`.
    pub fn load(path: &str) -> io::Result<Self> {
        let mut data = Vec::<u8>::new();
        let mut file = File::open(path)?;
        let _ = file.read_to_end(&mut data)?;

        // interpret the kind of PPM from the magic sequence
        let hdr = ImageHeader::read(&data)?;

        if hdr.is_color {
            let img: ColorImage = hdr.try_into()?;
            Ok(Image(ImageType::Color(img)))
        } else {
            let img: GrayImage = hdr.try_into()?;
            Ok(Image(ImageType::Grayscale(img)))
        }
    }
}



///////////////////////////////
// Raster Extraction Methods //
///////////////////////////////

impl TryFrom<ImageHeader<'_>> for ColorImage {
    type Error = io::Error;
    fn try_from(hdr: ImageHeader) -> Result<Self, Self::Error> {
        // each channel holds `size` pixels, so just preallocate enough heap
        let size = hdr.width * hdr.height;
        let mut rs: Vec<PxVal> = Vec::with_capacity(size);
        let mut gs: Vec<PxVal> = Vec::with_capacity(size);
        let mut bs: Vec<PxVal> = Vec::with_capacity(size);

        if hdr.is_ascii_raster {
            let raster = match String::from_utf8(hdr.raster.into()) {
                Ok(inner) => inner,
                Err(_) => return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Plain raster contains invalid ASCII".to_string()
                )),
            };

            let mut all_values = Vec::<PxVal>::with_capacity(size*3);
            for val in raster.split_whitespace() {
                match val.parse::<PxVal>() {
                    Ok(v) => all_values.push(v),
                    Err(_) => return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Plain raster contains non-numeric value '{}'.", val)
                    )),
                }
            }

            // for ascii files, make sure raster size is consistent w/ header
            let expected_len = hdr.width * hdr.height * 3;
            if all_values.len() != expected_len {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Inconsistent plain raster length '{}' (expected '{}')",
                        all_values.len(),
                        expected_len
                    )
                ));
            } else {
                for vals in all_values.chunks_exact(3) {
                    rs.push(vals[0]);
                    gs.push(vals[1]);
                    bs.push(vals[2]);
                }
            }
        } else {
            // sz is the size of the underlying datatype
            let sz = if hdr.maxval < 256 { 2 } else { 1 };

            // for raw files, we can quickly make sure raster size is consistent w/ header
            let expected_len = hdr.width * hdr.height * 3 * sz;
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

            // then iterate through the raster in chunks of three values to populate each channel.
            if hdr.maxval < 256 {
                // underlying raw pixel value is a u8
                for pixel in hdr.raster.chunks_exact(3) {
                    rs.push(pixel[0] as PxVal);
                    gs.push(pixel[1] as PxVal);
                    bs.push(pixel[2] as PxVal);
                }
            } else {
                // underlying raw pixel value is a u16 (big-endian)
                for pixel in hdr.raster.chunks(3*2) {
                    rs.push(((pixel[0] as PxVal) << 8) | (pixel[1] as PxVal));
                    gs.push(((pixel[2] as PxVal) << 8) | (pixel[3] as PxVal));
                    bs.push(((pixel[4] as PxVal) << 8) | (pixel[5] as PxVal));
                }
            }
        }

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

impl TryFrom<ImageHeader<'_>> for GrayImage {
    type Error = io::Error;
    fn try_from(hdr: ImageHeader) -> Result<Self, Self::Error> {
        let size = hdr.width * hdr.height;
        if hdr.is_ascii_raster {
            let raster = match String::from_utf8(hdr.raster.into()) {
                Ok(inner) => inner,
                Err(_) => return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Plain raster contains invalid ASCII".to_string()
                )),
            };

            let mut pixels = Vec::<PxVal>::with_capacity(size);
            for val in raster.split_whitespace() {
                match val.parse::<PxVal>() {
                    Ok(v) => pixels.push(v),
                    Err(_) => return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Plain raster contains non-numeric value '{}'.", val)
                    )),
                }
            }

            // for ascii files, make sure raster size is consistent w/ header
            let expected_len = hdr.width * hdr.height;
            if pixels.len() != expected_len {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Inconsistent plain raster length '{}' (expected '{}')",
                        pixels.len(),
                        expected_len
                    )
                ));
            }

            Ok(Self {
                width: hdr.width,
                height: hdr.height,
                maxval: hdr.maxval,
                pixels,
            })
        } else {
            // sz is the size of the underlying datatype
            let sz = if hdr.maxval < 256 { 2 } else { 1 };

            // for raw files, we can quickly make sure raster size is consistent w/ header
            let expected_len = hdr.width * hdr.height * sz;
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

            let pixels = if hdr.maxval < 256 {
                // underlying raw pixel value is a u8
                hdr.raster.iter()
                    .map(|px| *px as PxVal)
                    .collect()
            } else {
                // underlying raw pixel value is a u16 (big-endian)
                hdr.raster.chunks(2)
                    .map(|px|((px[0] as PxVal) << 8) | (px[1] as PxVal))
                    .collect()
            };

            Ok(Self {
                width: hdr.width,
                height: hdr.height,
                maxval: hdr.maxval,
                pixels,
            })
        }
    }
}


////////////////////////////////
// Image Manipulation Methods //
////////////////////////////////

pub trait ImageManip {
    fn brighten(&self, amount: i32) -> ColorImage;
    fn contrast(&self) -> GrayImage;
    fn grayscale(&self) -> GrayImage;
    fn negate(&self) -> ColorImage;
    fn sharpen(&self) -> ColorImage;
    fn smooth(&self) -> ColorImage;
}

impl ImageManip for ColorImage {
    fn brighten(&self, _amount: i32) -> ColorImage {
        unimplemented!()
    }
    fn contrast(&self) -> GrayImage {
        unimplemented!()
    }
    fn grayscale(&self) -> GrayImage {
        unimplemented!()
    }
    fn negate(&self) -> ColorImage {
        unimplemented!()
    }
    fn sharpen(&self) -> ColorImage {
        unimplemented!()
    }
    fn smooth(&self) -> ColorImage {
        unimplemented!()
    }
}

impl ImageManip for GrayImage {
    fn brighten(&self, _amount: i32) -> ColorImage {
        unimplemented!()
    }
    fn contrast(&self) -> GrayImage {
        unimplemented!()
    }
    fn grayscale(&self) -> GrayImage {
        unimplemented!()
    }
    fn negate(&self) -> ColorImage {
        unimplemented!()
    }
    fn sharpen(&self) -> ColorImage {
        unimplemented!()
    }
    fn smooth(&self) -> ColorImage {
        unimplemented!()
    }
}
