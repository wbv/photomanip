use std::env;
use std::env::ArgsOs;

// these enums and structs define the runtime configuration of our program

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
    let opts = parse_args(env::args());
}




// All the tests
#[cfg(test)]
mod tests {
    use super::*;

    // define a comparator for ProgOpts for testing
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
            "-ob".to_string(),
            "".to_string(),
            "".to_string(),
        ];

        let got = parse_args(cmdline.into_iter());

        assert_eq!(got, should_be);
    }

    #[test]
    fn test_outascii() {
        let should_be = ProgOpts {
            op: ManipOption::DoNothing,
            mode: OutputMode::Ascii,
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

    #[test]
    fn test_negate() {
        let should_be = ProgOpts {
            op: ManipOption::Negate,
            mode: OutputMode::Ascii,
            infile: Box::new(""),
            outfile: Box::new(""),
        };

        let cmdline = vec![
            "-n".to_string(),
            "-oa".to_string(),
            "".to_string(),
            "".to_string(),
        ];

        let got = parse_args(cmdline.into_iter());

        assert_eq!(got, should_be);
    }

    #[test]
    fn test_brighten() {
        let should_be = ProgOpts {
            op: ManipOption::Brighten,
            mode: OutputMode::Ascii,
            infile: Box::new(""),
            outfile: Box::new(""),
        };

        let cmdline = vec![
            "-b".to_string(),
            "24".to_string(),
            "-oa".to_string(),
            "".to_string(),
            "".to_string(),
        ];

        let got = parse_args(cmdline.into_iter());

        assert_eq!(got, should_be);
    }

    #[test]
    #[should_panic]
    fn test_brighten_noarg() {
        let cmdline = vec![
            "-b".to_string(),
            "-oa".to_string(),
            "".to_string(),
            "".to_string(),
        ];

        let got = parse_args(cmdline.into_iter());
    }


    #[test]
    fn test_contrast() {
        let should_be = ProgOpts {
            op: ManipOption::Contrast,
            mode: OutputMode::Ascii,
            infile: Box::new(""),
            outfile: Box::new(""),
        };

        let cmdline = vec![
            "-c".to_string(),
            "-oa".to_string(),
            "".to_string(),
            "".to_string(),
        ];

        let got = parse_args(cmdline.into_iter());

        assert_eq!(got, should_be);
    }

    #[test]
    fn test_grayscale() {
        let should_be = ProgOpts {
            op: ManipOption::Grayscale,
            mode: OutputMode::Ascii,
            infile: Box::new(""),
            outfile: Box::new(""),
        };

        let cmdline = vec![
            "-g".to_string(),
            "-oa".to_string(),
            "".to_string(),
            "".to_string(),
        ];

        let got = parse_args(cmdline.into_iter());

        assert_eq!(got, should_be);
    }

    #[test]
    fn test_smooth() {
        let should_be = ProgOpts {
            op: ManipOption::Smooth,
            mode: OutputMode::Ascii,
            infile: Box::new(""),
            outfile: Box::new(""),
        };

        let cmdline = vec![
            "-s".to_string(),
            "-oa".to_string(),
            "".to_string(),
            "".to_string(),
        ];

        let got = parse_args(cmdline.into_iter());

        assert_eq!(got, should_be);
    }

    #[test]
    fn test_sharpen() {
        let should_be = ProgOpts {
            op: ManipOption::Sharpen,
            mode: OutputMode::Ascii,
            infile: Box::new(""),
            outfile: Box::new(""),
        };

        let cmdline = vec![
            "-p".to_string(),
            "-oa".to_string(),
            "".to_string(),
            "".to_string(),
        ];

        let got = parse_args(cmdline.into_iter());

        assert_eq!(got, should_be);
    }

    #[test]
    #[should_panic]
    fn test_extra_contrast_arg() {
        let cmdline = vec![
            "-c".to_string(),
            "69".to_string(),
            "-oa".to_string(),
            "".to_string(),
            "".to_string(),
        ];

        let got = parse_args(cmdline.into_iter());
    }

    #[test]
    #[should_panic]
    fn test_two_args() {
        let cmdline = vec![
            "-c".to_string(),
            "bad".to_string(),
        ];

        let got = parse_args(cmdline.into_iter());
    }

    #[test]
    #[should_panic]
    fn test_one_arg() {
        let cmdline = vec![
            "bad".to_string(),
        ];

        let got = parse_args(cmdline.into_iter());
    }

    #[test]
    #[should_panic]
    fn test_no_args() {
        let cmdline = vec![];

        let got = parse_args(cmdline.into_iter());
    }
}
