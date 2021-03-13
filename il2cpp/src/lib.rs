use anyhow::bail;
use nom::bytes::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::IResult;

fn parse_plain_ident(i: &str) -> IResult<&str, &str> {
    take_while1(|c: char| {
        c.is_alphanumeric() || c == '!' || c == '`' || c == '_' || c == '&' || c == '|' || c == '-'
    })(i)
}

// example: Foo<Bar>
fn parse_namespace(i: &str) -> IResult<&str, &str> {
    let (i, ident) = parse_plain_ident(i)?;
    let (i, _generic) = opt(parse_generic_param)(i)?;

    Ok((i, ident))
}

fn parse_namespaces<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
    if let Ok((i, _v)) = tag::<_, _, nom::error::Error<&'a str>>(".cctor")(i) {
        return Ok((i, _v));
    }
    if let Ok((i, _v)) = tag::<_, _, nom::error::Error<&'a str>>(".ctor")(i) {
        return Ok((i, _v));
    }

    let (i, idents) = separated_list1(tag("."), parse_namespace)(i)?;

    Ok((i, idents[0]))
}

fn parse_mysterious_tags<'a>(i: &'a str) -> IResult<&'a str, ()> {
    // TODO: U26, U2DGetEnumerator
    let (i, _) = opt(tag("U26"))(i)?;
    let (i, _) = opt(tag("U2DGetEnumerator"))(i)?;
    let (i, _) = opt(tag("-GetEnumerator"))(i)?;

    Ok((i, ()))
}

fn parse_generic_param<'a>(i: &'a str) -> IResult<&'a str, Vec<Type<'a>>> {
    let (i, _) = tag("<")(i)?;
    let (i, idents) = separated_list0(tag(","), parse_type_maybe_tuple)(i)?;
    let (i, _) = tag(">")(i)?;

    Ok((i, idents))
}

fn parse_arguments<'a>(i: &'a str) -> IResult<&'a str, Vec<Type<'a>>> {
    let (i, _) = tag("(")(i)?;
    let (i, idents) = separated_list0(tag(","), parse_type)(i)?;
    let (i, _) = tag(")")(i)?;

    Ok((i, idents))
}

#[derive(Debug, PartialEq, Eq)]
struct EmbeddedPrefix<'a> {
    generic: Option<Vec<Type<'a>>>,
    ident: &'a str,
}

fn parse_type0<'a>(i: &'a str) -> IResult<&'a str, Type<'a>> {
    // generic prefix
    let (i, _gen_prefix) = opt(parse_generic_param)(i)?;

    let (i, ident) = parse_namespaces(i)?;

    Ok((i, Type { ident }))
}

fn parse_array_notation(i: &str) -> IResult<&str, ()> {
    let (i, _) = tag("[")(i)?;
    let (i, _) = separated_list0(tag(","), opt(tag("0...")))(i)?;
    let (i, _) = tag("]")(i)?;

    Ok((i, ()))
}

fn parse_type<'a>(i: &'a str) -> IResult<&'a str, Type<'a>> {
    let (i, mut _type_path) = separated_list1(tag("/"), parse_type0)(i)?;

    // let (i, _type_gen) = opt(parse_generic_param)(i)?;
    let (i, _arr) = many0(parse_array_notation)(i)?;
    let (i, _pointer) = opt(tag("*"))(i)?;

    let (i, _) = parse_mysterious_tags(i)?;

    let (i, _reference) = opt(tag("&"))(i)?;
    let (i, _nullable) = opt(tag("?"))(i)?;

    Ok((i, _type_path.pop().unwrap()))
}

fn parse_type_maybe_tuple<'a>(i: &'a str) -> IResult<&'a str, Type<'a>> {
    if i.len() > 0 && i.as_bytes()[0] == b'(' {
        let (i, _) = tag("(")(i)?;

        let (i, mut _types) = separated_list1(tag(","), parse_type)(i)?;

        let (i, _) = tag(")")(i)?;

        Ok((i, _types.pop().unwrap()))
    } else {
        parse_type(i)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Type<'a> {
    ident: &'a str,
}

#[derive(Debug)]
pub struct Signature<'a> {
    ret_ty: Type<'a>,
}

fn parse_method_ident_normal<'a>(i: &'a str) -> IResult<&str, ()> {
    let (i, _method_name) = parse_type(i)?;
    let (i, _method_gen) = opt(parse_generic_param)(i)?;

    Ok((i, ()))
}

fn parse_method_part<'a>(i: &'a str) -> IResult<&str, ()> {
    let (i, _) = tag("::")(i)?;

    let (i, _method_ident) = parse_method_ident_normal(i)?;

    let (i, _found) = opt(tag("."))(i)?;

    let i = if _found.is_some() {
        let (i, _extra_method_ident) = parse_namespaces(i)?;
        i
    } else {
        i
    };

    let (i, _args) = opt(parse_arguments)(i)?;

    Ok((i, ()))
}

fn parse_signature0<'a>(i: &'a str) -> IResult<&str, Signature<'a>> {
    let (i, _return) = parse_type(i)?;

    if i.len() == 0 {
        return Ok((i, Signature { ret_ty: _return }));
    }

    let (i, _) = tag(" ")(i)?;
    let (i, _name) = parse_type(i)?;

    let (i, _method_part) = opt(parse_method_part)(i)?;

    Ok((i, Signature { ret_ty: _return }))
}

pub fn parse_signature<'a>(s: &'a str) -> anyhow::Result<Signature<'a>> {
    match all_consuming(parse_signature0)(s) {
        Ok((_, out)) => Ok(out),
        Err(e) => {
            bail!("failed to parse: {:?}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_plain_ident() {
        let cases = &[
            ("hello", "", "hello"),
            ("hello<>", "<>", "hello"),
            ("!!0 BeardedManStudios", " BeardedManStudios", "!!0"),
            ("Foo.Bar", ".Bar", "Foo"),
            ("Foo.Bar/TypeHolder", ".Bar/TypeHolder", "Foo"),
            ("Foo/TypeHolder", "/TypeHolder", "Foo"),
        ];
        for case in cases {
            assert_eq!(parse_plain_ident(case.0), Ok((case.1, case.2)));
        }
    }

    #[test]
    fn test_namepsace() {
        let cases = &[
            //
            ("Foo.Bar", ".Bar", "Foo"),
            ("Foo<Bar>.Bar", ".Bar", "Foo"),
        ];
        for case in cases {
            assert_eq!(parse_namespace(case.0), Ok((case.1, case.2)));
        }
    }

    /*
    #[test]
    fn test_parse_type0() {
        let cases = vec![
            //
            ("<>c__", "", "c__"),
            ("<>c__<>", "<>", "c__"),
            ("<Hello>c__<>", "<>", "c__"),
        ];

        for case in cases {
            assert_eq!(parse_type0(case.0), Ok((case.1, Type { ident: case.2 })));
        }
    }
    */

    #[test]
    fn test_parse_type() {
        let cases = vec![
            //
            "hello/<>c__",
            "hello/<>c__/Hello",
            "I/<>c__DisplayClass4_0/<<Baz>g__Task|0>d",
            "I.Foo.API.Bar/<>c__DisplayClass4_0/<<Baz>g__Task|0>d",
            "AlmostEngine.Screenshot.ScreenshotManager",
            "Assets.FMLib.Extensions.GameObjectExtensions/<>c__DisplayClass7_0`1<System.Object>",
            "BestHTTP.Futures.FutureCallback`1<System.Object>[]",
            "Foo/<Bar-Bax>d__8<Boo>",
        ];
        for case in cases {
            assert_eq!(parse_type(case).unwrap().0, "");
        }
        //
    }

    #[test]
    fn test_parse_type_maybe_tuple() {
        let cases = vec![
            //
            "hello/<>c__",
            "hello/<>c__/Hello",
            "(hello/<>c__/Hello,hello/<>c__/Hello)",
        ];

        for case in cases {
            assert_eq!(parse_type_maybe_tuple(case).unwrap().0, "");
        }
    }

    #[test]
    fn test_parse_arguments() {
        let cases = vec![
            "()",
            "(Foo,Bar)",
            "(System.Reflection.MethodInfo[],System.Type,System.Collections.Generic.List`1<Sirenix.Serialization.BaseFormatter`1_SerializationCallback<T>>U26)",
        ];
        for case in cases {
            assert_eq!(parse_arguments(case).unwrap().0, "");
        }
    }

    #[test]
    fn test_parse_array_notation() {
        let cases = vec![
            //
            "[]",
            "[0...,0...]",
            "[,]",
        ];
        for case in cases {
            assert_eq!(parse_array_notation(case).unwrap().0, "");
        }
    }

    #[test]
    fn test_parse_method_part() {
        let cases = vec![
            //
            "::System.Linq.IOrderedEnumerable<TElement>.CreateOrderedEnumerable<System.Boolean>",
        ];
        for case in cases {
            assert_eq!(parse_method_part(case).unwrap().0, "");
        }
    }

    #[test]
    fn it_works() {
        let examples = [
            "System.Void Newtonsoft.Json.Converters.CustomCreationConverter`1<System.Object>::WriteJson(Newtonsoft.Json.JsonWriter,System.Object,Newtonsoft.Json.JsonSerializer)",
            "System.Collections.Generic.IEnumerator`1<com.adjust.sdk.JSONNode> com.adjust.sdk.JSONArray/<get_Childs>d__13::System.Collections.Generic.IEnumerable<com.adjust.sdk.JSONNode>.GetEnumerator()",
            "AlmostEngine.Screenshot.ScreenshotComposition AlmostEngine.Screenshot.ScreenshotManager::<UpdateCoroutine>b__24_3(AlmostEngine.Screenshot.ScreenshotBatch_ActiveItem)",
            "Sirenix.Serialization.BaseFormatter`1_SerializationCallback<T>[] Sirenix.Serialization.BaseFormatter`1<System.Collections.Generic.KeyValuePair`2<System.Object,System.Object>>::GetCallbacks(System.Reflection.MethodInfo[],System.Type,System.Collections.Generic.List`1<Sirenix.Serialization.BaseFormatter`1_SerializationCallback<T>>U26)",
            "System.Linq.IOrderedEnumerable`1<TElement> System.Linq.OrderedEnumerable`1<I.Foo.Baz.I>::System.Linq.IOrderedEnumerable<TElement>.CreateOrderedEnumerable<System.Boolean>(System.Func`2<TElement,TKey>,System.Collections.Generic.IComparer`1<TKey>,System.Boolean)",
        ];

        for example in &examples {
            parse_signature(example).unwrap();
        }
    }
}
