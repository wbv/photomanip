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

    // interpret the kind of PPM from the magic sequence
    let (grayscale, ascii) = 
        match std::str::from_utf8(&data[0..2]) {
           Ok("P6") => { (false, false) },
           Ok("P5") => { (true,  false) },
           Ok("P3") => { (false, true)  },
           Ok("P2") => { (true,  true)  },
           _ => {
               eprintln!("Not a valid PPM/PGM file: '{}'", path);
               return Err(io::Error::new(
                   io::ErrorKind::InvalidData, "Not a PPM/PGM file"
               ))
           }
        };

    // scan through the file header following the magic seqence, byte at a time
    // `scanner` is our index into data as we scan
    let mut scanner: usize = 2;

    // skip whitespace following the magic sequence
    for i in scanner..data.len() {
        if data[i].is_ascii_whitespace() {
            scanner = i+1;
            break;
        }
    }

    // check for line comment, read 'til EOL
    if data[scanner] == b'#' {
        while data[scanner] != b'\n' {
            scanner = scanner + 1;
        }
    }

    // skip until width string
    for i in scanner..data.len() {
        if data[i].is_ascii_whitespace() {
            scanner = i+1;
            break;
        }
    }

    // form the width from the next non-whitespace characters
    let mut width_str = String::new();
    for i in scanner..data.len() {
        if !data[i].is_ascii_whitespace() {
            width_str.push(data[i].clone() as char)
        } else {
            break;
        }
    }
    let width = match width_str.parse::<usize>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid ascii width found: '{}'", width_str);
            eprintln!("at postion {}", scanner);
            return Err(io::Error::new(
                io::ErrorKind::InvalidData, "Invalid width parameter"
            ))
        }
    };

    // skip until width string
    for i in scanner..data.len() {
        if data[i].is_ascii_whitespace() {
            scanner = i+1;
            break;
        }
    }

    // form the height from the next non-whitespace characters
    let mut height_str = String::new();
    for i in scanner..data.len() {
        if !data[i].is_ascii_whitespace() {
            height_str.push(data[i].clone() as char)
        } else {
            break;
        }
    }
    let height = match height_str.parse::<usize>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid ascii height found: '{}'", height_str);
            eprintln!("at postion {}", scanner);
            return Err(io::Error::new(
                io::ErrorKind::InvalidData, "Invalid height parameter"
            ))
        }
    };

    // form the maxval from the next non-whitespace characters
    let mut maxval_str = String::new();
    for i in scanner..data.len() {
        if !data[i].is_ascii_whitespace() {
            maxval_str.push(data[i].clone() as char)
        } else {
            break;
        }
    }
    let maxval = match maxval_str.parse::<usize>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid ascii maxval found: '{}'", maxval_str);
            eprintln!("at postion {}", scanner);
            return Err(io::Error::new(
                io::ErrorKind::InvalidData, "Invalid height parameter"
            ))
        }
    };

    if grayscale {
        Ok(Image::GrayImage(Box::new(
            GrayImage {
                width: width,
                height: height,
                maxval: maxval,
                pixels: vec![0],
            }
        )))
    }

    else {
        Ok(Image::ColorImage(Box::new(
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
    fn open_nonexistent_file() {
        let no_file = load_from_file("bogus");
        assert!(no_file.is_err());
    }

    #[test]
    fn open_valid_image_files() {
        // test all 4 variants of 
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
        }
    }

    #[test]
    fn open_non_image_file() {
        let img_path = img_folder() + "../Cargo.toml";
        let not_image = load_from_file(&img_path);
        assert!(not_image.is_err());
    }

}
