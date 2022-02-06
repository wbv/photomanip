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
    GrayImage(Box<GrayImage>),
    ColorImage(Box<ColorImage>),
}

pub fn load_from_file(path: &str) -> io::Result<Image> {
    let mut data = Vec::<u8>::new();
    let mut file = File::open(path)?;
    let n = file.read_to_end(&mut data)?;

    let (grayscale, ascii) = 
        match std::str::from_utf8(&data[0..2]) {
           Ok("P6") => { println!("Found a P6"); (false, false) },
           Ok("P5") => { println!("Found a P5"); (true,  false) },
           Ok("P3") => { println!("Found a P3"); (false, true)  },
           Ok("P2") => { println!("Found a P2"); (true,  true)  },
           _ => {
               eprintln!("Not a valid PPM/PGM file: '{}'", path);
               return Err(io::Error::new(
                io::ErrorKind::InvalidData, "Not a PPM/PGM file"
               ))
           }
        };

    if grayscale {
        Ok(Image::GrayImage(Box::new(
            GrayImage {
                width: 1,
                height: 1,
                maxval: 1,
                pixels: vec![0],
            }
        )))
    }

    else {
        Ok(Image::ColorImage(Box::new(
            ColorImage {
                width: 1,
                height: 1,
                maxval: 1,
                rpixels: vec![0],
                gpixels: vec![0],
                bpixels: vec![0]
            }
        )))
    }
}


impl ImageManip for ColorImage {
    fn brighten(&self, amount: i32) -> ColorImage {
        // TODO: STUB
        return ColorImage {
            width: 1,
            height: 1,
            maxval: 1,
            rpixels: vec![0],
            gpixels: vec![0],
            bpixels: vec![0]
        }
    }
    fn contrast(&self) -> ColorImage {
        // TODO: STUB
        return ColorImage {
            width: 1,
            height: 1,
            maxval: 1,
            rpixels: vec![0],
            gpixels: vec![0],
            bpixels: vec![0]
        }
    }
    fn grayscale(&self) -> ColorImage {
        // TODO: STUB
        return ColorImage {
            width: 1,
            height: 1,
            maxval: 1,
            rpixels: vec![0],
            gpixels: vec![0],
            bpixels: vec![0]
        }
    }
    fn negate(&self) -> ColorImage {
        // TODO: STUB
        return ColorImage {
            width: 1,
            height: 1,
            maxval: 1,
            rpixels: vec![0],
            gpixels: vec![0],
            bpixels: vec![0]
        }
    }
    fn sharpen(&self) -> ColorImage {
        // TODO: STUB
        return ColorImage {
            width: 1,
            height: 1,
            maxval: 1,
            rpixels: vec![0],
            gpixels: vec![0],
            bpixels: vec![0]
        }
    }
    fn smooth(&self) -> ColorImage {
        // TODO: STUB
        return ColorImage {
            width: 1,
            height: 1,
            maxval: 1,
            rpixels: vec![0],
            gpixels: vec![0],
            bpixels: vec![0]
        }
    }
}

impl ImageManip for GrayImage {
    fn brighten(&self, amount: i32) -> ColorImage {
        // TODO: STUB
        return ColorImage {
            width: 1,
            height: 1,
            maxval: 1,
            rpixels: vec![0],
            gpixels: vec![0],
            bpixels: vec![0]
        }
    }
    fn contrast(&self) -> ColorImage {
        // TODO: STUB
        return ColorImage {
            width: 1,
            height: 1,
            maxval: 1,
            rpixels: vec![0],
            gpixels: vec![0],
            bpixels: vec![0]
        }
    }
    fn grayscale(&self) -> ColorImage {
        // TODO: STUB
        return ColorImage {
            width: 1,
            height: 1,
            maxval: 1,
            rpixels: vec![0],
            gpixels: vec![0],
            bpixels: vec![0]
        }
    }
    fn negate(&self) -> ColorImage {
        // TODO: STUB
        return ColorImage {
            width: 1,
            height: 1,
            maxval: 1,
            rpixels: vec![0],
            gpixels: vec![0],
            bpixels: vec![0]
        }
    }
    fn sharpen(&self) -> ColorImage {
        // TODO: STUB
        return ColorImage {
            width: 1,
            height: 1,
            maxval: 1,
            rpixels: vec![0],
            gpixels: vec![0],
            bpixels: vec![0]
        }
    }
    fn smooth(&self) -> ColorImage {
        // TODO: STUB
        return ColorImage {
            width: 1,
            height: 1,
            maxval: 1,
            rpixels: vec![0],
            gpixels: vec![0],
            bpixels: vec![0]
        }
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
    fn open() {
        let bad_img = load_from_file("bogus");
        assert_eq!(bad_img.is_err(), true);

        let img_path = img_folder() + "color_ascii_baldy.ppm";
        println!("{}", img_path);
        let good_img = load_from_file(&img_path);
        assert_eq!(good_img.is_ok(), true);
    }

}
