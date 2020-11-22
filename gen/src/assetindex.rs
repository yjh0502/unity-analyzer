use log::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::*;

use super::*;

const IGNORE_EXTS: &[&str] = &["a", "so", "cs", "aar", "jar", "dll", "xml"];

fn try_parse_path(mut path: PathBuf) -> Option<(PathBuf, AssetFile)> {
    trace!("file={}", path.display());

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

    match AssetFile::from_path(&path) {
        Err(e) => {
            error!("failed to parse file: {:?}", e);
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

pub struct AssetIndex {
    root: PathBuf,

    /// path -> AssetFile
    assets: HashMap<PathBuf, AssetFile>,

    /// guid -> Path
    asset_guids: HashMap<String, PathBuf>,

    /// sorted forward references, (src, dst) tuple
    forward_refs: Vec<(String, String)>,
}

impl AssetIndex {
    pub fn from_path<P: AsRef<Path>>(root: P) -> Result<Self> {
        let root = root.as_ref();
        let assets_dir = Path::join(root, "Assets");

        let files_list = list_files(&assets_dir)?;
        let meta_files_list = list_meta_files(&assets_dir)?;

        // add unity-serialized files
        let sw = Stopwatch::start_new();
        let mut assets = files_list
            .into_par_iter()
            .filter_map(try_parse_path)
            .collect::<HashMap<_, _>>();
        let elapsed_assets = sw.elapsed_ms();

        let sw = Stopwatch::start_new();
        // add other assets
        meta_files_list
            .into_iter()
            .filter_map(try_parse_path)
            .for_each(|(path, asset)| {
                assets.entry(path).or_insert(asset);
            });
        let elapsed_meta = sw.elapsed_ms();

        // tracking file-level intra-dependencies
        let sw = Stopwatch::start_new();
        let mut forward_refs = Vec::new();
        let mut num_objects = 0;
        for (_path, asset) in assets.iter() {
            let guid = match &asset.meta {
                Some(meta) => meta.guid.clone(),
                None => String::new(),
            };

            num_objects += asset.objects.len();

            let mut refs = HashSet::new();
            for object in &asset.objects {
                for reference in &object.references {
                    if let Some(guid) = &reference.guid {
                        refs.insert(guid.clone());
                    }
                }
            }

            for reference in refs {
                forward_refs.push((guid.clone(), reference));
            }
        }
        forward_refs.sort();
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

        info!(
            "assets={}/{}ms, meta={}ms, objects={}, refs={}/{}ms",
            assets.len(),
            elapsed_assets,
            elapsed_meta,
            num_objects,
            forward_refs.len(),
            elapsed_ref,
        );

        Ok(Self {
            root: root.to_path_buf(),
            assets,
            asset_guids,
            forward_refs,
        })
    }

    pub fn forward_refs(&self, src: String) -> &[(String, String)] {
        use ordslice::Ext;

        let range = self
            .forward_refs
            .equal_range_by(|(ref_src, _ref_dst)| ref_src.cmp(&src));

        &self.forward_refs[range]
    }

    pub fn danglings(&self) -> Result<Vec<PathBuf>> {
        let resources_dir = Path::join(&self.root, "Assets/Resources");
        let streaming_assets_dir = Path::join(&self.root, "Assets/StreamingAssets");

        let mut visited = HashSet::new();
        let mut queue = Vec::new();

        for (path, asset) in &self.assets {
            // handle resources (run-time loadable assets)
            if path.starts_with(&resources_dir) || path.starts_with(&streaming_assets_dir) {
                if let Some(guid) = asset.guid() {
                    queue.push(guid);
                }
                continue;
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

        {
            // temp...
            let file = Path::join(&self.root, "ProjectSettings/EditorBuildSettings.asset");
            let prefix = "    guid: ";
            let f = File::open(&file)?;
            let reader = std::io::BufReader::new(f);
            for line in reader.lines() {
                let line = line?;

                if line.starts_with(prefix) {
                    let guid = &line[(prefix.len())..];
                    queue.push(guid.trim().to_owned());
                }
            }
        }

        while let Some(item) = queue.pop() {
            visited.insert(item.clone());

            for (_, dst) in self.forward_refs(item) {
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
                    danglings.push(path.clone());
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

    pub fn dbg_print_deps(&self, guid: &str) {
        let mut visited = HashSet::new();
        let mut q = std::collections::VecDeque::new();

        // initial GUID
        q.push_back((0, guid.to_owned()));
        visited.insert(guid.to_owned());

        while let Some((depth, guid)) = q.pop_front() {
            if let Some(path) = self.asset_guids.get(&guid) {
                for _ in 0..depth {
                    eprint!("  ");
                }
                eprintln!("{}", self.asset_path_str(path));
            }

            for (_src, dst) in self.forward_refs(guid) {
                if visited.insert(dst.to_owned()) {
                    q.push_front((depth + 1, dst.to_owned()));
                }
            }
        }
    }

    pub fn dbg_print_hierarchy(&self, _guid: &str) {
        todo!();
    }
}