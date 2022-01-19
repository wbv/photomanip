pub trait Image {
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

impl Image for ColorImage {
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

impl Image for GrayImage {
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

    fn make_image() -> GrayImage {
        return GrayImage {
            width: 3,
            height: 4,
            maxval: 255,
            pixels: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
    }

    #[test]
    fn test_make_image() {
        let _ = make_image();
    }

    #[test]
    fn test_mutate_image() {
        let mut img = make_image();
        let (w, h) = (img.width, img.height);
        assert_eq!(w, 3);
        assert_eq!(h, 4);

        img.pixels[2] = 69;
    }
}
