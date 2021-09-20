use std::env;

/* All possible manipulation options */
enum ManipOption {
    DoNothing,
    Negate,
    Brighten,
    Sharpen,
    Smooth,
    Grayscale,
    Contrast,
}


fn main() {
    for arg in env::args_os() {
        println!("{:?}", arg);
    }
}
