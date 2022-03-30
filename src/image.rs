use std::io;
use std::io::Read;
use std::fs::File;

pub trait ImageManip {
    fn brighten(&self, amount: i32) -> ColorImage;
    fn contrast(&self) -> ColorImage;
    fn grayscale(&self) -> ColorImage;
    fn negate(&self) -> ColorImage;
    fn sharpen(&self) -> ColorImage;
    fn smooth(&self) -> ColorImage;
}

#[derive(Debug, Clone)]
pub struct ColorImage {
    width: usize,
    height: usize,
    maxval: usize,
    rpixels: Vec<u16>,
    gpixels: Vec<u16>,
    bpixels: Vec<u16>,
}

#[derive(Debug, Clone)]
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
// but any line (something followed by '\n') that begins with a '#' is a comment and gets
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

    // tracks where to start reading a value as a string
    let mut param_start: usize = 2;

    // trakcs the parser state
    let mut state = State::Newline;

    while params.len() < 4 {
        match scanner.next() {
            Some((i, &ch)) => {
                match state {
                    State::Newline => {
                        if ch == b'#' {
                            state = State::Comment;
                        } else if ch == b'\n' {
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
                        if ch == b'\n' {
                            state = State::Newline;
                        }
                    }
                    State::Whitespace => {
                        if ch == b'\n' {
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
                            if ch == b'\n' {
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



///////////////////////////////////////////////////////////////////////////////
// unit tests /////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    fn img_folder() -> String {
        env!("CARGO_MANIFEST_DIR").to_owned() + "/img/"
    }

    fn make_gray_image() -> Box<GrayImage> {
        Box::new(
            GrayImage {
                width: 3,
                height: 4,
                maxval: 255,
                pixels: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            }
        )
    }

    fn make_color_image() -> Box<ColorImage> {
        Box::new(
            ColorImage {
                width: 3,
                height: 4,
                maxval: 255,
                rpixels: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                gpixels: vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
                bpixels: vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
            }
        )
    }

    #[test]
    fn make_image() {
        let _ = make_gray_image();
        let _ = make_color_image();
    }

    #[test]
    fn mutate_image() {
        let mut img = make_gray_image();
        let (w, h) = (img.width, img.height);
        assert_eq!(w, 3);
        assert_eq!(h, 4);

        img.pixels[2] = 69;

        assert_eq!(img.pixels[2], 69);
    }

    #[test]
    fn open_nonexistent_file() {
        let no_file = load_from_file("bogus");
        assert!(no_file.is_err());
    }

    #[test]
    fn open_valid_image_files() {
        // test all 4 variants of image
        let test_images = vec![
            "color_ascii_baldy.ppm",
            "gray_ascii_baldy.pgm",
            "color_raw_baldy.ppm",
            "gray_raw_baldy.pgm",
        ];
        for i in &test_images {
            let img_path = img_folder() + i;
            let good_img = load_from_file(&img_path);
            assert!(good_img.is_ok());
            println!("{:?}", good_img.unwrap());
        }
    }

    #[test]
    fn open_non_image_file() {
        let img_path = img_folder() + "../Cargo.toml";
        let not_image = load_from_file(&img_path);
        assert!(not_image.is_err());
    }

}
