use std::env;
use std::env::ArgsOs;

/* these enums and structs define the runtime configuration of our program */

/// The possible manipulation options, including "Do Nothing"
#[derive(Debug, PartialEq)]
enum ManipOption {
    DoNothing,
    Negate,
    Brighten,
    Sharpen,
    Smooth,
    Grayscale,
    Contrast,
}

/// Output mode for the image written out
#[derive(Debug, PartialEq)]
enum OutputMode {
    Ascii,
    Binary,
}

/// All runtime options collected together
#[derive(Debug)]
struct ProgOpts<'a> {
    op: ManipOption,
    mode: OutputMode,
    infile: Box<&'a str>,
    outfile: Box<&'a str>,
}


/// TODO: implement
/// Parses the commandline variables into a program state struct
fn parse_args<'a, I>(args: I) -> ProgOpts<'a>
where
    I: Iterator<Item = String>
{
    // Stub. Return a default
    ProgOpts {
        op: ManipOption::DoNothing,
        mode: OutputMode::Ascii,
        infile: Box::new(""),
        outfile: Box::new("")
    }
}

fn main() {
    for arg in env::args_os() {
        println!("{:?}", arg);
    }

    parse_args(env::args());
}



#[cfg(test)]
mod tests {
    use super::*;

    impl<'a> PartialEq for ProgOpts<'a> {
        fn eq(&self, other: &ProgOpts<'a>) -> bool {
            self.op      == other.op      &&
            self.mode    == other.mode    &&
            self.infile  == other.infile  &&
            self.outfile == other.outfile
        }
    }

    #[test]
    fn test_outbin() {
        let should_be = ProgOpts {
            op: ManipOption::DoNothing,
            mode: OutputMode::Binary,
            infile: Box::new(""),
            outfile: Box::new(""),
        };

        let cmdline = vec![
            "-oa".to_string(),
            "".to_string(),
            "".to_string(),
        ];

        let got = parse_args(cmdline.into_iter());

        assert_eq!(got, should_be);
    }
}
