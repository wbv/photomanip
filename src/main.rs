use std::env;

mod image;

//
// these enums and structs define the runtime configuration of our program
//
/// The possible manipulation options, including "Do Nothing"
#[derive(Debug, PartialEq)]
enum ManipOption {
    DoNothing,
    Negate,
    Brighten(usize),
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
struct ProgOpts {
    op: ManipOption,
    mode: OutputMode,
    infile: String,
    outfile: String,
}


/// Parses the commandline variables into a program state struct
fn parse_args<I>(args: I) -> Result<ProgOpts, ()>
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

fn main() -> Result<(), ()> {
    let _ = parse_args(env::args());

    Ok(())
}



///////////////////////////////////////////////////////////////////////////////
// unit tests /////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // define a comparator for ProgOpts for testing
    impl PartialEq for ProgOpts {
        fn eq(&self, other: &ProgOpts) -> bool {
            self.op      == other.op      &&
            self.mode    == other.mode    &&
            self.infile  == other.infile  &&
            self.outfile == other.outfile
        }
    }

    // form an env::Args -like String iterator
    fn to_args(cmd: &'static str) -> Vec<String> {
        cmd.split(' ').map(|s| String::from(s)).collect()
    }

    #[test]
    fn outbin() {
        let should_be = ProgOpts {
            op: ManipOption::DoNothing,
            mode: OutputMode::Binary,
            infile: String::from("infile"),
            outfile: String::from("outfile"),
        };

        let cmdline = to_args("-ob infile outfile");
        let got = parse_args(cmdline.into_iter()).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn outascii() {
        let should_be = ProgOpts {
            op: ManipOption::DoNothing,
            mode: OutputMode::Ascii,
            infile: String::from("infile"),
            outfile: String::from("outfile"),
        };

        let cmdline = to_args("-oa infile outfile");
        let got = parse_args(cmdline.into_iter()).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn outfile() {
        let should_be = ProgOpts {
            op: ManipOption::DoNothing,
            mode: OutputMode::Ascii,
            infile: String::from("infile"),
            outfile: String::from("test!"),
        };

        let cmdline = to_args("-oa infile test!");
        let got = parse_args(cmdline.into_iter()).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn infile() {
        let should_be = ProgOpts {
            op: ManipOption::DoNothing,
            mode: OutputMode::Ascii,
            infile: String::from("tested!"),
            outfile: String::from("outfile"),
        };

        let cmdline = to_args("-oa tested! outfile");
        let got = parse_args(cmdline.into_iter()).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn negate() {
        let should_be = ProgOpts {
            op: ManipOption::Negate,
            mode: OutputMode::Ascii,
            infile: String::from("infile"),
            outfile: String::from("outfile"),
        };

        let cmdline = to_args("-n -oa infile outfile");
        let got = parse_args(cmdline.into_iter()).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn brighten() {
        let should_be = ProgOpts {
            op: ManipOption::Brighten(24),
            mode: OutputMode::Ascii,
            infile: String::from("infile"),
            outfile: String::from("outfile"),
        };

        let cmdline = to_args("-b 24 -oa infile outfile");
        let got = parse_args(cmdline.into_iter()).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn brighten_noarg() {
        let cmdline = to_args("-b -oa infile outfile");
        let got = parse_args(cmdline.into_iter());
        assert_eq!(got, Err(()));
    }

    #[test]
    fn contrast() {
        let should_be = ProgOpts {
            op: ManipOption::Contrast,
            mode: OutputMode::Ascii,
            infile: String::from("infile"),
            outfile: String::from("outfile"),
        };

        let cmdline = to_args("-c -oa infile outfile");
        let got = parse_args(cmdline.into_iter()).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn grayscale() {
        let should_be = ProgOpts {
            op: ManipOption::Grayscale,
            mode: OutputMode::Ascii,
            infile: String::from("infile"),
            outfile: String::from("outfile"),
        };

        let cmdline = to_args("-g -oa infile outfile");
        let got = parse_args(cmdline.into_iter()).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn smooth() {
        let should_be = ProgOpts {
            op: ManipOption::Smooth,
            mode: OutputMode::Ascii,
            infile: String::from("infile"),
            outfile: String::from("outfile"),
        };

        let cmdline = to_args("-s -oa infile outfile");
        let got = parse_args(cmdline.into_iter()).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn sharpen() {
        let should_be = ProgOpts {
            op: ManipOption::Sharpen,
            mode: OutputMode::Ascii,
            infile: String::from("infile"),
            outfile: String::from("outfile"),
        };

        let cmdline = to_args("-p -oa infile outfile");
        let got = parse_args(cmdline.into_iter()).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn extra_contrast_arg() {
        let cmdline = to_args("-c 69 -oa infile outfile");
        let got = parse_args(cmdline.into_iter());
        assert_eq!(got, Err(()));
    }

    #[test]
    fn two_args() {
        let cmdline = to_args("-c bad");
        let got = parse_args(cmdline.into_iter());
        assert_eq!(got, Err(()));
    }

    #[test]
    fn one_arg() {
        let cmdline = to_args("bad");
        let got = parse_args(cmdline.into_iter());
        assert_eq!(got, Err(()));
    }

    #[test]
    fn no_args() {
        let cmdline = to_args("");
        let got = parse_args(cmdline.into_iter());
        assert_eq!(got, Err(()));
    }

    #[test]
    fn six_args() {
        let cmdline = to_args("1 2 3 4 5 6");
        let got = parse_args(cmdline.into_iter());
        assert_eq!(got, Err(()));
    }
}
