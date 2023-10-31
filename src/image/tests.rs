/////////////////////////////
// unit tests for image.rs //
/////////////////////////////
use super::*;

////////////////////////////////
// Helper functions for tests //
////////////////////////////////

fn img_folder() -> String {
    env!("CARGO_MANIFEST_DIR").to_owned() + "/img/"
}

fn make_gray_image() -> Box<GrayImage<u8>> {
    Box::new(
        GrayImage {
            width: 3,
            height: 4,
            maxval: 15,
            pixels: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        }
    )
}

fn make_color_image() -> Box<ColorImage<u8>> {
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

// TODO:
//   compare equivalent gray images with different maxvals
//   compare equivalent color images with different maxvals
//   compare equivalent gray and color images
//
//impl PartialEq for ColorImage<u8> {
//    fn eq(&self, other: &Self) -> bool { true }
//}


//////////////////
// actual tests //
//////////////////

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
fn open_color8_raw_image() {
    match load_from_file(&(img_folder() + "feep_raw.ppm")).unwrap() {
        ImageType::Color8(img) => {
            assert_eq!(img.rpixels.len(), 16);
            assert_eq!(img.gpixels.len(), 16);
            assert_eq!(img.bpixels.len(), 16);
            assert_eq!(img.rpixels[0], 0);
            assert_eq!(img.rpixels[3], 15);
            assert_eq!(img.gpixels[5], 15);
            assert_eq!(img.bpixels[10], 7);
        }
        x => {
            panic!("Parsed into wrong image type {:?}", x);
        }
    }
}

#[test]
fn open_non_image_file() {
    let img_path = img_folder() + "../Cargo.toml";
    let not_image = load_from_file(&img_path);
    assert!(not_image.is_err());
}

#[test]
fn handle_mac_line_endings() {
    let cr_img = load_from_file(&(img_folder() + "ascii_cr_wisdom.ppm"));
    let unix_img = load_from_file(&(img_folder() + "ascii_wisdom.ppm"));
    assert_eq!(cr_img.unwrap(), unix_img.unwrap());
}

#[test]
fn handle_windows_line_endings() {
    let crlf_img = load_from_file(&(img_folder() + "ascii_crlf_wisdom.ppm"));
    let unix_img = load_from_file(&(img_folder() + "ascii_wisdom.ppm"));
    assert_eq!(crlf_img.unwrap(), unix_img.unwrap());
}

#[test]
fn raw_and_ascii_pgms_equal() {
    let raw_img   = load_from_file(&(img_folder() + "feep_raw.pgm")).unwrap();
    let ascii_img = load_from_file(&(img_folder() + "feep.pgm"    )).unwrap();
    assert_eq!(raw_img, ascii_img);
}

#[test]
fn raw_and_ascii_ppms_equal() {
    let raw_img   = load_from_file(&(img_folder() + "feep_raw.ppm")).unwrap();
    let ascii_img = load_from_file(&(img_folder() + "feep.ppm"    )).unwrap();
    assert_eq!(raw_img, ascii_img);
}

#[test]
fn different_maxval_equal_pgms() {
    let my_img   = load_from_file(&(img_folder() + "feep_raw.pgm"     )).unwrap();
    let gimp_img = load_from_file(&(img_folder() + "feep_raw_gimp.pgm")).unwrap();
    assert_eq!(my_img, gimp_img);
}

#[test]
fn different_maxval_equal_ppms() {
    let my_img   = load_from_file(&(img_folder() + "feep_raw.ppm"     )).unwrap();
    let gimp_img = load_from_file(&(img_folder() + "feep_raw_gimp.ppm")).unwrap();
    assert_eq!(my_img, gimp_img);
}
