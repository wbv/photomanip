mod args;
mod image;

use args::ProgOpts;

fn main() -> Result<(), ()> {
    let _ = ProgOpts::from_env();

    Ok(())
}


