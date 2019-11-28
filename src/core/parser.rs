// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::str;
use std::str::FromStr;

// We normally don't do wildcard imports, all the nom macros use other macros, and this is just
// easier.
use nom::*;

use regex;

use crate::core::geometry::Point3f;
use crate::core::paramset::{ParamList, ParamSet, ParamSetItem, Value};
use crate::core::pbrt::Float;

#[derive(PartialEq, Debug)]
pub enum Error {
    NomError(nom::Err),
    NomIncomplete(nom::Needed),
}

// TODO(wathiede): why does this result in:
//      expected at least 2 type arguments
// impl From<IResult> for Error {
//     fn from(err: IResult) -> Error {
//         match err {
//             IResult::Error(e) => Error::NomError(e),
//             IResult::Incomplete(n) => Error::NomIncomplete(n),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub enum Directive {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    // TODO(wathiede): convert to 3 x Vector3f?
    LookAt(
        Float, Float, Float, // eye xyz
        Float, Float, Float, // look xyz
        Float, Float, Float, // up xyz
    ),
    Camera(String, ParamSet),
    Sampler(String, ParamSet),
    Integrator(String, ParamSet),
    Film(String, ParamSet),
    WorldBegin,
    WorldEnd,
    AttributeBegin,
    AttributeEnd,
    LightSource(String, ParamSet),
    Material(String, ParamSet),
    Shape(String, ParamSet),
    Translate(Float, Float, Float),
    Scale(Float, Float, Float),
    Rotate(Float, Float, Float, Float),
    Texture(
        String, // name
        String, // type
        String, // texname
        ParamSet,
    ),
    Unhandled(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    pub directives: Vec<Directive>,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(quoted_name<&str>,
   map_res!(
       delimited!(tag!("\""), alphanumeric, tag!("\"")),
       str::from_utf8
   )
);

named!(
    alphanumeric_str<&str>,
    map_res!(alphanumeric, str::from_utf8)
);

fn bool(input: &[u8]) -> IResult<&[u8], bool> {
    flat_map!(
        input,
        recognize!(alt!(tag!("true") | tag!("false"))),
        parse_to!(bool)
    )
}

// number is a superset of nom::double! that includes '1' and '1.'
#[allow(clippy::cyclomatic_complexity)]
fn number(input: &[u8]) -> IResult<&[u8], Float> {
    flat_map!(
        input,
        recognize!(tuple!(
            opt!(alt!(tag!("+") | tag!("-"))),
            alt!(
                complete!(delimited!(opt!(digit), tag!("."), digit))
                    | complete!(preceded!(digit, tag!(".")))
                    | digit
            ),
            opt!(complete!(tuple!(
                alt!(tag!("e") | tag!("E")),
                opt!(alt!(tag!("+") | tag!("-"))),
                digit
            )))
        )),
        parse_to!(Float)
    )
}

fn integer(input: &[u8]) -> IResult<&[u8], i64> {
    flat_map!(
        input,
        recognize!(tuple!(opt!(alt!(tag!("+") | tag!("-"))), complete!(digit))),
        parse_to!(i64)
    )
}

fn strip_comment(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    // TODO(wathiede): handle "#" if necessary.
    let re = regex::bytes::Regex::new(r"#.*\n").unwrap();
    let res = re.replace_all(input, &b"\n"[..]);

    IResult::Done(&b""[..], Vec::from(res))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(param_set_item_values_bool<Value>,
    do_parse!(
        values: alt!(
            ws!(delimited!(tag!("["), many1!(bool), tag!("]")))
            | ws!(many1!(bool))
        ) >>
        (Value::Bool(ParamList(values)))
    )
);

named!(
    parse_point3f<Point3f>,
    ws!(do_parse!(
        x: number >> y: number >> z: number >> (Point3f { x, y, z })
    ))
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(param_set_item_values_point<Value>,
    ws!(
        do_parse!(
                tag!("[") >>
                points: many1!(parse_point3f) >>
                tag!("]") >>
            (Value::Point3f(points.into()))
        )
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(param_set_item_values_float<Value>,
    do_parse!(
        values: alt!(
            ws!(delimited!(tag!("["), many1!(number), tag!("]")))
            | ws!(many1!(number))
        ) >>
        (Value::Float(ParamList(values)))
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(param_set_item_values_rgb<Value>,
    do_parse!(
        values: alt!(
            ws!(delimited!(tag!("["), many1!(number), tag!("]")))
            | ws!(many1!(number))
        ) >>
        (Value::RGB(ParamList(values)))
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(param_set_item_values_blackbody<Value>,
    do_parse!(
        values: alt!(
            ws!(delimited!(tag!("["), many1!(number), tag!("]")))
            | ws!(many1!(number))
        ) >>
        (Value::Blackbody(ParamList(values)))
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(param_set_item_values_integer<Value>,
    do_parse!(
        values: alt!(
            ws!(delimited!(tag!("["), many1!(integer), tag!("]")))
            | ws!(many1!(integer))
        ) >>
        (Value::Int(ParamList(values)))
    )
);

named!(
    ascii<String>,
    map_res!(
        map_res!(
            delimited!(tag!("\""), take_until!("\""), tag!("\"")),
            str::from_utf8
        ),
        String::from_str
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(param_set_item_values_string<Value>,
    alt!(
        do_parse!(
            values: ws!(delimited!(tag!("["), many1!(ascii), tag!("]"))) >>
            (Value::String(ParamList(values)))
        ) |
        do_parse!(
            value: ws!(ascii) >>
            (Value::String(ParamList(vec![value])))
        )
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(param_set_item_values_texture<Value>,
    alt!(
        do_parse!(
            values: ws!(delimited!(tag!("["), many1!(ascii), tag!("]"))) >>
            (Value::Texture(ParamList(values)))
        ) |
        do_parse!(
            value: ws!(ascii) >>
            (Value::Texture(ParamList(vec![value])))
        )
    )
);

fn param_set_item_values<'a, 'b>(input: &'a [u8], psi_type: &'b [u8]) -> IResult<&'a [u8], Value> {
    match psi_type {
        b"bool" => param_set_item_values_bool(input),
        b"float" => param_set_item_values_float(input),
        b"integer" => param_set_item_values_integer(input),
        b"string" => param_set_item_values_string(input),
        b"point" => param_set_item_values_point(input),
        b"rgb" => param_set_item_values_rgb(input),
        b"texture" => param_set_item_values_texture(input),
        b"blackbody" => param_set_item_values_blackbody(input),
        _ => panic!(format!(
            "unhandled param_set_item {:?}",
            str::from_utf8(psi_type).unwrap()
        )),
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(
    param_set_item<ParamSetItem>,
    do_parse!(
        tag!("\"") >>
        typ: alphanumeric >>
        space >>
        name: map_res!(alphanumeric, str::from_utf8) >>
        tag!("\"") >>
        values: call!(param_set_item_values, typ) >>
        (ParamSetItem::new(name, &values))
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(param_set<Vec<ParamSetItem>>,
    many0!(param_set_item)
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(
    look_at<Directive>,
    ws!(
        do_parse!(
            tag!("LookAt") >>
            ex: number >>
            ey: number >>
            ez: number >>
            lx: number >>
            ly: number >>
            lz: number >>
            ux: number >>
            uy: number >>
            uz: number >>
            (Directive::LookAt(ex, ey, ez, lx, ly, lz, ux, uy, uz))
        )
    )
);

named!(
    world_begin<Directive>,
    ws!(do_parse!(tag!("WorldBegin") >> (Directive::WorldBegin)))
);
named!(
    world_end<Directive>,
    ws!(do_parse!(tag!("WorldEnd") >> (Directive::WorldEnd)))
);

/// $name is the nom parser created, and $tag is the tag to look for and the Directive type
/// returned.
macro_rules! directive_param_set {
    ($name:tt, $tag:tt) => {
        named!(
            $name<Directive>,
            ws!(do_parse!(
                tag!(stringify!($tag))
                    >> name: quoted_name
                    >> ps: param_set
                    >> (Directive::$tag(name.into(), ps.into()))
            ))
        );
    };
}

directive_param_set!(sampler, Sampler);
directive_param_set!(integrator, Integrator);
directive_param_set!(film, Film);
directive_param_set!(light_source, LightSource);
directive_param_set!(material, Material);
directive_param_set!(shape, Shape);
directive_param_set!(camera, Camera);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(
    scale<Directive>,
    ws!(
        do_parse!(
            tag!("Scale") >>
            x: number >>
            y: number >>
            z: number >>
            (Directive::Scale(x, y, z))
        )
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(
    rotate<Directive>,
    ws!(
        do_parse!(
            tag!("Rotate") >>
            angle: number >>
            x: number >>
            y: number >>
            z: number >>
            (Directive::Rotate(angle, x, y, z))
        )
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(
    translate<Directive>,
    ws!(
        do_parse!(
            tag!("Translate") >>
            x: number >>
            y: number >>
            z: number >>
            (Directive::Translate(x, y, z))
        )
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(
    texture<Directive>,
    ws!(
        do_parse!(
            tag!("Texture") >>
            name: quoted_name >>
            typ: quoted_name >>
            class: quoted_name >>
            ps: param_set >>
            (Directive::Texture(name.into(), typ.into(), class.into(), ps.into()))
        )
    )
);

named!(
    attribute_begin<Directive>,
    ws!(do_parse!(
        tag!("AttributeBegin") >> (Directive::AttributeBegin)
    ))
);

named!(
    attribute_end<Directive>,
    ws!(do_parse!(tag!("AttributeEnd") >> (Directive::AttributeEnd)))
);

named!(
    unhandled<Directive>,
    ws!(do_parse!(
        statement: alphanumeric_str >> (Directive::Unhandled(statement.into()))
    ))
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(
    directives<Vec<Directive>>,
    dbg_dmp!(many1!(
        alt!(
            film
            | shape
            | scale
            | rotate
            | camera
            | texture
            | sampler
            | look_at
            | material
            | translate
            | world_end
            | integrator
            | world_begin
            | light_source
            | attribute_end
            | attribute_begin
            | unhandled
        )
    )
    )
);

pub fn parse_scene(input: &[u8]) -> Result<Scene, Error> {
    match strip_comment(input) {
        // TODO(wathiede): Implement From for IResult and use ? operator here.
        IResult::Done(_, i) => match parse_scene_macro(&i) {
            IResult::Done(_, scene) => Ok(scene),
            IResult::Error(e) => Err(Error::NomError(e)),
            IResult::Incomplete(n) => Err(Error::NomIncomplete(n)),
        },
        IResult::Error(e) => Err(Error::NomError(e)),
        IResult::Incomplete(n) => Err(Error::NomIncomplete(n)),
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(
    parse_scene_macro<Scene>,
    ws!(
        do_parse!(
            directives: directives >>
            (Scene{directives})
        )
    )
);

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use nom::IResult;

    use super::*;

    #[allow(dead_code)]
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
            let ref res = param_set_item_values_float(&input);
            assert_eq!(
                res,
                &IResult::Done(&b""[..], Value::Float(ParamList(vec![1., 2., 3.])))
            );
        };
    }

    #[test]
    fn test_param_set_item_values_point() {
        let input = &b"[.1 .2 .3  .4 .5 .6]\n"[..];
        let ref res = param_set_item_values_point(&input);
        assert_eq!(
            res,
            &IResult::Done(
                &b""[..],
                Value::Point3f(
                    vec![
                        Point3f {
                            x: 0.1,
                            y: 0.2,
                            z: 0.3,
                        },
                        Point3f {
                            x: 0.4,
                            y: 0.5,
                            z: 0.6,
                        },
                    ]
                    .into()
                )
            )
        );
    }

    #[test]
    fn test_param_set_item_values_float() {
        let input = &b"[.4 .45 .5]\n"[..];
        let ref res = param_set_item_values_float(&input);
        assert_eq!(
            res,
            &IResult::Done(&b""[..], Value::Float(ParamList(vec![0.4, 0.45, 0.5])))
        );

        let input = &b"1 2. -3.0"[..];
        let ref res = param_set_item_values_float(input);
        assert_eq!(
            res,
            &IResult::Done(&b""[..], Value::Float(ParamList(vec![1., 2., -3.])))
        );

        let input = &b"[  1 2 3]"[..];
        let ref res = param_set_item_values_float(input);
        assert_eq!(
            res,
            &IResult::Done(&b""[..], Value::Float(ParamList(vec![1., 2., 3.])))
        );
    }

    #[test]
    fn test_param_set_item_values_integer() {
        let input = &b"[-1 2 3]\n"[..];
        let ref res = param_set_item_values_integer(&input);
        assert_eq!(
            res,
            &IResult::Done(&b""[..], Value::Int(ParamList(vec![-1, 2, 3])))
        );

        let input = &b"[400]\n"[..];
        let ref res = param_set_item_values_integer(&input);
        assert_eq!(
            res,
            &IResult::Done(&b""[..], Value::Int(ParamList(vec![400])))
        );
    }

    #[test]
    fn test_param_set_item_values_string() {
        let input = &b"[\"foo\"]\n"[..];
        let ref res = param_set_item_values_string(&input);
        assert_eq!(
            res,
            &IResult::Done(&b""[..], Value::String(ParamList(vec!["foo".to_owned()])))
        );

        let input = &b"\"foo\"\n"[..];
        let ref res = param_set_item_values_string(&input);
        assert_eq!(
            res,
            &IResult::Done(&b""[..], Value::String(ParamList(vec!["foo".to_owned()])))
        );
    }

    #[test]
    fn test_param_set_item_values_texture() {
        let input = &b"[\"foo\"]\n"[..];
        let ref res = param_set_item_values_texture(&input);
        assert_eq!(
            res,
            &IResult::Done(&b""[..], Value::Texture(ParamList(vec!["foo".to_owned()])))
        );

        let input = &b"\"foo\"\n"[..];
        let ref res = param_set_item_values_texture(&input);
        assert_eq!(
            res,
            &IResult::Done(&b""[..], Value::Texture(ParamList(vec!["foo".to_owned()])))
        );
    }

    #[test]
    fn test_param_set_item_float() {
        let input = &b"\"float foo\" [ 0 1 2 ]"[..];
        let ref res = param_set_item(input);
        assert_eq!(
            res,
            &IResult::Done(
                &b""[..],
                ParamSetItem::new("foo", &Value::Float(ParamList(vec![0., 1., 2.])))
            )
        );
    }

    #[test]
    fn test_param_set_item_integer() {
        let input = &b"\"integer foo\" [400]"[..];
        let ref res = param_set_item(input);
        assert_eq!(
            res,
            &IResult::Done(
                &b""[..],
                ParamSetItem::new("foo", &Value::Int(ParamList(vec![400])))
            )
        );
    }

    #[test]
    fn test_param_set_item_bool() {
        let input = &b"\"bool foo\" [true false true false]"[..];
        let ref res = param_set_item(input);
        assert_eq!(
            res,
            &IResult::Done(
                &b""[..],
                ParamSetItem::new(
                    "foo",
                    &Value::Bool(ParamList(vec![true, false, true, false]))
                )
            )
        );
    }

    #[test]
    fn test_param_set_item_multi() {
        let input = &b"\"bool foo\" [true false true false]
\"integer bar\" [ 0 1 2 ]
"[..];
        let ref res = param_set(input);
        assert_eq!(
            res,
            &IResult::Done(
                &b""[..],
                vec![
                    ParamSetItem::new(
                        "foo",
                        &Value::Bool(ParamList(vec![true, false, true, false])),
                    ),
                    ParamSetItem::new("bar", &Value::Int(ParamList(vec![0, 1, 2]))),
                ]
            )
        );

        let input = &b"\"integer xresolution\" [400] \"integer yresolution\" [200]
\"string filename\" \"simple.png\"
"[..];
        let ref res = param_set(input);
        assert_eq!(
            res,
            &IResult::Done(
                &b""[..],
                vec![
                    ParamSetItem::new("xresolution", &Value::Int(ParamList(vec![400]))),
                    ParamSetItem::new("yresolution", &Value::Int(ParamList(vec![200]))),
                    ParamSetItem::new(
                        "filename",
                        &Value::String(ParamList(vec!["simple.png".to_owned()])),
                    ),
                ]
            )
        );
        let input = &b"\"string filename\" \"simple.png\"
\"integer xresolution\" [400] \"integer yresolution\" [200]"[..];
        let ref res = param_set(input);
        assert_eq!(
            res,
            &IResult::Done(
                &b""[..],
                vec![
                    ParamSetItem::new(
                        "filename",
                        &Value::String(ParamList(vec!["simple.png".to_owned()])),
                    ),
                    ParamSetItem::new("xresolution", &Value::Int(ParamList(vec![400]))),
                    ParamSetItem::new("yresolution", &Value::Int(ParamList(vec![200]))),
                ]
            )
        );
    }

    #[test]
    fn test_look_at() {
        let input = &b"LookAt 3 4 1.5  # eye
.5 .5 0  # look at point
0 0 1    # up vector
"[..];
        if let IResult::Done(_, input) = strip_comment(input) {
            let ref mut res = look_at(&input);
            assert_eq!(
                res,
                &IResult::Done(
                    &b""[..],
                    Directive::LookAt(3., 4., 1.5, 0.5, 0.5, 0., 0., 0., 1.)
                )
            );
        };
    }

    #[test]
    fn test_camera() {
        let input = &b"Camera \"perspective\" \"float fov\" 45"[..];
        if let IResult::Done(_, input) = strip_comment(input) {
            let ref mut res = camera(&input);
            assert_eq!(
                res,
                &IResult::Done(
                    &b""[..],
                    Directive::Camera(
                        "perspective".into(),
                        vec![ParamSetItem::new("fov", &Value::Float(vec![45.].into()))].into()
                    ),
                )
            );
        };
    }

    #[test]
    fn test_sampler() {
        let input = &b"Sampler \"halton\" \"integer pixelsamples\" 128"[..];
        if let IResult::Done(_, input) = strip_comment(input) {
            let ref mut res = sampler(&input);
            assert_eq!(
                res,
                &IResult::Done(
                    &b""[..],
                    Directive::Sampler(
                        "halton".into(),
                        vec![ParamSetItem::new(
                            "pixelsamples",
                            &Value::Int(vec![128].into()),
                        )]
                        .into()
                    )
                )
            );
        };
    }

    #[test]
    fn test_integrator() {
        let input = &b"Integrator \"path\""[..];
        if let IResult::Done(_, input) = strip_comment(input) {
            let ref mut res = integrator(&input);
            assert_eq!(
                res,
                &IResult::Done(
                    &b""[..],
                    Directive::Integrator("path".into(), vec![].into())
                )
            );
        };
    }

    #[test]
    fn test_film() {
        let input = &b"Film \"image\" \"string filename\" \"simple.png\"
\"integer xresolution\" [400] \"integer yresolution\" [200]"[..];
        let ref mut res = film(&input);
        assert_eq!(
            res,
            &IResult::Done(
                &b""[..],
                Directive::Film(
                    "image".into(),
                    vec![
                        ParamSetItem::new(
                            "filename",
                            &Value::String(vec!["simple.png".to_owned()].into()),
                        ),
                        ParamSetItem::new("xresolution", &Value::Int(vec![400].into())),
                        ParamSetItem::new("yresolution", &Value::Int(vec![200].into())),
                    ]
                    .into()
                )
            )
        );
    }

    #[test]
    fn test_attribute() {
        let input =
            &b"AttributeBegin\n  LightSource \"infinite\" \"rgb L\" [.4 .45 .5]\nAttributeEnd"[..];
        let ref mut res = directives(&input);
        assert_eq!(
            res,
            &IResult::Done(
                &b""[..],
                vec![
                    Directive::AttributeBegin,
                    Directive::LightSource(
                        "infinite".into(),
                        vec![ParamSetItem::new(
                            "L",
                            &Value::RGB(ParamList(vec![0.4, 0.45, 0.5])),
                        )]
                        .into(),
                    ),
                    Directive::AttributeEnd,
                ]
            )
        );

        let input = &b"AttributeBegin\n  Material \"mirror\"\nAttributeEnd"[..];
        let ref mut res = directives(&input);
        assert_eq!(
            res,
            &IResult::Done(
                &b""[..],
                vec![
                    Directive::AttributeBegin,
                    Directive::Material("mirror".into(), vec![].into()),
                    Directive::AttributeEnd,
                ]
            )
        );

        let input = &b"AttributeBegin\n  Shape \"sphere\" \"float radius\" 1\nAttributeEnd"[..];
        let ref mut res = directives(&input);
        assert_eq!(
            res,
            &IResult::Done(
                &b""[..],
                vec![
                    Directive::AttributeBegin,
                    Directive::Shape(
                        "sphere".into(),
                        vec![ParamSetItem::new("radius", &Value::Float(vec![1.].into()))].into(),
                    ),
                    Directive::AttributeEnd,
                ]
                .into(),
            )
        );

        let input = &b"AttributeBegin\n  Material \"mirror\"\n  Shape \"sphere\" \"float radius\" 1\nAttributeEnd"[..];
        let ref mut res = directives(&input);
        assert_eq!(
            res,
            &IResult::Done(
                &b""[..],
                vec![
                    Directive::AttributeBegin,
                    Directive::Material("mirror".into(), vec![].into()),
                    Directive::Shape(
                        "sphere".into(),
                        vec![ParamSetItem::new("radius", &Value::Float(vec![1.].into()))].into(),
                    ),
                    Directive::AttributeEnd,
                ]
                .into(),
            )
        );
    }

    #[test]
    fn test_light_source() {
        let input = &b"LightSource \"infinite\" \"rgb L\" [.4 .45 .5]"[..];
        let ref mut res = light_source(&input);
        assert_eq!(
            res,
            &IResult::Done(
                &b""[..],
                Directive::LightSource(
                    "infinite".into(),
                    vec![ParamSetItem::new(
                        "L",
                        &Value::RGB(ParamList(vec![0.4, 0.45, 0.5])),
                    )]
                    .into()
                )
            )
        );
    }

    #[test]
    fn test_material() {
        let input = &b"Material \"mirror\""[..];
        let ref mut res = material(&input);
        assert_eq!(
            res,
            &IResult::Done(
                &b""[..],
                Directive::Material("mirror".into(), vec![].into()),
            )
        );
    }

    #[test]
    fn test_shape() {
        let input = &b"Shape \"sphere\" \"float radius\" 1"[..];
        let ref mut res = shape(&input);
        assert_eq!(
            res,
            &IResult::Done(
                &b""[..],
                Directive::Shape(
                    "sphere".into(),
                    vec![ParamSetItem::new("radius", &Value::Float(vec![1.].into()))].into(),
                ),
            )
        );
    }

    #[test]
    fn test_translate() {
        let input = &b"Translate 0 0 -1"[..];
        let ref mut res = translate(&input);
        assert_eq!(
            res,
            &IResult::Done(&b""[..], Directive::Translate(0., 0., -1.))
        );
    }

    #[test]
    fn test_scale() {
        let input = &b"Scale 0 0 -1"[..];
        let ref mut res = scale(&input);
        assert_eq!(res, &IResult::Done(&b""[..], Directive::Scale(0., 0., -1.)));
    }

    #[test]
    fn test_rotate() {
        let input = &b"Rotate 30 0 0 -1"[..];
        let ref mut res = rotate(&input);
        assert_eq!(
            res,
            &IResult::Done(&b""[..], Directive::Rotate(30., 0., 0., -1.))
        );
    }

    #[test]
    fn test_parse_scene() {
        let input = include_bytes!("testdata/scene1.pbrt");
        let res = parse_scene(&input[..]);
        let want = Scene {
            directives: vec![
                Directive::LookAt(3., 4., 1.5, 0.5, 0.5, 0., 0., 0., 1.),
                Directive::Camera(
                    "perspective".into(),
                    vec![ParamSetItem::new("fov", &Value::Float(vec![45.].into()))].into(),
                ),
                Directive::Sampler(
                    "halton".into(),
                    vec![ParamSetItem::new(
                        "pixelsamples",
                        &Value::Int(vec![128].into()),
                    )]
                    .into(),
                ),
                Directive::Integrator("path".into(), vec![].into()),
                Directive::Film(
                    "image".into(),
                    vec![
                        ParamSetItem::new(
                            "filename",
                            &Value::String(vec!["simple.png".to_owned()].into()),
                        ),
                        ParamSetItem::new("xresolution", &Value::Int(vec![400].into())),
                        ParamSetItem::new("yresolution", &Value::Int(vec![300].into())),
                    ]
                    .into(),
                ),
                Directive::WorldBegin,
                Directive::LightSource(
                    "infinite".into(),
                    vec![ParamSetItem::new(
                        "L",
                        &Value::RGB(vec![0.4, 0.45, 0.5].into()),
                    )]
                    .into(),
                ),
                Directive::LightSource(
                    "distant".into(),
                    vec![
                        ParamSetItem::new(
                            "from",
                            &Value::Point3f(
                                vec![Point3f {
                                    x: -30.,
                                    y: 40.,
                                    z: 100.,
                                }]
                                .into(),
                            ),
                        ),
                        ParamSetItem::new("L", &Value::Blackbody(vec![3000., 1.5].into())),
                    ]
                    .into(),
                ),
                Directive::AttributeBegin,
                Directive::Material("mirror".into(), vec![].into()),
                Directive::Shape(
                    "sphere".into(),
                    vec![ParamSetItem::new("radius", &Value::Float(vec![1.].into()))].into(),
                ),
                Directive::AttributeEnd,
                Directive::AttributeBegin,
                Directive::Texture(
                    "checks".into(),
                    "spectrum".into(),
                    "checkerboard".into(),
                    vec![
                        ParamSetItem::new("uscale", &Value::Float(vec![8.].into())),
                        ParamSetItem::new("vscale", &Value::Float(vec![8.].into())),
                        ParamSetItem::new("tex1", &Value::RGB(ParamList(vec![0.1, 0.1, 0.1]))),
                        ParamSetItem::new("tex2", &Value::RGB(ParamList(vec![0.8, 0.8, 0.8]))),
                    ]
                    .into(),
                ),
                Directive::Material(
                    "matte".into(),
                    vec![ParamSetItem::new(
                        "Kd",
                        &Value::Texture(vec!["checks".into()].into()),
                    )]
                    .into(),
                ),
                Directive::Translate(0., 0., -1.),
                Directive::Shape(
                    "trianglemesh".into(),
                    vec![
                        ParamSetItem::new("indices", &Value::Int(vec![0, 1, 2, 0, 2, 3].into())),
                        ParamSetItem::new(
                            "P",
                            &Value::Point3f(
                                vec![
                                    Point3f {
                                        x: -20.,
                                        y: -20.,
                                        z: 0.,
                                    },
                                    Point3f {
                                        x: 20.,
                                        y: -20.,
                                        z: 0.,
                                    },
                                    Point3f {
                                        x: 20.,
                                        y: 20.,
                                        z: 0.,
                                    },
                                    Point3f {
                                        x: -20.,
                                        y: 20.,
                                        z: 0.,
                                    },
                                ]
                                .into(),
                            ),
                        ),
                        ParamSetItem::new(
                            "st",
                            &Value::Float(vec![0., 0., 1., 0., 1., 1., 0., 1.].into()),
                        ),
                    ]
                    .into(),
                ),
                Directive::AttributeEnd,
                Directive::WorldEnd,
            ],
        };
        assert_eq!(res.unwrap(), want);
    }
}
/*
Shape "trianglemesh"
    "integer indices" [0 1 2 0 2 3]
    "point P" [ -20 -20 0   20 -20 0   20 20 0   -20 20 0 ]
    "float st" [ 0 0   1 0    1 1   0 1 ]

 */
