use anyhow::bail;
use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
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

impl std::fmt::Display for IndentSig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (width, s) = if self.array_elem {
            (self.indent + 2, "- ")
        } else {
            (self.indent, "")
        };
        write!(f, "{:>width$}", s, width = width)
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

impl<'a> std::fmt::Display for UnityYamlIter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in &self.lines {
            write!(f, "{}", line)?;
        }
        Ok(())
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

impl<'a> std::fmt::Display for UnityYaml<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.cur, self.children)
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

impl std::fmt::Display for ObjectHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = if self.stripped { " stripped" } else { "" };
        write!(f, "!u!{} &{}{}\n", self.object_id, self.file_id, s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineValue<'a> {
    None,
    Str(&'a str),
    Map(Vec<(&'a str, &'a str)>),
}

impl<'a> std::fmt::Display for LineValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            LineValue::None => {
                write!(f, "\n")
            }
            LineValue::Str(inner) => {
                write!(f, "{}\n", inner)
            }
            LineValue::Map(inner) => {
                write!(f, "{{")?;
                for i in 0..inner.len() {
                    let tup = &inner[i];
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", tup.0, tup.1)?;
                }
                write!(f, "}}\n")
            }
        }
    }
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

impl<'a> std::fmt::Display for Line<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.indent)?;
        if let Some(ref k) = self.key {
            write!(f, "{}: ", k)?;
        }
        write!(f, "{}", self.value)
    }
}

fn newline_n_or_rn(i: &str) -> nom::IResult<&str, ()> {
    let (i, _) = nom::branch::alt((tag("\n"), tag("\r\n")))(i)?;
    Ok((i, ()))
}

fn parse_object_header(i: &str) -> nom::IResult<&str, ObjectHeader> {
    let (i, (_, _, _, tag1, _, neg, tag2, _, tag3)) = nom::sequence::tuple((
        newline_n_or_rn,
        tag("--- "),
        tag("!u!"),
        digit1,
        tag(" &"),
        opt(tag("-")),
        digit1,
        space0,
        alphanumeric0,
    ))(i)?;

    let object_id = tag1.parse().unwrap();
    let file_id: i64 = tag2.parse().unwrap();
    let file_id = if neg.is_some() { -file_id } else { file_id };

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
    let (i, _) = newline_n_or_rn(i)?;
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
    let (i, (_, vec, _)) = nom::sequence::tuple((
        //
        tag("{"),
        nom::multi::many1(parse_yaml_embed_pair),
        tag("}"),
    ))(i)?;
    Ok((i, LineValue::Map(vec)))
}

fn parse_yaml_str_value(i: &str) -> nom::IResult<&str, LineValue> {
    // example
    // foo bar baz - baz\n
    let (i, value) = not_line_ending(i)?;
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
    let (i, value) = take_while(key_char)(i)?;
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
        alt((parse_yaml_embed_value0, parse_yaml_str_value)),
    ))(i)?;
    Ok((i, (Some(key), value)))
}

fn parse_yaml_like(i: &str) -> nom::IResult<&str, Line> {
    use nom::error::*;

    if i.len() > 5 && &i[0..5] == "\n--- " {
        return Err(nom::Err::Error(nom::error::Error::new(
            "looks like an object header",
            ErrorKind::Tag,
        )));
    }

    let (i, (indent, (key, value))) = nom::sequence::tuple((
        parse_indent_sig,
        alt((
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
        newline_n_or_rn,
        tag("%TAG !u! tag:unity3d.com,2011:"),
    ))(i)?;
    Ok((i, ()))
}

fn parse_unity_yaml<'a>(i: &'a str) -> nom::IResult<&'a str, UnityYamlFile> {
    let (i, (_header, objects, _)) = all_consuming(nom::sequence::tuple((
        parse_file_header,
        nom::multi::many1(parse_yaml_object),
        opt(newline_n_or_rn),
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
            "\n      - 0.37482873",
            "\n  - hello",
            "\n  - hello: world",
            "\n  - {x: 1, y: 2}",
        ];

        for line in input {
            let res = parse_yaml_like(line);
            assert!(res.is_ok());
        }
    }

    #[test]
    fn parse_array_map() {
        let line = "\n  - {x: 1, y: 2}";
        let (_remain, parsed) = parse_yaml_like(line).unwrap();
        assert_eq!(_remain.len(), 0);
        assert_eq!(parsed.key, None);
        assert_eq!(parsed.value, LineValue::Map(vec![("x", "1"), ("y", "2")]));
    }

    #[test]
    fn parse_array_kv_map() {
        let line = "\n  - key: {x: 1, y: 2}";
        let (_remain, parsed) = parse_yaml_like(line).unwrap();
        assert_eq!(_remain.len(), 0);
        assert_eq!(parsed.key, Some("key"));
        assert_eq!(parsed.value, LineValue::Map(vec![("x", "1"), ("y", "2")]));
    }

    #[test]
    fn parse_yaml_keyvalue_line_newline_test() {
        let line = "foobarbaz: {x: 1,\n  y: 2}";
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
        let line = "\n      - 0.37482873";
        let res = parse_yaml_like(line);
        assert!(res.is_ok());
    }

    #[test]
    fn parse_object_header() {
        let line = "\n--- !u!123 &456";

        let res = parse_yaml_like(line);
        assert!(res.is_err());
    }

    #[test]
    fn parse_array() {
        let input = r#"
--- !u!123 &456
hello:
  - foo:
    bar:
    baz:
  - foo:
    bar:
    bzz:"#;

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
        let input = r#"
--- !u!1 &2
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

    #[test]
    fn file_test() {
        let content = r#"%YAML 1.1
%TAG !u! tag:unity3d.com,2011:
--- !u!1 &113650
GameObject:
  m_ObjectHideFlags: 0
  m_PrefabParentObject: {fileID: 0}
  m_PrefabInternal: {fileID: 100100000}
  serializedVersion: 4
  m_Component:
  - 4: {fileID: 402652}
  - 20: {fileID: 2009090}
  m_Layer: 0
  m_Name: DeviceCameraRenderer
  m_TagString: Untagged
  m_Icon: {fileID: 0}
  m_NavMeshLayer: 0
  m_StaticEditorFlags: 0
  m_IsActive: 1
--- !u!4 &402652
Transform:
  m_ObjectHideFlags: 1
  m_PrefabParentObject: {fileID: 0}
  m_PrefabInternal: {fileID: 100100000}
  m_GameObject: {fileID: 113650}
  m_LocalRotation: {x: 0, y: 0, z: 0, w: 1}
  m_LocalPosition: {x: 0, y: 0, z: .252090454}
  m_LocalScale: {x: 1, y: 1, z: 1}
  m_Children: []
  m_Father: {fileID: 0}
  m_RootOrder: 0
--- !u!20 &2009090
Camera:
  m_ObjectHideFlags: 1
  m_PrefabParentObject: {fileID: 0}
  m_PrefabInternal: {fileID: 100100000}
  m_GameObject: {fileID: 113650}
  m_Enabled: 1
  serializedVersion: 2
  m_ClearFlags: 2
  m_BackGroundColor: {r: 0, g: 0, b: 0, a: 0}
  m_NormalizedViewPortRect:
    serializedVersion: 2
    x: 0
    y: 0
    width: 1
    height: 1
  near clip plane: .300000012
  far clip plane: 1000
  field of view: 60
  orthographic: 0
  orthographic size: 5
  m_Depth: 100
  m_CullingMask:
    serializedVersion: 2
    m_Bits: 32
  m_RenderingPath: -1
  m_TargetTexture: {fileID: 0}
  m_TargetDisplay: 0
  m_HDR: 0
  m_OcclusionCulling: 0
  m_StereoConvergence: 10
  m_StereoSeparation: .0219999999
--- !u!1001 &100100000
Prefab:
  m_ObjectHideFlags: 1
  serializedVersion: 2
  m_Modification:
    m_TransformParent: {fileID: 0}
    m_Modifications: []
    m_RemovedComponents: []
  m_ParentPrefab: {fileID: 0}
  m_RootGameObject: {fileID: 113650}
  m_IsPrefabParent: 1"#;

        let out = parse_unity_yaml(content);
        if let Err(ref e) = out {
            eprintln!("out={:?}", e);
        }
        assert!(out.is_ok());

        let objects = out.unwrap().1.objects;
        assert_eq!(objects.len(), 4);
    }
}
