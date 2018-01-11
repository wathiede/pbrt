use std::convert;
use std::str;
use std::str::FromStr;

extern crate nom;
use self::nom::{alphanumeric, digit, double, double_s, is_digit, rest, space, ErrorKind, IResult,
                InputLength, Slice};

extern crate regex;

#[derive(Debug, PartialEq)]
struct LookAt {
    values: Vec<f64>,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(
    look_at<LookAt>,
    ws!(do_parse!(
        tag!("LookAt") >>
        values: many1!(number) >>
        (LookAt{
            values: values
        })
    ))
);

#[derive(Debug, PartialEq)]
struct Param {
    typ: String,
    name: String,
    // TODO(wathiede): make this a trait that handles GetFirst, etc. and only the concrete types
    // are converted to int, float, etc.
    values: Vec<f64>,
}

// number is a superset of nom::double! that includes '1' and '1.'
fn number(input: &[u8]) -> IResult<&[u8], f64> {
    flat_map!(
        input,
        recognize!(tuple!(
            opt!(alt!(tag!("+") | tag!("-"))),
            alt!(
                complete!(delimited!(opt!(digit), tag!("."), digit))
                    | complete!(preceded!(digit, tag!("."))) | digit
            ),
            opt!(complete!(tuple!(
                alt!(tag!("e") | tag!("E")),
                opt!(alt!(tag!("+") | tag!("-"))),
                digit
            )))
        )),
        parse_to!(f64)
    )
}

fn strip_comment(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let re = regex::bytes::Regex::new(r"#.*\n").unwrap();
    let res = re.replace_all(input, &b"\n"[..]);

    IResult::Done(&b""[..], Vec::from(res))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(param_set_item_values<Vec<f64>>,
    alt!(
        ws!(delimited!(tag!("["), many1!(number), tag!("]")))
        | ws!(many1!(number))
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(
    param_set_item_type_name<(&str, &str)>,
    do_parse!(
        typ: map_res!(alphanumeric, str::from_utf8) >>
        space >>
        name: map_res!(alphanumeric, str::from_utf8) >>
        ((typ, name))
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(
    param_set_item<Param>,
    do_parse!(
        tag!("\"") >>
        tn: param_set_item_type_name >>
        tag!("\"") >>
        values: param_set_item_values >>
        (Param {
            typ: tn.0.to_owned(),
            name: tn.1.to_owned(),
            values: values,
        })
    )
);

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use nom::IResult;

    use super::*;

    fn dump<O>(res: &IResult<&[u8], O>)
    where
        O: Debug,
    {
        match res {
            &IResult::Done(ref i, ref o) => println!("i: {:?} | o: {:?}", str::from_utf8(i), o),
            &IResult::Error(ref e) => panic!("e: {:#?}", e),
            &IResult::Incomplete(n) => panic!("need: {:?}", n),
        }
    }

    //#[test]
    //fn test_prefix() {
    //    named!( preceded_wrap<&[u8], &[u8]>, preceded!(tag!("a"), alphanumeric) );
    //    assert_eq!(
    //        preceded_wrap(&b"axyz"[..]),
    //        IResult::Done(&b""[..], &b"xyz"[..])
    //    );
    //    named!( terminated_wrap<&[u8], &[u8]>, terminated!(alphanumeric, tag!("\n")) );
    //    assert_eq!(
    //        terminated_wrap(&b"xyz\n"[..]),
    //        IResult::Done(&b"\n"[..], &b"xyz"[..])
    //    );
    //}

    #[test]
    fn test_strip_comment() {
        assert_eq!(
            strip_comment(&b"a # comment\nb # comment\n"[..]),
            IResult::Done(&b""[..], Vec::from(&b"a \nb \n"[..]))
        );
        assert_eq!(
            strip_comment(&b"a # comment\nb\n"[..]),
            IResult::Done(&b""[..], Vec::from(&b"a \nb\n"[..]))
        );
        assert_eq!(
            strip_comment(&b"a\nb\n"[..]),
            IResult::Done(&b""[..], Vec::from(&b"a\nb\n"[..]))
        );
        assert_eq!(
            strip_comment(&b"a\nb # comment\n"[..]),
            IResult::Done(&b""[..], Vec::from(&b"a\nb \n"[..]))
        );
    }

    #[test]
    fn test_number() {
        assert_eq!(number(&b"3.14"[..]), IResult::Done(&b""[..], 3.14));
        assert_eq!(number(&b".1"[..]), IResult::Done(&b""[..], 0.1));
        assert_eq!(number(&b"0.2"[..]), IResult::Done(&b""[..], 0.2));
        assert_eq!(number(&b"3."[..]), IResult::Done(&b""[..], 3.));
        assert_eq!(number(&b"4"[..]), IResult::Done(&b""[..], 4.));
    }

    #[test]
    fn test_number_comment_number() {
        if let IResult::Done(_, input) = strip_comment(&b"[ 1 # comment\n2 3]\n"[..]) {
            let input: &[u8] = &input;
            let ref res = param_set_item_values(input);
            dump(res);
            assert_eq!(res, &IResult::Done(&b""[..], vec![1., 2., 3.]));
        };
    }

    #[test]
    fn test_param_set_item_type_name() {
        let input = &b"float foo"[..];
        let ref res = param_set_item_type_name(input);
        dump(res);
        assert_eq!(res, &IResult::Done(&b""[..], ("float", "foo")));
    }

    #[test]
    fn test_param_set_item_values() {
        let input = &b"1 2. -3.0"[..];
        let ref res = param_set_item_values(input);
        dump(res);
        assert_eq!(res, &IResult::Done(&b""[..], vec![1., 2., -3.]));

        let input = &b"[  1 2 3]"[..];
        let ref res = param_set_item_values(input);
        assert_eq!(res, &IResult::Done(&b""[..], vec![1., 2., 3.]));

        // TODO(wathiede): support non-float types:
        // "string filename" "foo.exr"
        // "point origin" [ 0 1 2 ]
        // "normal N" [ 0 1 0  0 0 1 ] # array of 2 normal values
        // "bool renderquickly" "true"
    }

    #[test]
    fn test_param_set_item() {
        let input = &b"\"float foo\" [ 0 1 2 ]"[..];
        let ref res = param_set_item(input);
        assert_eq!(
            res,
            &IResult::Done(
                &b""[..],
                Param {
                    typ: "float".to_owned(),
                    name: "foo".to_owned(),
                    values: vec![0., 1., 2.],
                }
            )
        );
    }

    #[test]
    fn test_look_at() {
        let input = &b"LookAt 3 4 1.5  # eye\n \
    .5 .5 0  # look at point\n \
    0 0 1    # up vector\n"[..];
        if let IResult::Done(_, input) = strip_comment(input) {
            let input: &[u8] = &input;
            let ref res = look_at(input);
            dump(res);
            assert_eq!(
                res,
                &IResult::Done(
                    &b""[..],
                    LookAt {
                        values: vec![3., 4., 1.5, 0.5, 0.5, 0., 0., 0., 1.],
                    }
                )
            );
        };
    }
}
