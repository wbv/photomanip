use super::*;


#[test]
fn outbin() {
    let should_be = ProgOpts {
        op: ManipOption::DoNothing,
        mode: OutputMode::Binary,
        infile: String::from("infile"),
        outfile: String::from("outfile"),
    };

    let got: ProgOpts = "-ob infile outfile".parse().unwrap();
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

    let got: ProgOpts = "-oa infile outfile".parse().unwrap();
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

    let got: ProgOpts = "-oa infile test!".parse().unwrap();
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

    let got: ProgOpts = "-oa tested! outfile".parse().unwrap();
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

    let got: ProgOpts = "-n -oa infile outfile".parse().unwrap();
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

    let got: ProgOpts = "-b 24 -oa infile outfile".parse().unwrap();
    assert_eq!(got, should_be);
}

#[test]
fn brighten_noarg() {
    assert!("-b -oa infile outfile".parse::<ProgOpts>().is_err());
}

#[test]
fn contrast() {
    let should_be = ProgOpts {
        op: ManipOption::Contrast,
        mode: OutputMode::Ascii,
        infile: String::from("infile"),
        outfile: String::from("outfile"),
    };

    let got: ProgOpts = "-c -oa infile outfile".parse().unwrap();
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

    let got: ProgOpts = "-g -oa infile outfile".parse().unwrap();
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

    let got: ProgOpts = "-s -oa infile outfile".parse().unwrap();
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

    let got: ProgOpts = "-p -oa infile outfile".parse().unwrap();
    assert_eq!(got, should_be);
}

#[test]
fn extra_contrast_arg() {
    assert!("-c 69 -oa infile outfile".parse::<ProgOpts>().is_err());
}

#[test]
fn two_args() {
    assert!("-c bad".parse::<ProgOpts>().is_err());
}

#[test]
fn one_arg() {
    assert!("bad".parse::<ProgOpts>().is_err());
}

#[test]
fn no_args() {
    assert!("".parse::<ProgOpts>().is_err());
}

#[test]
fn six_args() {
    assert!("1 2 3 4 5 6".parse::<ProgOpts>().is_err());
}
