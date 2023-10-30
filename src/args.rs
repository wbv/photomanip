use std::convert::TryInto;
use std::env::Args;
use std::str::FromStr;


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

/// `ProgOpts` contain the runtime options for a single invocation of photomanip
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct ProgOpts {
    op: ManipOption,
    mode: OutputMode,
    infile: String,
    outfile: String,
}

impl ProgOpts {
    pub fn from_env() -> Result<ProgOpts, String> {
        ProgOpts::parse(std::env::args())
    }

    /// Parses the given command line `args` into explicit program options
    pub fn parse<I>(args: I) -> Result<ProgOpts, String>
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
                other => return Err(format!("Unexpected option '{}'", other)),
            },
            5 => match args[0].as_str() {
                "-b" => ManipOption::Brighten(usize::from_str_radix(&args[1], 10).unwrap()),
                _ => return Err(format!("")),
            },
            _ => return Err(format!("")),
        };

        let mode = match args[args.len() - 3].as_str() {
            "-oa" => OutputMode::Ascii,
            "-ob" => OutputMode::Binary,
            _ => return Err(format!("")),
        };

        return Ok(ProgOpts {
            op,
            mode,
            infile: args[args.len() - 2].clone(),
            outfile: args[args.len() - 1].clone(),
        });
    }
}

impl TryInto<ProgOpts> for Args {
    type Error = String;

    /// Attempts to interpret the current [std::env::Args] as program options
    fn try_into(self) -> Result<ProgOpts, Self::Error> {
        ProgOpts::parse(self)
    }
}

impl FromStr for ProgOpts {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args = s.split_whitespace().map(String::from);
        ProgOpts::parse(args)
    }
}
