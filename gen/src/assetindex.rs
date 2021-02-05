use log::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::*;

use super::*;

const IGNORE_EXTS: &[&str] = &["a", "so", "aar", "jar", "dll", "xml"];

fn try_parse_path(mut path: PathBuf) -> Option<(PathBuf, AssetFile<'static>)> {
    debug!("file={}", path.display());

    let is_meta = match path.extension() {
        Some(ext) => ext == "meta",
        _ => false,
    };

    if is_meta {
        // meta file
        let meta = match FileInfo::from_path(&path) {
            Ok(meta) => meta,
            Err(_e) => {
                error!(
                    "failed to parse a meta file for file={}, {}",
                    path.display(),
                    _e
                );
                return None;
            }
        };
        path.set_extension("");
        if path.is_dir() {
            // ignore directories
            return None;
        }

        if let Some(ext) = path.extension() {
            if IGNORE_EXTS.iter().find(|v| *v == &ext).is_some() {
                return None;
            }
        }

        return Some((path, AssetFile::from_meta(meta)));
    }

    let content0 = std::fs::read_to_string(&path).ok()?;
    let content: &'static str = unsafe { std::mem::transmute(content0.as_str()) };
    std::mem::forget(content0);

    match AssetFile::from_str(&content) {
        Err(e) => {
            error!("failed to parse file filename={:?}, error={:?}", path, e);
            None
        }
        Ok(mut parsed) => {
            let mut filename = std::ffi::OsString::from(path.file_name().unwrap());
            filename.push(".meta");
            let mut meta_file = PathBuf::from(&path);
            meta_file.set_file_name(filename);

            match FileInfo::from_path(&meta_file) {
                Ok(meta) => {
                    parsed.meta = Some(meta);
                }
                Err(_e) => {
                    error!(
                        "failed to parse a meta file for file={}, {}",
                        path.display(),
                        _e
                    );
                }
            }

            Some((path, parsed))
        }
    }
}

fn check_yaml<P: AsRef<Path>>(path: P) -> Result<bool> {
    let mut file = File::open(path)?;
    let header = b"%YAML 1.1";

    let mut buf = Vec::with_capacity(header.len());
    buf.resize(header.len(), 0);

    file.read(&mut buf)?;

    Ok(buf.as_slice() == header)
}

fn list_meta_files<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();

    use walkdir::WalkDir;
    for entry in WalkDir::new(&dir) {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            continue;
        }

        let path = entry.path();

        if let Some(ext) = path.extension() {
            if ext == "meta" {
                out.push(path.into());
            }
        }
    }
    Ok(out)
}

pub fn list_files<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();

    use walkdir::WalkDir;
    for entry in WalkDir::new(&dir) {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            continue;
        }

        let path = entry.path();
        if !check_yaml(path)? {
            continue;
        }

        out.push(path.into());
    }

    Ok(out)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ForwardRef {
    pub src_guid: String,
    pub src_file_id: i64,
    pub dst_guid: String,
    pub dst_file_id: i64,
}

pub struct AssetIndex {
    root: PathBuf,

    /// path -> AssetFile
    pub assets: HashMap<PathBuf, AssetFile<'static>>,

    /// guid -> Path
    asset_guids: HashMap<String, PathBuf>,

    /// sorted forward references, sort by (src_guid, src_file_id)
    forward_refs: Vec<ForwardRef>,

    /// sorted backward references, sort by (dst_guid, dst_file_id)
    backward_refs: Vec<ForwardRef>,
}

fn sw_step(sw: &mut stopwatch::Stopwatch, label: &'static str) {
    info!("{}: {}ms", label, sw.elapsed_ms());
    sw.restart();
}

impl AssetIndex {
    pub fn from_path<P: AsRef<Path>>(root: P) -> Result<Self> {
        let root = root.as_ref();
        let assets_dir = Path::join(root, "Assets");

        let mut sw0 = Stopwatch::start_new();

        let files_list = {
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            let mut files_list = list_files(&assets_dir)?;
            files_list.shuffle(&mut rng);
            files_list
        };
        let meta_files_list = list_meta_files(&assets_dir)?;

        sw_step(&mut sw0, "listing");

        // add unity-serialized files
        let sw = Stopwatch::start_new();
        let mut assets = files_list
            .into_par_iter()
            .filter_map(try_parse_path)
            .collect::<HashMap<_, _>>();

        sw_step(&mut sw0, "parsing-assets");

        // add other assets
        meta_files_list
            .into_iter()
            .filter_map(try_parse_path)
            .for_each(|(path, asset)| {
                assets.entry(path).or_insert(asset);
            });
        let elapsed_parsing = sw.elapsed_ms();

        sw_step(&mut sw0, "parsing-meta");

        // tracking file-level intra-dependencies
        let sw = Stopwatch::start_new();
        let mut refs = HashSet::new();
        let mut num_objects = 0;
        let mut total_len = 0;
        for (_path, asset) in assets.iter() {
            let src_guid = match &asset.meta {
                Some(meta) => meta.guid.clone(),
                None => String::new(),
            };

            total_len += asset.text_len;
            num_objects += asset.objects.len();

            for object in &asset.objects {
                for reference in &object.references {
                    if let Some(guid) = &reference.guid {
                        refs.insert(ForwardRef {
                            src_guid: src_guid.clone(),
                            src_file_id: object.header.file_id,
                            dst_guid: guid.clone(),
                            dst_file_id: reference.file_id,
                        });
                    }
                }
            }
        }

        sw_step(&mut sw0, "analyzing-deps");

        let mut forward_refs = refs.into_iter().collect::<Vec<_>>();
        let mut backward_refs = forward_refs.clone();
        debug!("refs.len()={:?}", forward_refs.len());

        forward_refs.sort();
        backward_refs.sort_by(|a, b| {
            a.dst_guid
                .cmp(&b.dst_guid)
                .then(a.dst_file_id.cmp(&b.dst_file_id))
        });
        info!("analyzing-sort: {}ms", sw.elapsed_ms());

        sw_step(&mut sw0, "analyzing-sort");

        let elapsed_ref = sw.elapsed_ms();

        // guid to path mapping
        let asset_guids = assets
            .iter()
            .filter_map(|(path, asset)| {
                asset
                    .meta
                    .as_ref()
                    .map(|meta| (meta.guid.clone(), path.clone()))
            })
            .collect();

        sw_step(&mut sw0, "analyzing-mapping");

        info!(
            "assets={}/{}ms ({}, {}/s), objects={}, refs={}/{}ms",
            assets.len(),
            elapsed_parsing,
            bytesize::ByteSize::b(total_len as u64),
            bytesize::ByteSize::b(total_len as u64 * 1000 / elapsed_parsing as u64),
            num_objects,
            forward_refs.len(),
            elapsed_ref,
        );

        Ok(Self {
            root: root.to_path_buf(),
            assets,
            asset_guids,
            forward_refs,
            backward_refs,
        })
    }

    pub fn forward_refs(&self, src_guid: &str) -> &[ForwardRef] {
        use ordslice::Ext;
        let range = self
            .forward_refs
            .equal_range_by(|r| r.src_guid.as_str().cmp(src_guid));

        &self.forward_refs[range]
    }

    pub fn backward_refs(&self, dst_guid: &str) -> &[ForwardRef] {
        use ordslice::Ext;
        let range = self
            .backward_refs
            .equal_range_by(|r| r.dst_guid.as_str().cmp(dst_guid));

        &self.backward_refs[range]
    }

    pub fn scene_guids(&self) -> Result<Vec<(String, String)>> {
        let path = Path::join(&self.root, "ProjectSettings/EditorBuildSettings.asset");

        let file_content = std::fs::read_to_string(&path)?;
        let asset_file = AssetFile::from_str(&file_content)?;
        let parsed = &asset_file.objects[0].parsed;

        let get_scenes = || -> Option<Vec<(String, String)>> {
            let seq = try_find_value(&parsed, "m_Scenes")?;

            let v = seq
                .iter()
                .filter_map(|item| {
                    let path = object::try_find_value_str(&item, "path")?;
                    let guid = object::try_find_value_str(&item, "guid")?;
                    Some((path.to_owned(), guid.to_owned()))
                })
                .collect::<Vec<_>>();
            Some(v)
        };

        match get_scenes() {
            Some(v) => Ok(v),
            None => bail!("failed to list scenes"),
        }
    }

    pub fn danglings(&self, includes: Vec<String>) -> Result<Vec<PathBuf>> {
        let resources_dir = Path::join(&self.root, "Assets/Resources");
        let streaming_assets_dir = Path::join(&self.root, "Assets/StreamingAssets");

        let mut visited = HashSet::new();
        let mut queue = self
            .scene_guids()?
            .into_iter()
            .map(|(_path, guid)| guid)
            .collect::<Vec<_>>();

        let mut patterns = Vec::new();
        for pat in includes {
            patterns.push(glob::Pattern::new(&pat)?);
        }

        for (path, asset) in &self.assets {
            // handle resources (run-time loadable assets)
            if path.starts_with(&resources_dir) || path.starts_with(&streaming_assets_dir) {
                if let Some(guid) = asset.guid() {
                    queue.push(guid);
                }
                continue;
            }

            if let Ok(postfix) = path.strip_prefix(&self.root) {
                for pat in &patterns {
                    if pat.matches_path(postfix) {
                        if let Some(guid) = asset.guid() {
                            queue.push(guid);
                        }
                    }
                }
            }

            if let Some(meta) = &asset.meta {
                // mark all asset bundles dirty
                if meta.asset_bundle_name().is_some() {
                    if let Some(guid) = asset.guid() {
                        queue.push(guid);
                    }
                }
            }
        }

        info!("roots={}", queue.len());

        while let Some(item) = queue.pop() {
            visited.insert(item.clone());

            for forward_ref in self.forward_refs(&item) {
                let dst = &forward_ref.dst_guid;
                if visited.contains(dst) {
                    continue;
                }

                queue.push(dst.clone());
            }
        }

        info!("total={}, visited={}", self.assets.len(), visited.len());

        let mut danglings = Vec::new();
        for (path, asset) in &self.assets {
            if let Some(meta) = &asset.meta {
                if !visited.contains(&meta.guid) {
                    let path = path.strip_prefix(&self.root)?;
                    danglings.push(path.to_path_buf());
                }
            }
        }

        danglings.sort();
        Ok(danglings)
    }

    fn asset_path_str(&self, path: &Path) -> String {
        let stripped = path
            .strip_prefix(&self.root)
            .expect("failed to strip prefix");
        stripped.to_string_lossy().to_string()
    }

    pub fn try_asset_path_by_guid(&self, guid: &str) -> Option<&PathBuf> {
        match self.asset_guids.get(guid) {
            Some(path) => Some(path),
            None => {
                warn!(
                        "guid not found in index, maybr missing reference or package dependency: guid={}",
                        guid
                    );
                None
            }
        }
    }

    pub fn dbg_print_reverse_deps(&self, guid: &str) {
        let refs = self.backward_refs(guid);

        for r in refs {
            let path = match self.try_asset_path_by_guid(&r.src_guid) {
                Some(p) => p,
                None => continue,
            };

            let asset_file = &self.assets[path];
            let hierarchy_path = asset_file
                .dbg_transform_path(r.src_file_id)
                .unwrap_or_else(|| "<todo_unknown_path>".to_owned());

            eprintln!("{} (from={})", self.asset_path_str(path), hierarchy_path);
        }
    }

    pub fn dbg_print_deps(&self, guid: &str) {
        let mut visited = HashSet::new();
        let mut q = std::collections::VecDeque::new();

        // initial GUID
        q.push_back((0, "<root>".to_owned(), guid.to_owned()));
        visited.insert(guid.to_owned());

        while let Some((depth, src_path, guid)) = q.pop_front() {
            let path = match self.asset_guids.get(&guid) {
                Some(path) => path,
                None => {
                    warn!(
                        "guid not found in index, maybr missing reference or package dependency: guid={}",
                        guid
                    );
                    continue;
                }
            };

            for _ in 0..depth {
                eprint!("  ");
            }
            eprintln!("{} (from={})", self.asset_path_str(&path), src_path);

            let src_asset_file = self.asset_by_guid(&guid).unwrap();

            for forward_ref in self.forward_refs(&guid) {
                let dst_guid = &forward_ref.dst_guid;
                /*
                let asset_file = match self.asset_by_guid(dst_guid) {
                    Some(asset) => asset,
                    None => {
                        debug!("unknown guid={}", dst_guid);
                        continue;
                    }
                };
                */

                let src_path = src_asset_file
                    .dbg_transform_path(forward_ref.src_file_id)
                    .unwrap_or_else(|| "<unkown_root>".to_owned());

                if visited.insert(dst_guid.to_owned()) {
                    q.push_front((depth + 1, src_path, dst_guid.to_owned()));
                }
            }
        }
    }

    pub fn asset_by_guid(&self, guid: &str) -> Option<&AssetFile> {
        let path = self.asset_guids.get(guid)?;
        let asset = self.assets.get(path)?;
        Some(asset)
    }

    pub fn dbg_print_hierarchy(&self, guid: &str) -> Option<()> {
        let asset = self.asset_by_guid(guid)?;
        info!("dbg_print_hierarchy, guid={}", guid);
        asset.dbg_object_hierarchy();
        Some(())
    }

    pub fn dbg_stats(&self) -> String {
        format!(
            "assets={} refs={}",
            self.assets.len(),
            self.forward_refs.len()
        )
    }
}
