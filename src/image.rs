use std::io;

use std::io::Read;
use std::fs::File;

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


// Parsing rules:
//
// after the first two characters "Px", there's
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
pub fn load_from_file(path: &str) -> io::Result<Image> {
    let mut data = Vec::<u8>::new();
    let mut file = File::open(path)?;
    let _ = file.read_to_end(&mut data)?;

    // interpret the kind of PPM from the magic sequence
    let (grayscale, ascii) =
        match std::str::from_utf8(&data[0..2]) {
            Ok("P6") => { (false, false) },
            Ok("P5") => { (true,  false) },
            Ok("P3") => { (false, true)  },
            Ok("P2") => { (true,  true)  },
            _ => {
                eprintln!("Not a valid PPM/PGM file: '{}'", path);
                return Err(
                    io::Error::new(
                        io::ErrorKind::InvalidData, "Not a PPM/PGM file"
                    )
                )
            }
        };

    // Finite State Machine for parsing
    enum State {
        Newline,
        Whitespace,
        Comment,
        Value,
    }

    // iterator over the data
    let mut scanner = data.iter().enumerate().skip(2);

    // stores values returned
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
                            let value_string = std::str::from_utf8(&data[param_start..i]);
                            match value_string {
                                Ok(value) => {
                                    match value.parse::<usize>() {
                                        Ok(v) => {
                                            // save the succesffully interpreted value
                                            params.push(v);

                                            // also save our index if we've found width, length,
                                            // maxval
                                            if params.len() == 3 {
                                                params.push(i);
                                            }
                                        }
                                        Err(_) => {
                                            eprintln!("Invalid image param");
                                            return Err(
                                                io::Error::new(
                                                    io::ErrorKind::InvalidData, "Bad image param"
                                                    )
                                                )
                                        }
                                    }
                                },
                                Err(_) => {
                                    eprintln!("Invalid image param (non-unicode data)");
                                    return Err(
                                        io::Error::new(
                                            io::ErrorKind::InvalidData, "Bad image param"
                                        )
                                    )
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
               eprintln!("Reached end of file before finding all image parameters (found {:?})", params);
               return Err(io::Error::new(
                   io::ErrorKind::InvalidData, "Not a valid PPM/PGM file"
               ))
            }
        }
    }

    let (width, height, maxval) = (params[0], params[1], params[2]);

    if grayscale {
        if ascii {
            println!("Ascii grayscale image");
        } else {
            println!("Raw grayscale image");
        }

        Ok(Image::Grayscale8(Box::new(
            GrayImage {
                width: width,
                height: height,
                maxval: maxval,
                pixels: vec![0],
            }
        )))
    }

    else {
        if ascii {
            println!("Ascii color image");
        } else {
            println!("Raw color image");
        }

        Ok(Image::Color8(Box::new(
            ColorImage {
                width: width,
                height: height,
                maxval: maxval,
                rpixels: vec![0],
                gpixels: vec![0],
                bpixels: vec![0]
            }
        )))
    }
}

///////////////////////////////
// Raster Extraction Methods //
///////////////////////////////

// TODO:
//   implement FromRaster stubs
//   (maybe) change FromRaster to TryInto<T>
trait FromRaster {
    /// Trait for an Image to form itself given its header parameters
    /// and a `&[u8]` containing its raster
    fn from_raster(w: usize, h: usize, m: usize, data: &[u8]) -> Self;
}

impl FromRaster for ColorImage<u8> {
    fn from_raster(w: usize, h: usize, m: usize, data: &[u8]) -> ColorImage<u8> {
        ColorImage {
            width: w,
            height: h,
            maxval: m,
            rpixels: vec![0],
            gpixels: vec![0],
            bpixels: vec![0]
        }
    }
}

impl FromRaster for ColorImage<u16> {
    fn from_raster(w: usize, h: usize, m: usize, data: &[u8]) -> ColorImage<u16> {
        ColorImage {
            width: w,
            height: h,
            maxval: m,
            rpixels: vec![0],
            gpixels: vec![0],
            bpixels: vec![0]
        }
    }
}

impl FromRaster for GrayImage<u8> {
    fn from_raster(w: usize, h: usize, m: usize, data: &[u8]) -> GrayImage<u8> {
        GrayImage {
            width: w,
            height: h,
            maxval: m,
            pixels: vec![0]
        }
    }
}

impl FromRaster for GrayImage<u16> {
    fn from_raster(w: usize, h: usize, m: usize, data: &[u8]) -> GrayImage<u16> {
        GrayImage {
            width: w,
            height: h,
            maxval: m,
            pixels: vec![0]
        }
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



