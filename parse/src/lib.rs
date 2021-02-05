use anyhow::bail;
use nom::bytes::complete::*;
use nom::character::complete::*;
use std::iter::Iterator;

pub type Result<T> = std::result::Result<T, anyhow::Error>;

pub struct UnityYamlFile<'a> {
    pub objects: Vec<UnityYamlObject<'a>>,
}

/// indent signature
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct IndentSig {
    indent: usize,
    array_elem: bool,
}
impl IndentSig {
    fn new(indent: usize, array_elem: bool) -> Self {
        Self { indent, array_elem }
    }

    fn empty() -> Self {
        Self::new(0, false)
    }

    fn iter_indent(&self) -> Self {
        if self.array_elem {
            Self {
                // TODO
                indent: self.indent + 2,
                array_elem: false,
            }
        } else {
            self.clone()
        }
    }
}

/// 첫 줄이 array면 array iterator입니다.
/// - a: <- next()
///   b: <- next()
/// - c: <- 안 보임
///
/// 첫 줄이 head면 object iterator입니다.
/// a:
/// - foo: <- next()
///   bar:
/// - baz: <- next()
///   foz:
/// 다른 예시
/// a:
///   foo: <- next()
///   bar: <- next()
#[derive(Clone, Debug)]
pub struct UnityYamlIter<'a> {
    indent: IndentSig,
    lines: Vec<Line<'a>>,

    /// parsing object
    parse_object: bool,
    cursor: usize,
}

impl<'a> UnityYamlIter<'a> {
    fn from_lines<'b>(lines: &'b [Line<'a>], parse_object: bool) -> Self {
        let indent = if lines.len() > 0 {
            if parse_object {
                lines[0].indent.iter_indent()
            } else {
                lines[0].indent.clone()
            }
        } else {
            IndentSig::empty()
        };
        let lines = lines.to_vec();

        Self {
            lines,
            indent,
            parse_object,
            cursor: 0,
        }
    }
}

/// scope가 object root일 때, object를 차례로 반환합니다.
/// 예를 들어,
/// Foo: <- 첫 번째 줄
///   bar: <- 첫 번째
///   baz: <- 두 번째
impl<'a> Iterator for UnityYamlIter<'a> {
    type Item = UnityYaml<'a>;

    fn next<'b>(&'b mut self) -> Option<Self::Item> {
        if self.cursor == self.lines.len() {
            return None;
        }

        let line = self.lines[self.cursor].clone();
        let start = self.cursor;

        self.cursor += 1;
        while self.cursor < self.lines.len() {
            let line = &self.lines[self.cursor];
            if line.indent <= self.indent {
                break;
            }
            self.cursor += 1;
        }

        let children = if self.parse_object {
            &self.lines[start + 1..self.cursor]
        } else {
            &self.lines[start..self.cursor]
        };

        let child_parse_object =
            if self.parse_object && children.len() > 0 && children[0].indent.array_elem {
                false
            } else {
                true
            };

        Some(UnityYaml {
            cur: line,
            children: UnityYamlIter::from_lines(children, child_parse_object),
        })
    }
}

#[derive(Debug)]
pub struct UnityYaml<'a> {
    cur: Line<'a>,
    children: UnityYamlIter<'a>,
}
impl<'a> UnityYaml<'a> {
    pub fn key<'b>(&'b self) -> Option<&'a str> {
        self.cur.key
    }

    pub fn as_value<'b>(&'b self) -> Option<&'b LineValue<'a>> {
        Some(&self.cur.value)
    }

    pub fn as_str<'b>(&'b self) -> Option<&'a str> {
        match self.cur.value {
            LineValue::Str(ref s) => Some(s),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        self.as_str()?.parse().ok()
    }

    pub fn is_object(&self) -> bool {
        self.children.parse_object
    }

    pub fn is_array(&self) -> bool {
        self.children
            .lines
            .get(0)
            .map(|line| line.indent.array_elem)
            .unwrap_or(false)
    }

    pub fn iter<'b>(&'b self) -> UnityYamlIter<'a> {
        self.children.clone()
    }
}

#[derive(Debug)]
pub struct UnityYamlObject<'a> {
    pub header: ObjectHeader,
    lines: Vec<Line<'a>>,
}

impl<'a> UnityYamlObject<'a> {
    pub fn iter<'b>(&'b self) -> UnityYamlIter<'a> {
        UnityYamlIter::from_lines(&self.lines, true)
    }
}

#[allow(unused)]
pub fn debug_print(iter: UnityYamlIter, depth: usize) {
    for item in iter {
        println!("#{} item={:?}", depth, item.cur);
        debug_print(item.children, depth + 1);
    }
}

#[allow(unused)]
pub fn debug_print_item(item: UnityYaml, depth: usize) {
    println!("#{} item={:?}", depth, item.cur);
    for child in item.iter() {
        debug_print_item(child, depth + 1);
    }
}

/// yaml 파일에 대한 정보. prefab/scene 등이 이에 해당합니다.
/// 내부에서 object tree 구조를 가지고 있습니다.
/// `--- !u!1001 &33075763` 에서, object_id=101, file_id=33075763
#[derive(Debug, Clone)]
pub struct ObjectHeader {
    /// file-local object id. 따로 쓰는 것 같지는 않지만 일단 파싱합니다
    pub object_id: u64,
    /// fileID
    pub file_id: i64,
    /// Tag. stripped?
    pub stripped: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineValue<'a> {
    None,
    Str(&'a str),
    Map(Vec<(&'a str, &'a str)>),
}

#[derive(Debug, Clone)]
pub struct Line<'a> {
    indent: IndentSig,
    key: Option<&'a str>,
    value: LineValue<'a>,
}

impl<'a> Line<'a> {
    #[allow(unused)]
    fn new(indent: IndentSig, key: Option<&'a str>, value: LineValue<'a>) -> Self {
        Self { indent, key, value }
    }
}

fn parse_object_header(i: &str) -> nom::IResult<&str, ObjectHeader> {
    let (i, (_, _, tag1, _, tag2, _, tag3, _)) = nom::sequence::tuple((
        tag("--- "),
        tag("!u!"),
        digit1,
        tag(" &"),
        digit1,
        space0,
        alphanumeric0,
        newline,
    ))(i)?;

    let object_id = tag1.parse().unwrap();
    let file_id = tag2.parse().unwrap();

    let stripped = if tag3 == "stripped" {
        true
    } else if tag3.is_empty() {
        false
    } else {
        todo!()
    };

    Ok((
        i,
        ObjectHeader {
            object_id,
            file_id,
            stripped,
        },
    ))
}

fn key_char(chr: char) -> bool {
    chr != ':' && chr != '\r' && chr != '\n'
}

fn embed_value_char(chr: char) -> bool {
    chr != ',' && chr != '}' && chr != ' '
}

fn parse_indent_sig(i: &str) -> nom::IResult<&str, IndentSig> {
    let (i, cur_indent) = space0(i)?;
    let (i, array_elem) = match tag::<_, _, nom::error::Error<_>>("- ")(i) {
        Ok((i, _)) => (i, true),
        Err(_) => (i, false),
    };
    Ok((i, IndentSig::new(cur_indent.len(), array_elem)))
}

fn parse_yaml_embed_pair(i: &str) -> nom::IResult<&str, (&str, &str)> {
    let (i, (_, key, _, _, value)) = nom::sequence::tuple((
        take_while(|c| c == ',' || c == ' ' || c == '\r' || c == '\n'),
        take_while(key_char),
        tag(":"),
        space0,
        take_while(embed_value_char),
    ))(i)?;
    Ok((i, (key, value)))
}

fn parse_yaml_embed_value0(i: &str) -> nom::IResult<&str, LineValue> {
    // example
    // {a: 1, b: 2}\n
    let (i, (_, vec, _, _)) = nom::sequence::tuple((
        //
        tag("{"),
        nom::multi::many1(parse_yaml_embed_pair),
        tag("}"),
        newline,
    ))(i)?;
    Ok((i, LineValue::Map(vec)))
}

fn parse_yaml_str_value(i: &str) -> nom::IResult<&str, LineValue> {
    // example
    // foo bar baz - baz\n
    let (i, (value, _)) = nom::sequence::tuple((not_line_ending, newline))(i)?;
    Ok((i, LineValue::Str(value)))
}

fn parse_yaml_embed_line(i: &str) -> nom::IResult<&str, (Option<&str>, LineValue)> {
    // example
    // {a: 1, b: 2}\n
    let (i, value) = parse_yaml_embed_value0(i)?;
    Ok((i, (None, value)))
}

fn parse_yaml_valueonly_line(i: &str) -> nom::IResult<&str, (Option<&str>, LineValue)> {
    // example
    // foobarbaz\n
    let (i, (value, _)) = nom::sequence::tuple((take_while(key_char), newline))(i)?;
    Ok((i, (Some(value), LineValue::None)))
}

fn parse_yaml_keyvalue_line(i: &str) -> nom::IResult<&str, (Option<&str>, LineValue)> {
    // example
    // foobarbaz: hello world\n
    // foobarbaz: {x: 1, y: 2}\n
    let (i, (key, _, _, value)) = nom::sequence::tuple((
        take_while(key_char),
        tag(":"),
        space0,
        nom::branch::alt((parse_yaml_embed_value0, parse_yaml_str_value)),
    ))(i)?;
    Ok((i, (Some(key), value)))
}

fn parse_yaml_like(i: &str) -> nom::IResult<&str, Line> {
    use nom::error::*;

    if i.len() > 3 && &i[0..3] == "---" {
        return Err(nom::Err::Error(nom::error::Error::new(
            "looks like an object header",
            ErrorKind::Tag,
        )));
    }

    let (i, (indent, (key, value))) = nom::sequence::tuple((
        parse_indent_sig,
        nom::branch::alt((
            parse_yaml_embed_line,
            parse_yaml_keyvalue_line,
            parse_yaml_valueonly_line,
        )),
    ))(i)?;

    Ok((i, Line { indent, key, value }))
}

fn parse_yaml_object(i: &str) -> nom::IResult<&str, UnityYamlObject> {
    let (i, (header, lines)) =
        nom::sequence::tuple((parse_object_header, nom::multi::many1(parse_yaml_like)))(i)?;

    Ok((i, UnityYamlObject { header, lines }))
}

fn parse_file_header(i: &str) -> nom::IResult<&str, ()> {
    let (i, _) = nom::sequence::tuple((
        tag("%YAML 1.1"),
        newline,
        tag("%TAG !u! tag:unity3d.com,2011:"),
        newline,
    ))(i)?;
    Ok((i, ()))
}

fn parse_unity_yaml<'a>(i: &'a str) -> nom::IResult<&'a str, UnityYamlFile> {
    let (i, (_header, objects)) = nom::combinator::all_consuming(nom::sequence::tuple((
        parse_file_header,
        nom::multi::many1(parse_yaml_object),
    )))(i)?;

    Ok((i, UnityYamlFile { objects }))
}

pub fn parse_str<'a>(input: &'a str) -> Result<UnityYamlFile<'a>> {
    match parse_unity_yaml(input) {
        Ok((_remain, parsed)) => Ok(parsed),
        Err(_e) => {
            log::error!("{:?}", _e);
            bail!("parse error")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indent_ord_array() {
        let a = IndentSig::new(2, false);
        let b = IndentSig::new(2, true);
        assert!(a < b);
    }

    #[test]
    fn indent_ord() {
        let a = IndentSig::new(2, false);
        let b = IndentSig::new(4, false);
        assert!(a < b);
    }

    #[test]
    fn parse_line_test() {
        let input = &[
            //
            "      - 0.37482873\n",
            "  - hello\n",
            "  - hello: world\n",
            "  - {x: 1, y: 2}\n",
        ];

        for line in input {
            let res = parse_yaml_like(line);
            assert!(res.is_ok());
        }
    }

    #[test]
    fn parse_array_map() {
        let line = "  - {x: 1, y: 2}\n";
        let (_remain, parsed) = parse_yaml_like(line).unwrap();
        assert_eq!(_remain.len(), 0);
        assert_eq!(parsed.key, None);
        assert_eq!(parsed.value, LineValue::Map(vec![("x", "1"), ("y", "2")]));
    }

    #[test]
    fn parse_array_kv_map() {
        let line = "  - key: {x: 1, y: 2}\n";
        let (_remain, parsed) = parse_yaml_like(line).unwrap();
        assert_eq!(_remain.len(), 0);
        assert_eq!(parsed.key, Some("key"));
        assert_eq!(parsed.value, LineValue::Map(vec![("x", "1"), ("y", "2")]));
    }

    #[test]
    fn parse_yaml_keyvalue_line_newline_test() {
        let line = "foobarbaz: {x: 1,\n  y: 2}\n";
        let (_remain, (key, value)) = parse_yaml_keyvalue_line(line).unwrap();
        assert_eq!(_remain.len(), 0);
        assert_eq!(key, Some("foobarbaz"));
        assert_eq!(value, LineValue::Map(vec![("x", "1"), ("y", "2")]));
    }

    #[test]
    fn parse_yaml_keyvalue_line_test() {
        let input = &[
            "foobarbaz: hello world\n",
            "foobarbaz: {x: 1, y: 2}\n",
            "foobarbaz: {x: 1,\n  y: 2}\n",
            "foobar - baz: foo - bar\n",
        ];

        for line in input {
            assert!(parse_yaml_keyvalue_line(line).is_ok());
        }
    }

    #[test]
    fn parse_number_line() {
        let line = "      - 0.37482873\n";
        let res = parse_yaml_like(line);
        assert!(res.is_ok());
    }

    #[test]
    fn parse_array() {
        let input = r#"--- !u!123 &456
hello:
  - foo:
    bar:
    baz:
  - foo:
    bar:
    bzz:
"#;

        let (remain, parsed) = parse_yaml_object(&input).unwrap();
        assert_eq!(remain.len(), 0);

        assert_eq!(parsed.iter().count(), 1);

        let root = parsed.iter().next().unwrap();
        assert_eq!(root.iter().count(), 2);
        for item in root.iter() {
            assert_eq!(item.iter().count(), 3);
            for terminal in item.iter() {
                assert_eq!(terminal.iter().count(), 0);
            }
        }
    }

    #[test]
    fn parse_obj() {
        let input = r#"--- !u!1 &2
GameObject:
  m_ObjectHideFlags: 0
  m_CorrespondingSourceObject: {fileID: 0}
"#;

        let (remain, parsed) = parse_yaml_object(&input).unwrap();
        assert_eq!(remain.len(), 0);
    }

    #[test]
    fn parse_embed_value() {
        let line = "{x: -0}\n";
        let res = parse_yaml_embed_value0(line);
        assert!(res.is_ok());
    }

    #[test]
    fn parse_embed_value_multi() {
        let line = "{x: -0, y: -0, z: -0, w: 1}\n";
        let res = parse_yaml_embed_value0(line);
        assert!(res.is_ok());
    }

    #[test]
    fn iter_obj_test() {
        use LineValue::Str;
        let cur = Line::new(IndentSig::new(0, false), Some("bar"), Str(""));
        let lines = vec![
            Line::new(IndentSig::new(2, false), Some("bar2"), Str("baz")),
            Line::new(IndentSig::new(2, false), Some("bar3"), Str("baz")),
        ];

        let item = UnityYaml {
            cur,
            children: UnityYamlIter::from_lines(&lines, true),
        };

        assert_eq!(item.iter().count(), 2);
        for child in item.iter() {
            assert_eq!(child.iter().count(), 0);
        }
    }

    #[test]
    fn iter_obj_test2() {
        use LineValue::Str;
        let lines = vec![
            Line::new(IndentSig::new(4, false), Some("bar"), Str("baz")),
            Line::new(IndentSig::new(4, false), Some("bar2"), Str("baz")),
            Line::new(IndentSig::new(4, false), Some("bar3"), Str("baz")),
        ];

        let iter = UnityYamlIter::from_lines(&lines, true);
        assert_eq!(iter.clone().count(), 3);
        for item in iter {
            assert_eq!(item.iter().count(), 0);
        }
    }

    #[test]
    fn iter_arr_test() {
        use LineValue::Str;

        let lines = vec![
            Line::new(IndentSig::new(2, false), Some("foo"), Str("")),
            Line::new(IndentSig::new(2, true), Some("foo"), Str("baz")),
            Line::new(IndentSig::new(4, false), Some("bar1"), Str("baz")),
            Line::new(IndentSig::new(4, false), Some("bar2"), Str("baz")),
            Line::new(IndentSig::new(2, true), Some("foo1"), Str("baz")),
            Line::new(IndentSig::new(4, false), Some("bar3"), Str("baz")),
            Line::new(IndentSig::new(4, false), Some("bar4"), Str("baz")),
        ];

        let mut iter = UnityYamlIter::from_lines(&lines, true);
        let item = iter.next().unwrap();

        assert_eq!(item.iter().count(), 2);
        for item in item.iter() {
            assert_eq!(item.iter().count(), 3);
        }
    }
}
