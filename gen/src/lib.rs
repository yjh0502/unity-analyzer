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
    pub fn from_path<P: AsRef<std::path::Path>>(filename: P) -> Result<Self> {
        let file = File::open(&filename)?;
        let mut file = BufReader::new(file);

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        let data = String::from_utf8(buf)?;
        Ok(Self { data })
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
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

    let buf = YamlBuf::from_path(filename)?;

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
    pub file_id: i64,
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
    pub PrefabImporter: Option<NativeFormatImporter>,
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
        if let Some(ref v) = self
            .NativeFormatImporter
            .as_ref()
            .and_then(|v| v.assetBundleName.as_ref())
        {
            return Some(v);
        }
        if let Some(ref v) = self
            .PrefabImporter
            .as_ref()
            .and_then(|v| v.assetBundleName.as_ref())
        {
            return Some(v);
        }
        None
    }
}

#[derive(Default)]
pub struct HierarchyIndex {
    #[allow(unused)]
    relations: Vec<(i64, i64)>,
    roots: Vec<i64>,
}

impl HierarchyIndex {
    fn from_objects(objects: &[Object]) -> Self {
        let mut relations = Vec::new();
        let mut roots = Vec::new();

        for obj in objects {
            // eprintln!("tyname={:?}, file_id={:?}", obj.ty_name, obj.header.file_id);
            if obj.is_transform() {
                if let Some(children) = obj.children() {
                    for child in children {
                        relations.push((obj.header.file_id, child));
                    }
                }
                let father = obj.father();
                // eprintln!("father={:?}", father);
                if father == Some(0) {
                    trace!("root file_id={:?}", obj.header.file_id);
                    roots.push(obj.header.file_id);
                }
            }
        }

        relations.sort();

        Self { relations, roots }
    }

    #[allow(unused)]
    fn children(&self, parent_file_id: i64) -> &[(i64, i64)] {
        use ordslice::Ext;

        let range = self
            .relations
            .equal_range_by(|(src, _dst)| parent_file_id.cmp(&src));

        &self.relations[range]
    }
}

pub struct AssetFile {
    pub meta: Option<FileInfo>,
    pub objects: Vec<Object>,

    pub text_len: usize,

    // file_id -> objects idx
    file_id_indices: HashMap<i64, usize>,

    // heirarchy_index
    pub index: HierarchyIndex,
}

impl AssetFile {
    pub fn from_meta(meta: FileInfo) -> Self {
        Self {
            meta: Some(meta),
            objects: Vec::new(),
            text_len: 0,
            file_id_indices: HashMap::new(),
            index: HierarchyIndex::default(),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut objects = Vec::new();
        let mut file_id_indices = HashMap::new();

        let path = path.as_ref();
        let buf = YamlBuf::from_path(path)?;
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

        let index = HierarchyIndex::from_objects(&objects);

        Ok(Self {
            meta: None,
            objects,
            text_len: buf.data_len(),
            file_id_indices,
            index,
        })
    }

    pub fn guid(&self) -> Option<String> {
        match &self.meta {
            Some(meta) => Some(meta.guid.clone()),
            None => None,
        }
    }

    pub fn object_by_file_id<'a>(&'a self, file_id: i64) -> Option<&'a Object> {
        trace!("object_by_file_id id={}", file_id);
        let idx = self.file_id_indices[&file_id];
        let obj = &self.objects[idx];
        Some(obj)
    }

    pub fn gameobject<'a>(&'a self, file_id: i64) -> Option<&'a Object> {
        let obj = self.object_by_file_id(file_id)?;
        let go_file_id = obj.gameobject()?;
        if go_file_id == 0 {
            // TODO: m_GameObject.fileID == 0. m_PrefabInstance.fileID should be used
            return None;
        }
        self.object_by_file_id(go_file_id)
    }

    pub fn transform<'a>(&'a self, file_id: i64) -> Option<&'a Object> {
        trace!("transform file_id={}", file_id);
        let go = self.gameobject(file_id)?;
        trace!("go={:?}", go.header.file_id);
        let components = go.components()?;
        trace!("components={:?}", components);

        for file_id in components {
            if let Some(component) = self.object_by_file_id(file_id) {
                if component.is_transform() {
                    return Some(component);
                }
            } else {
                warn!("failed to find file_id={}", file_id);
            }
        }

        None
    }

    pub fn dbg_transform_path(&self, file_id: i64) -> Option<String> {
        // in case of stripped file, m_GameObject.fileID == 0 and following rule should be applied
        //  - find prefab instance file with m_PrefabInstance.fileID
        //  - find parent transform from PrefabInstance.m_Modification.m_TransformParent.fileID
        if file_id == 0 {
            return Some("<todo_prefab_instance>".to_owned());
        }
        // FILE_ID_ALIAS_PREFAB_ROOT
        if file_id == 100100000 {
            return Some("<todo_prefab_root>".to_owned());
        }

        // find GameObject first
        trace!("dbg_transform_path id={}", file_id);
        let mut transform = self.transform(file_id)?;

        let mut path = Vec::new();
        loop {
            let go = self.gameobject(transform.header.file_id)?;
            let name = go.get_str("m_Name")?;
            path.push(name);

            let father_id = transform.father()?;
            if father_id == 0 {
                break;
            }
            trace!("father_id={}", father_id);
            transform = self.object_by_file_id(father_id)?;
        }

        path.reverse();
        let path = path
            .into_iter()
            .fold(String::new(), |acc, n| format!("{}/{}", acc, n));
        Some(path)
    }

    fn dbg_object_hierarchy0(&self, prefix: &str, file_id: i64) -> Option<()> {
        let name = self.name_by_file_id(file_id)?;

        let path = format!("{}/{}", prefix, name);
        eprintln!("{}", path);

        let transform = self.transform(file_id).expect("failed to get transform");
        for file_id in transform.children()? {
            if let None = self.dbg_object_hierarchy0(&path, file_id) {
                error!("failed to print hierarchy, file_id={}", file_id);
            }
        }
        Some(())
    }

    pub fn dbg_object_hierarchy(&self) {
        for root_file_id in &self.index.roots {
            if let None = self.dbg_object_hierarchy0("", *root_file_id) {
                error!("failed to print hierarchy, file_id={}", root_file_id);
            }
        }
    }

    pub fn by_parent(&self, parent_file_id: Option<i64>) -> Result<Vec<i64>> {
        match parent_file_id {
            None => Ok(self.index.roots.clone()),
            Some(file_id) => {
                let transform = match self.transform(file_id) {
                    Some(t) => t,
                    None => return Ok(Vec::new()),
                };
                let res = match transform.children() {
                    Some(children) => children,
                    None => Vec::new(),
                };
                Ok(res)
            }
        }
    }

    pub fn roots(&self) -> &[i64] {
        &self.index.roots
    }

    pub fn name_by_file_id(&self, file_id: i64) -> Option<String> {
        match self.name_by_file_id0(file_id)? {
            ObjectName::Name(name) => Some(name),
            ObjectName::Prefab { guid } => Some(format!("<prefab instance, guid={}>", guid)),
        }
    }

    pub fn name_by_file_id_ref(
        &self,
        file_id: i64,
        idx: &assetindex::AssetIndex,
    ) -> Option<String> {
        match self.name_by_file_id0(file_id)? {
            ObjectName::Name(name) => Some(name),
            ObjectName::Prefab { guid } => {
                let prefab = idx.asset_by_guid(&guid)?;

                let roots = prefab.roots();
                assert_eq!(roots.len(), 1);

                prefab.name_by_file_id(roots[0])
            }
        }
    }

    pub fn prefab_source_guid(&self, file_id: i64) -> Option<&str> {
        use yaml_rust::Yaml;
        let transform = self.object_by_file_id(file_id)?;

        if transform.is_prefab_transform() {
            // find prefab instance
            let inst_ref = transform
                .parsed
                .as_hash()?
                .get(&Yaml::from_str("m_PrefabInstance"))?;
            let file_id = object::try_get_file_id(inst_ref)?;
            let inst = self.object_by_file_id(file_id)?;

            // find prefab guid from prefab instance
            let src_ref = inst
                .parsed
                .as_hash()?
                .get(&Yaml::from_str("m_SourcePrefab"))?;
            object::try_get_guid(src_ref)
        } else {
            None
        }
    }

    pub fn name_by_file_id0(&self, file_id: i64) -> Option<ObjectName> {
        let name = if let Some(guid) = self.prefab_source_guid(file_id) {
            ObjectName::Prefab {
                guid: guid.to_owned(),
            }
        } else {
            let go = self.gameobject(file_id)?;
            ObjectName::Name(go.get_str("m_Name")?.to_owned())
        };

        Some(name)
    }
}

pub enum ObjectName {
    Name(String),
    Prefab { guid: String },
}
