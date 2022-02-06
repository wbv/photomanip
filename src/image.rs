use std::io;
use std::path::Path;

pub trait ImageManip {
    fn brighten(&self, amount: i32) -> ColorImage;
    fn contrast(&self) -> ColorImage;
    fn grayscale(&self) -> ColorImage;
    fn negate(&self) -> ColorImage;
    fn sharpen(&self) -> ColorImage;
    fn smooth(&self) -> ColorImage;
}

pub struct ColorImage {
    width: usize,
    height: usize,
    maxval: usize,
    rpixels: Vec<u16>,
    gpixels: Vec<u16>,
    bpixels: Vec<u16>,
}

pub struct GrayImage {
    width: usize,
    height: usize,
    maxval: usize,
    pixels: Vec<u16>,
}

pub enum Image {
    GrayImage(Box<GrayImage>),
    ColorImage(Box<ColorImage>),
}

pub fn load_from_file(path: &Path) -> io::Result<Image> {
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
        let mut img = load_from_file(Path::new("bogus")).unwrap();

        match img {
            Image::ColorImage(mut img) => img.rpixels[img.width - 1] = 42,
            Image::GrayImage(mut img) => img.pixels[img.width - 1] = 12,
        }
    }

}
