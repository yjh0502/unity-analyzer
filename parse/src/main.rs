use log::*;

use nom::bytes::complete::*;
use nom::character::complete::*;
use std::iter::Iterator;
use stopwatch::Stopwatch;

type Result<T> = std::result::Result<T, anyhow::Error>;

pub struct UnityYamlFile<'a> {
    lines: Vec<UnityYamlLine<'a>>,
}

impl<'a> UnityYamlFile<'a> {
    pub fn iter(&self) -> UnityYamlFileIter {
        UnityYamlFileIter {
            cursor: 0,
            file: self,
        }
    }
}

pub struct UnityYamlFileIter<'a> {
    file: &'a UnityYamlFile<'a>,

    cursor: usize,
}

impl<'a> Iterator for UnityYamlFileIter<'a> {
    type Item = UnityYamlObject<'a>;

    fn next(&mut self) -> Option<UnityYamlObject<'a>> {
        if self.cursor >= self.file.lines.len() {
            return None;
        }

        let header = match self.file.lines[self.cursor] {
            UnityYamlLine::Header(ref header) => header,
            UnityYamlLine::Line(ref line) => {
                unreachable!("should be header, found={:?}", line);
            }
        };
        self.cursor += 1;
        let start = self.cursor;

        while self.cursor < self.file.lines.len() {
            let line = &self.file.lines[self.cursor];
            if let UnityYamlLine::Header(_) = line {
                break;
            }
            self.cursor += 1;
        }

        Some(UnityYamlObject {
            header,
            lines: &self.file.lines[start..self.cursor],
        })
    }
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
}

/// iterate over same indent/array_elem
#[derive(Clone)]
pub struct UnityYamlIndentIter<'a> {
    indent: IndentSig,
    lines: &'a [UnityYamlLine<'a>],

    cursor: usize,
}

impl<'a> UnityYamlIndentIter<'a> {
    fn from_lines(lines: &'a [UnityYamlLine]) -> Self {
        let indent = if lines.len() > 0 {
            lines[0].line_unwrap().indent.clone()
        } else {
            IndentSig {
                indent: 0,
                array_elem: false,
            }
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

        let line = self.lines[self.cursor].line_unwrap();

        if line.indent < self.indent {
            return None;
        }

        self.cursor += 1;
        let start = self.cursor;
        while self.cursor < self.lines.len() {
            let line = self.lines[self.cursor].line_unwrap();
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
            .map(|line| {
                let line = line.line_unwrap();
                !line.indent.array_elem
            })
            .unwrap_or(false)
    }

    pub fn is_array(&self) -> bool {
        self.children
            .lines
            .get(0)
            .map(|line| {
                let line = line.line_unwrap();
                line.indent.array_elem
            })
            .unwrap_or(false)
    }

    pub fn iter(&self) -> UnityYamlIndentIter {
        self.children.clone()
    }
}

#[derive(Debug)]
pub struct UnityYamlObject<'a> {
    #[allow(unused)]
    header: &'a ObjectHeader<'a>,
    #[allow(unused)]
    lines: &'a [UnityYamlLine<'a>],
}

impl<'a> UnityYamlObject<'a> {
    pub fn iter(&self) -> UnityYamlIndentIter<'a> {
        UnityYamlIndentIter::from_lines(self.lines)
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
enum UnityYamlLine<'a> {
    Header(ObjectHeader<'a>),
    Line(Line<'a>),
}

impl<'a> UnityYamlLine<'a> {
    fn line_unwrap(&'a self) -> &'a Line<'a> {
        if let UnityYamlLine::Line(ref v) = self {
            v
        } else {
            unreachable!();
        }
    }
}

#[derive(Debug)]
struct ObjectHeader<'a> {
    tag1: &'a str,
    tag2: &'a str,
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

fn parse_chunk_header(i: &str) -> nom::IResult<&str, UnityYamlLine> {
    let (i, (_, _, tag1, _, tag2, _)) =
        nom::sequence::tuple((tag("--- "), tag("!u!"), digit1, tag(" &"), digit1, newline))(i)?;

    Ok((i, UnityYamlLine::Header(ObjectHeader { tag1, tag2 })))
}

fn key_char(chr: char) -> bool {
    chr != ':'
}

fn parse_yaml_like_elem(i: &str) -> nom::IResult<&str, UnityYamlLine> {
    let (i, (cur_indent, _, value, _)) =
        nom::sequence::tuple((space0, tag("- "), not_line_ending, newline))(i)?;

    Ok((
        i,
        UnityYamlLine::Line(Line {
            indent: IndentSig::new(cur_indent.len(), true),
            key: "",
            value,
        }),
    ))
}

fn parse_yaml_like(i: &str) -> nom::IResult<&str, UnityYamlLine> {
    let (i, (cur_indent, key, _, _, value, _)) = nom::sequence::tuple((
        space0,
        take_while(key_char),
        tag(":"),
        space0,
        not_line_ending,
        newline,
    ))(i)?;

    Ok((
        i,
        UnityYamlLine::Line(Line {
            indent: IndentSig::new(cur_indent.len(), false),
            key,
            value,
        }),
    ))
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
    let (i, _) = parse_file_header(i)?;

    let (i, lines) = nom::multi::many1(nom::branch::alt((
        parse_chunk_header,
        parse_yaml_like,
        parse_yaml_like_elem,
    )))(i)?;

    Ok((i, UnityYamlFile { lines }))
}

fn test_parse(input: &str) -> Result<()> {
    let out = parse_unity_yaml(input);
    match out {
        Ok((remaining, parsed)) => {
            let remain_clip_len = remaining.len().min(100);
            debug!(
                "parsed={:?}, remaining len={}, content={}",
                parsed.lines.len(),
                remaining.len(),
                &remaining[..remain_clip_len]
            );

            let elem = parsed.iter().skip(1).next().unwrap();
            debug_print(elem.iter(), 0);

            let count = parsed.iter().count();
            info!("objects={}", count);
        }
        Err(e) => {
            debug!("err={:?}", e);
        }
    };
    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();

    let sw = Stopwatch::start_new();

    let filename = "../files/Art02_Humans.unity";
    let file = std::fs::read_to_string(filename)?;
    test_parse(&file)?;

    let elapsed = sw.elapsed_ms();
    info!(
        "file={}, size={}, elapsed={}ms, {}/sec",
        filename,
        bytesize::ByteSize(file.len() as u64),
        elapsed,
        bytesize::ByteSize(file.len() as u64 * 1000 / elapsed as u64)
    );

    Ok(())
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
