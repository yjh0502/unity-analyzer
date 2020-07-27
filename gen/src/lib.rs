use log::*;
use serde_gen::*;
use std::fs::File;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};
use stopwatch::Stopwatch;

type Result<T> = std::result::Result<T, anyhow::Error>;

pub struct YamlBuf {
    data: String,
}

impl YamlBuf {
    pub fn from_filename(filename: &str) -> Result<Self> {
        let file = File::open(&filename)?;
        let mut file = BufReader::new(file);

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        let data = String::from_utf8(buf)?;
        Ok(Self { data })
    }

    pub fn iter(&self) -> YamlIter {
        YamlIter {
            s: self.data.as_str(),
        }
    }
}

pub struct YamlIter<'a> {
    s: &'a str,
}

impl<'a> Iterator for YamlIter<'a> {
    type Item = Result<(&'a str, &'a str)>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut s = self.s;
        if s.is_empty() {
            return None;
        }

        let start = s.find("\n---")?;
        s = &s[(start + 1)..];

        let head_end_of_line = s.find("\n")?;
        let _tag = &s[4..head_end_of_line];

        let start = s.find("\n")?;
        s = &s[(start + 1)..];

        let end = match s.find("\n---") {
            Some(idx) => idx,
            None => s.len(),
        };

        let body = &s[..end];

        self.s = &s[end..];
        Some(Ok((_tag, body)))
    }
    //
}

pub fn extract_types_all(filename: &str, tag: &str) -> Result<HashMap<String, Ty>> {
    let file = File::open(filename)?;

    let parsed: Ty = serde_yaml::from_reader(file)?;
    info!("parsed={:?}", parsed);

    let mut types = HashMap::<String, Ty>::new();
    types.insert(tag.to_owned(), parsed);
    Ok(types)
}

pub fn extract_types(filename: &str) -> Result<HashMap<String, Ty>> {
    let sw = Stopwatch::start_new();

    let buf = YamlBuf::from_filename(filename)?;

    let mut types = HashMap::<String, Ty>::new();
    for res in buf.iter() {
        let (_tag, body) = res?;

        let parsed: Ty = serde_yaml::from_str(body)?;
        let (k, ty) = match parsed {
            Ty::Map(mut v) => {
                assert_eq!(v.len(), 1);
                v.pop().unwrap()
            }
            _ => {
                todo!();
            }
        };

        let prev_ty = match types.get(&k) {
            Some(v) => v.clone(),
            None => Ty::Unit,
        };

        let next_ty = prev_ty + ty;
        types.insert(k, next_ty);
    }

    debug!("filename={}, took={}ms", filename, sw.elapsed_ms());

    Ok(types)
}

// helpers
pub fn files_list(filename: &str) -> Result<Vec<String>> {
    let files_list = BufReader::new(File::open(filename)?);
    let files_list = files_list
        .lines()
        .map(|v| v.map(|v| v.to_owned()))
        .collect::<std::io::Result<Vec<String>>>()?;
    Ok(files_list)
}
