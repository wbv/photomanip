use super::{ManipOption, OutputMode, ProgOpts, parse};

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
    let got = parse(cmdline.into_iter()).unwrap();
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
    let got = parse(cmdline.into_iter()).unwrap();
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
    let got = parse(cmdline.into_iter()).unwrap();
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
    let got = parse(cmdline.into_iter()).unwrap();
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
    let got = parse(cmdline.into_iter()).unwrap();
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
    let got = parse(cmdline.into_iter()).unwrap();
    assert_eq!(got, should_be);
}

#[test]
fn brighten_noarg() {
    let cmdline = to_args("-b -oa infile outfile");
    let got = parse(cmdline.into_iter());
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
    let got = parse(cmdline.into_iter()).unwrap();
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
    let got = parse(cmdline.into_iter()).unwrap();
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
    let got = parse(cmdline.into_iter()).unwrap();
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
    let got = parse(cmdline.into_iter()).unwrap();
    assert_eq!(got, should_be);
}

#[test]
fn extra_contrast_arg() {
    let cmdline = to_args("-c 69 -oa infile outfile");
    let got = parse(cmdline.into_iter());
    assert_eq!(got, Err(()));
}

#[test]
fn two_args() {
    let cmdline = to_args("-c bad");
    let got = parse(cmdline.into_iter());
    assert_eq!(got, Err(()));
}

#[test]
fn one_arg() {
    let cmdline = to_args("bad");
    let got = parse(cmdline.into_iter());
    assert_eq!(got, Err(()));
}

#[test]
fn no_args() {
    let cmdline = to_args("");
    let got = parse(cmdline.into_iter());
    assert_eq!(got, Err(()));
}

#[test]
fn six_args() {
    let cmdline = to_args("1 2 3 4 5 6");
    let got = parse(cmdline.into_iter());
    assert_eq!(got, Err(()));
}
