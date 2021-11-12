pub struct Image {
    width: usize,
    height: usize,
    color: bool,
    rpixels: Box<[u8]>,
    gpixels: Box<[u8]>,
    bpixels: Box<[u8]>,
}



///////////////////////////////////////////////////////////////////////////////
// unit tests /////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    fn make_image() -> Image {
        return Image {
            width: 3,
            height: 4,
            color: false,
            rpixels: Box::new([]),
            gpixels: Box::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
            bpixels: Box::new([]),
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

        img.gpixels[2] = 69;
    }
}
