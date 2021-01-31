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
#[derive(Clone)]
pub struct UnityYamlIndentIter<'a> {
    indent: IndentSig,
    lines: &'a [Line<'a>],

    cursor: usize,
}

impl<'a> UnityYamlIndentIter<'a> {
    fn from_lines(lines: &'a [Line]) -> Self {
        let indent = if lines.len() > 0 {
            lines[0].indent.clone()
        } else {
            IndentSig::empty()
        };

        Self {
            lines,
            indent,
            cursor: 0,
        }
    }
}

impl<'a> Iterator for UnityYamlIndentIter<'a> {
    type Item = UnityYaml<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor == self.lines.len() {
            return None;
        }

        let line = &self.lines[self.cursor];

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
            children: UnityYamlIndentIter::from_lines(&self.lines[start..self.cursor]),
        })
    }
}

pub struct UnityYaml<'a> {
    cur: &'a Line<'a>,
    children: UnityYamlIndentIter<'a>,
}
impl<'a> UnityYaml<'a> {
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

    pub fn iter(&self) -> UnityYamlIndentIter {
        self.children.clone()
    }
}

#[derive(Debug)]
pub struct UnityYamlObject<'a> {
    #[allow(unused)]
    header: ObjectHeader<'a>,
    #[allow(unused)]
    lines: Vec<Line<'a>>,
}

impl<'a> UnityYamlObject<'a> {
    pub fn iter(&'a self) -> UnityYamlIndentIter<'a> {
        UnityYamlIndentIter::from_lines(&self.lines)
    }
}

#[allow(unused)]
fn debug_print(iter: UnityYamlIndentIter, depth: usize) {
    for item in iter {
        eprintln!("#{} item={:?}", depth, item.cur);
        debug_print(item.children, depth + 1);
    }
}

#[derive(Debug)]
struct ObjectHeader<'a> {
    tag1: &'a str,
    tag2: &'a str,
    tag3: &'a str,
}

#[derive(Debug)]
pub struct Line<'a> {
    indent: IndentSig,
    key: &'a str,
    value: &'a str,
}

impl<'a> Line<'a> {
    pub fn is_embedded_object(&self) -> bool {
        self.value.len() > 0 && self.value.as_bytes()[0] == b'{'
    }
}

fn parse_object_header(i: &str) -> nom::IResult<&str, ObjectHeader> {
    let (i, (_, _, tag1, _, tag2, tag3, _)) = nom::sequence::tuple((
        tag("--- "),
        tag("!u!"),
        digit1,
        tag(" &"),
        digit1,
        alphanumeric0,
        newline,
    ))(i)?;

    Ok((i, ObjectHeader { tag1, tag2, tag3 }))
}

fn key_char(chr: char) -> bool {
    chr != ':'
}

fn parse_indent_sig(i: &str) -> nom::IResult<&str, IndentSig> {
    let (i, cur_indent) = space0(i)?;
    let (i, array_elem) = match tag::<_, _, nom::error::Error<_>>("- ")(i) {
        Ok((i, _)) => (i, true),
        Err(_) => (i, false),
    };
    Ok((i, IndentSig::new(cur_indent.len(), array_elem)))
}

fn parse_yaml_like(i: &str) -> nom::IResult<&str, Line> {
    use nom::error::*;

    let (i, (indent, key, _, _, value, _)) = nom::sequence::tuple((
        parse_indent_sig,
        take_while(key_char),
        tag(":"),
        space0,
        not_line_ending,
        newline,
    ))(i)?;

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
        Err(_e) => bail!("parse error"),
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
}
