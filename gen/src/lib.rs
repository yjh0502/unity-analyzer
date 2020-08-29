use anyhow::bail;
use log::*;
use serde_derive::Deserialize;
use serde_gen::*;
use std::fs::File;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    path::Path,
};
use stopwatch::Stopwatch;

type Result<T> = std::result::Result<T, anyhow::Error>;

pub struct YamlBuf {
    data: String,
}

impl YamlBuf {
    pub fn from_filename<P: AsRef<std::path::Path>>(filename: P) -> Result<Self> {
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

/// 다른 파일로의 reference.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reference {
    pub file_id: usize,
    /// guid가 없는 경우는 local reference, 있는 경우는 remote reference
    pub guid: Option<String>,
}

/// 파일에 대한 정보(meta). 파일은 경로와 guid를 가지고 있습니다.
#[derive(Deserialize)]
pub struct FileInfo {
    pub guid: String,
}

impl FileInfo {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let parsed = serde_yaml::from_str(&content)?;
        Ok(parsed)
    }
}

fn parse_object(header: ObjectHeader, body: &str) -> Result<Object> {
    let parsed = serde_yaml::from_str::<serde_yaml::Value>(body)?;
    let mut references = Vec::new();
    find_references(&parsed, &mut references)?;

    Ok(Object { header, references })
}

fn find_references(node: &serde_yaml::Value, out: &mut Vec<Reference>) -> Result<()> {
    use serde_yaml::Value::*;

    match node {
        Mapping(v) => {
            let mut file_id = None;
            let mut guid = None;
            for (k, v) in v.iter() {
                if let String(key) = k {
                    if key == "fileID" {
                        file_id = Some(v);
                    } else if key == "guid" {
                        guid = Some(v);
                    }
                }
                find_references(v, out)?;
            }

            if let Some(file_id) = file_id {
                if let Some(file_id) = file_id.as_u64() {
                    out.push(Reference {
                        file_id: file_id as usize,
                        guid: guid.and_then(|v| v.as_str()).map(|v| v.to_owned()),
                    });
                }
            }
        }
        Sequence(v) => {
            for item in v.iter() {
                find_references(item, out)?;
            }
        }
        _ => {}
    }

    Ok(())
}

#[derive(Debug)]
pub struct AssetFile {
    pub guid: Option<String>,
    pub objects: Vec<Object>,
}

impl AssetFile {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut objects = Vec::new();

        let path = path.as_ref();
        let buf = YamlBuf::from_filename(path)?;
        for res in buf.iter() {
            let (_key, body) = res?;
            let header = match ObjectHeader::from_str(_key) {
                Ok(header) => header,
                Err(_e) => {
                    error!("failed to parse header, \"{}\"", _key);
                    return Err(_e);
                }
            };

            let obj = parse_object(header, body)?;
            objects.push(obj);
        }

        Ok(Self {
            guid: None,
            objects,
        })
    }
}

/// yaml 파일에 대한 정보. prefab/scene 등이 이에 해당합니다.
/// 내부에서 object tree 구조를 가지고 있습니다.
#[derive(Debug)]
pub struct ObjectHeader {
    /// file-local object id. 따로 쓰는 것 같지는 않지만 일단 파싱합니다
    pub object_id: usize,
    /// fileID
    pub file_id: usize,
}

impl ObjectHeader {
    pub fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        if !s.starts_with("!u!") {
            bail!("unknown header");
        }
        let s = &s[3..];
        let mut split = s.split(" ");

        let object_id = match split.next() {
            Some(s) => s.parse::<usize>()?,
            None => bail!("unknown header"),
        };

        let file_id = match split.next() {
            Some(s) => {
                if !s.starts_with("&") {
                    bail!("unknown header");
                }
                (&s[1..]).parse::<usize>()?
            }
            None => bail!("unknown header"),
        };

        Ok(ObjectHeader { object_id, file_id })
    }
}

/// object 정보. 의존성 분석을 위한거라, reference 정보만 담고 있습니다.
#[derive(Debug)]
pub struct Object {
    pub header: ObjectHeader,
    pub references: Vec<Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_header() {
        ObjectHeader::from_str("!u!29 &1").expect("failed to parse");
    }
}
