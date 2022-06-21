#[cfg(test)]
mod tests;

//
// these enums and structs define the runtime configuration of our program
//
/// The possible manipulation options, including "Do Nothing"
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum ManipOption {
    DoNothing,
    Negate,
    Brighten(usize),
    Sharpen,
    Smooth,
    Grayscale,
    Contrast,
}

/// Output mode for the image written out
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum OutputMode {
    Ascii,
    Binary,
}

/// All runtime options collected together
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct ProgOpts {
    op: ManipOption,
    mode: OutputMode,
    infile: String,
    outfile: String,
}


/// Parses the commandline variables into a program state struct
pub fn parse<I>(args: I) -> Result<ProgOpts, ()>
where
    I: Iterator<Item = String>
{
    let args: Vec<String> = args.collect();

    let op = match args.len() {
        3 => ManipOption::DoNothing,
        4 => match args[0].as_str() {
            "-c" => ManipOption::Contrast,
            "-g" => ManipOption::Grayscale,
            "-n" => ManipOption::Negate,
            "-p" => ManipOption::Sharpen,
            "-s" => ManipOption::Smooth,
            _ => return Err(()),
        },
        5 => match args[0].as_str() {
            "-b" => ManipOption::Brighten(usize::from_str_radix(&args[1], 10).unwrap()),
            _ => return Err(()),
        },
        _ => return Err(()),
    };

    let mode = match args[args.len() - 3].as_str() {
        "-oa" => OutputMode::Ascii,
        "-ob" => OutputMode::Binary,
        _ => return Err(()),
    };

    return Ok(ProgOpts {
        op: op,
        mode: mode,
        infile: args[args.len() - 2].clone(),
        outfile: args[args.len() - 1].clone(),
    });
}
