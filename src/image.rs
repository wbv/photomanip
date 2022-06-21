use std::io;
use std::io::Read;
use std::fs::File;

#[cfg(test)]
mod tests;

pub trait ImageManip {
    fn brighten(&self, amount: i32) -> ColorImage;
    fn contrast(&self) -> ColorImage;
    fn grayscale(&self) -> ColorImage;
    fn negate(&self) -> ColorImage;
    fn sharpen(&self) -> ColorImage;
    fn smooth(&self) -> ColorImage;
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorImage {
    width: usize,
    height: usize,
    maxval: usize,
    rpixels: Vec<u16>,
    gpixels: Vec<u16>,
    bpixels: Vec<u16>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GrayImage {
    width: usize,
    height: usize,
    maxval: usize,
    pixels: Vec<u16>,
}

#[derive(Debug)]
pub enum Image {
    Grayscale(Box<GrayImage>),
    Color(Box<ColorImage>),
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

        Ok(Image::Grayscale(Box::new(
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

        Ok(Image::Color(Box::new(
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


impl ImageManip for ColorImage {
    fn brighten(&self, amount: i32) -> ColorImage {
        unimplemented!()
    }
    fn contrast(&self) -> ColorImage {
        unimplemented!()
    }
    fn grayscale(&self) -> ColorImage {
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
    fn brighten(&self, amount: i32) -> ColorImage {
        unimplemented!()
    }
    fn contrast(&self) -> ColorImage {
        unimplemented!()
    }
    fn grayscale(&self) -> ColorImage {
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



