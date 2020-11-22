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

pub mod assetindex;
pub mod object;
pub mod objectheader;
pub mod typegen;

use object::Object;
use objectheader::ObjectHeader;

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
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct FileInfo {
    pub guid: String,
    pub NativeFormatImporter: Option<NativeFormatImporter>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct NativeFormatImporter {
    pub assetBundleName: Option<String>,
}

impl FileInfo {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let parsed = serde_yaml::from_str(&content)?;
        Ok(parsed)
    }

    pub fn asset_bundle_name(&self) -> Option<&str> {
        let importer = self.NativeFormatImporter.as_ref()?;
        importer.assetBundleName.as_ref().map(|s| s.as_str())
    }
}
#[derive(Debug)]
pub struct AssetFile {
    pub meta: Option<FileInfo>,
    pub objects: Vec<Object>,

    // file_id -> objects idx
    file_id_indices: HashMap<i64, usize>,
}

impl AssetFile {
    pub fn from_meta(meta: FileInfo) -> Self {
        Self {
            meta: Some(meta),
            objects: Vec::new(),
            file_id_indices: HashMap::new(),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut objects = Vec::new();
        let mut file_id_indices = HashMap::new();

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

            let obj = Object::from_header_body(header, body)?;

            file_id_indices.insert(obj.header.file_id, objects.len());
            objects.push(obj);
        }

        Ok(Self {
            meta: None,
            objects,
            file_id_indices,
        })
    }

    pub fn guid(&self) -> Option<String> {
        match &self.meta {
            Some(meta) => Some(meta.guid.clone()),
            None => None,
        }
    }

    pub fn object_by_file_id<'a>(&'a self, file_id: i64) -> Option<&'a Object> {
        let idx = self.file_id_indices[&file_id];
        let obj = &self.objects[idx];
        Some(obj)
    }

    pub fn gameobject<'a>(&'a self, file_id: i64) -> Option<&'a Object> {
        let obj = self.object_by_file_id(file_id)?;
        let go = obj.gameobject()?;
        self.object_by_file_id(go)
    }

    pub fn transform<'a>(&'a self, file_id: i64) -> Option<&'a Object> {
        let go = self.gameobject(file_id)?;
        let components = go.components()?;

        for file_id in components {
            if let Some(component) = self.object_by_file_id(file_id) {
                if component.ty_name == object::TY_TRANSFORM {
                    return Some(component);
                }
            } else {
                warn!("failed to find file_id={}", file_id);
            }
        }

        None
    }

    pub fn dbg_transform_path(&self, file_id: i64) {
        let idx = self.file_id_indices[&file_id];
        let _obj = &self.objects[idx];

        // find GameObject first
    }

    pub fn dbg_object_hierarchy(&self) {
        let mut transforms = Vec::new();
        let mut relation = Vec::new();

        for obj in self.objects.iter() {
            if obj.ty_name == object::TY_TRANSFORM {
                if let Some(children) = obj.children() {
                    for child in children {
                        relation.push((obj.header.file_id, child));
                    }
                }

                transforms.push(obj);
            }
        }
    }
}
