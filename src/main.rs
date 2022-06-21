use std::env;

mod args;
mod image;


fn main() -> Result<(), ()> {
    let _ = args::parse(env::args());

    Ok(())
}


