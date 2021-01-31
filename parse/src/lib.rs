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

    fn is_empty(&self) -> bool {
        *self == Self::empty()
    }
}

/// iterate over same indent/array_elem
#[derive(Clone, Debug)]
pub struct UnityYamlIter<'a> {
    indent: IndentSig,
    lines: Vec<Line<'a>>,

    cursor: usize,
}

impl<'a> UnityYamlIter<'a> {
    fn from_lines<'b>(lines: &'b [Line<'a>]) -> Self {
        let indent = if lines.len() > 0 {
            lines[0].indent.clone()
        } else {
            IndentSig::empty()
        };

        Self {
            lines: lines.to_vec(),
            indent,
            cursor: 0,
        }
    }
}

impl<'a> Iterator for UnityYamlIter<'a> {
    type Item = UnityYaml<'a>;

    fn next<'b>(&'b mut self) -> Option<Self::Item> {
        if self.cursor == self.lines.len() {
            return None;
        }

        let line = self.lines[self.cursor].clone();

        if line.indent < self.indent {
            return None;
        }

        self.cursor += 1;
        let start = self.cursor;
        while self.cursor < self.lines.len() {
            let line = &self.lines[self.cursor];
            if line.indent <= self.indent {
                break;
            }
            self.cursor += 1;
        }

        Some(UnityYaml {
            cur: line,
            children: UnityYamlIter::from_lines(&self.lines[start..self.cursor]),
        })
    }
}

#[derive(Debug)]
pub struct UnityYaml<'a> {
    cur: Line<'a>,
    children: UnityYamlIter<'a>,
}
impl<'a> UnityYaml<'a> {
    pub fn key(&self) -> &'a str {
        self.cur.key
    }

    pub fn is_value(&self) -> bool {
        self.children.lines.is_empty()
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
        self.children
            .lines
            .get(0)
            .map(|line| !line.indent.array_elem)
            .unwrap_or(false)
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
        UnityYamlIter::from_lines(&self.lines)
    }
}

#[allow(unused)]
fn debug_print(iter: UnityYamlIter, depth: usize) {
    for item in iter {
        eprintln!("#{} item={:?}", depth, item.cur);
        debug_print(item.children, depth + 1);
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

#[derive(Debug, Clone)]
pub enum LineValue<'a> {
    None,
    Str(&'a str),
    Map(Vec<(&'a str, &'a str)>),
}

#[derive(Debug, Clone)]
pub struct Line<'a> {
    indent: IndentSig,
    key: &'a str,
    value: LineValue<'a>,
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

fn parse_yaml_non_value(i: &str) -> nom::IResult<&str, LineValue> {
    let (i, _) = newline(i)?;
    Ok((i, LineValue::None))
}

fn parse_yaml_embed_pair(i: &str) -> nom::IResult<&str, (&str, &str)> {
    let (i, (_, key, _, _, value)) = nom::sequence::tuple((
        take_while(|c| c == ',' || c == ' '),
        take_while(key_char),
        tag(":"),
        space0,
        take_while(embed_value_char),
    ))(i)?;
    Ok((i, (key, value)))
}

fn parse_yaml_embed_value0(i: &str) -> nom::IResult<&str, LineValue> {
    let (i, (_, vec, _, _)) = nom::sequence::tuple((
        //
        tag("{"),
        nom::multi::many1(parse_yaml_embed_pair),
        tag("}"),
        newline,
    ))(i)?;
    Ok((i, LineValue::Map(vec)))
}

fn parse_yaml_embed_value(i: &str) -> nom::IResult<&str, LineValue> {
    let (i, (_, _, value, _)) =
        nom::sequence::tuple((tag(":"), space0, parse_yaml_embed_value0, newline))(i)?;
    Ok((i, value))
}

fn parse_yaml_str_value(i: &str) -> nom::IResult<&str, LineValue> {
    let (i, (_, _, value, _)) =
        nom::sequence::tuple((tag(":"), space0, not_line_ending, newline))(i)?;
    Ok((i, LineValue::Str(value)))
}

fn parse_yaml_like(i: &str) -> nom::IResult<&str, Line> {
    use nom::error::*;

    let (i, (indent, key, value)) = nom::sequence::tuple((
        parse_indent_sig,
        take_while(key_char),
        nom::branch::alt((
            parse_yaml_non_value,
            parse_yaml_embed_value,
            parse_yaml_str_value,
        )),
    ))(i)?;

    let (key, value) = match value {
        LineValue::None => (&key[0..0], LineValue::Str(key)),
        value => (key, value),
    };

    // TODO: perf
    if indent.is_empty() && key.starts_with("---") {
        return Err(nom::Err::Error(nom::error::Error::new(
            "looks like an object header",
            ErrorKind::Tag,
        )));
    }

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
    fn parse_number_line() {
        let line = "      - 0.37482873\n";
        let res = parse_yaml_like(line);
        assert!(res.is_ok());
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
}
