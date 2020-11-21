use argh::FromArgs;
use gen::*;
use log::*;
use rayon::prelude::*;
use serde_gen::*;
use std::fs::File;
use std::{
    collections::{HashMap, HashSet},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};
use stopwatch::Stopwatch;

type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(FromArgs, Debug)]
#[argh(description = "top level")]
struct TopLevel {
    #[argh(subcommand)]
    nested: SubCommands,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum SubCommands {
    TypeGen(CommandTypeGen),
    Parse(CommandParse),
    ListFiles(CommandListFiles),
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "type-gen", description = "type-gen")]
struct CommandTypeGen {
    #[argh(option, description = "delimited")]
    delimited: bool,

    #[argh(positional)]
    filename: String,

    #[argh(positional)]
    out_filename: String,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "parse", description = "parse")]
struct CommandParse {
    #[argh(positional)]
    dir: PathBuf,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "ls", description = "ls")]
struct CommandListFiles {
    #[argh(positional)]
    dir: String,
}

mod typegen {
    use super::*;

    pub fn filter_ty(ty: Ty, spec: &FilterSpec) -> Ty {
        match ty {
            Ty::Map(inner) => {
                let mut matched = true;
                for neg_key in spec.neg_keys.iter() {
                    let key_found = inner.iter().find(|(k, _v)| k == neg_key).is_some();
                    if key_found {
                        matched = false;
                        break;
                    }
                }

                for key in spec.keys.iter() {
                    let key_found = inner.iter().find(|(k, _v)| k == key).is_some();
                    if !key_found {
                        matched = false;
                        break;
                    }
                }

                if !matched {
                    let inner = inner
                        .into_iter()
                        .map(|(k, v)| (k, filter_ty(v, spec)))
                        .collect();
                    Ty::Map(inner)
                } else {
                    spec.alt.clone()
                }
            }
            Ty::Seq(inner) => Ty::Seq(Box::new(filter_ty(*inner, spec))),
            Ty::Some(inner) => Ty::Some(Box::new(filter_ty(*inner, spec))),
            other => other,
        }
    }

    pub struct FilterSpec {
        pub keys: Vec<String>,
        pub neg_keys: Vec<String>,
        pub alt: Ty,
    }

    impl FilterSpec {
        pub fn new(keys: &[&str], neg_keys: &[&str], alt: &[(&str, Ty)]) -> Self {
            let keys = keys.iter().map(|k| (*k).to_owned()).collect();
            let neg_keys = neg_keys.iter().map(|k| (*k).to_owned()).collect();
            let alt = Ty::Map(
                alt.iter()
                    .map(|(k, ty)| ((*k).to_owned(), ty.clone()))
                    .collect(),
            );
            Self {
                keys,
                neg_keys,
                alt,
            }
        }
    }
}

fn typegen(cmd: CommandTypeGen) -> Result<()> {
    let files_list = gen::files_list(&cmd.filename)?;

    let outfile = File::create(&cmd.out_filename)?;
    let mut outfile = BufWriter::new(outfile);

    let mut all_types = HashMap::<String, Ty>::new();

    let sw = Stopwatch::start_new();
    let types_list = files_list
        .into_iter()
        .filter_map(|file| {
            let res = if cmd.delimited {
                extract_types(&file)
            } else {
                extract_types_all(&file, "Root")
            };
            match res {
                Ok(types) => Some(types),
                Err(_e) => {
                    log::error!("failed to parse file, filename={}, e={}", file, _e);
                    None
                }
            }
        })
        .collect::<Vec<_>>();
    info!("type extraction took={}ms", sw.elapsed_ms());

    for types in types_list {
        for (k, ty) in types.into_iter() {
            let prev_ty = match all_types.get(&k) {
                Some(v) => v.clone(),
                None => Ty::Unit,
            };

            let next_ty = prev_ty + ty;
            all_types.insert(k, next_ty);
        }
    }

    use typegen::*;

    let specs = &[
        FilterSpec::new(
            &["r", "g", "b", "a"],
            &["rgba"],
            &[("r", Ty::F), ("g", Ty::F), ("b", Ty::F), ("a", Ty::F)],
        ),
        FilterSpec::new(
            &["x", "y", "z", "w"],
            &["curve", "enabled"],
            &[("x", Ty::F), ("y", Ty::F), ("z", Ty::F), ("w", Ty::F)],
        ),
        FilterSpec::new(
            &["x", "y", "z"],
            &["curve", "enabled"],
            &[("x", Ty::F), ("y", Ty::F), ("z", Ty::F)],
        ),
        FilterSpec::new(
            &["x", "y"],
            &["curve", "enabled"],
            &[("x", Ty::F), ("y", Ty::F)],
        ),
        FilterSpec::new(
            &["inSlope", "outSlope", "inWeight", "outWeight"],
            &[],
            &[
                ("serializedVersion", Ty::U),
                ("time", Ty::F),
                ("value", Ty::Any),
                ("inSlope", Ty::Any),
                ("outSlope", Ty::Any),
                ("tangentMode", Ty::U),
                ("weightedMode", Ty::Some(Box::new(Ty::U))),
                ("inWeight", Ty::Some(Box::new(Ty::Any))),
                ("outWeight", Ty::Some(Box::new(Ty::Any))),
            ],
        ),
        FilterSpec::new(
            &["fileID"],
            &["enabled"],
            &[
                ("fileID", Ty::Str(String::new())),
                ("guid", Ty::Some(Box::new(Ty::Str(String::new())))),
                ("type", Ty::Some(Box::new(Ty::U))),
            ],
        ),
    ];

    for (_k, ty) in all_types.iter_mut() {
        for spec in specs {
            *ty = filter_ty(ty.clone(), spec);
        }
    }

    let mut builder = TyBuilder::new();
    builder.set_any_ty("::serde_yaml::Value");

    let mut all_types = all_types.into_iter().collect::<Vec<_>>();
    all_types.sort_by(|(a, _), (b, _)| a.cmp(b));

    let enum_str = {
        use std::fmt::Write;

        let mut s = String::new();

        write!(
            s,
            r#"#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub enum Root {{
"#
        )?;

        for (k, _ty) in all_types.iter() {
            write!(s, "    {}({}),\n", k, k)?;
        }
        write!(s, "}}")?;
        s
    };

    for (k, ty) in all_types.into_iter() {
        let out_src = builder.build(&k, ty);
        write!(outfile, "// {}\n", k)?;
        write!(outfile, "{}\n", out_src)?;
    }

    if cmd.delimited {
        write!(outfile, "{}", enum_str)?;
    }

    Ok(())
}

#[allow(unused)]
struct AssetIndex {
    root: PathBuf,

    /// guid -> AssetFile
    assets: HashMap<PathBuf, AssetFile>,

    /// sorted
    forward_refs: Vec<(String, String)>,
}

impl AssetIndex {
    fn from_path<P: AsRef<Path>>(root: P) -> Result<Self> {
        let root = root.as_ref();
        let assets_dir = Path::join(root, "Assets");

        let files_list = list_files0(&assets_dir)?;
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
            forward_refs,
        })
    }

    fn forward_refs(&self, src: String) -> &[(String, String)] {
        use ordslice::Ext;

        let range = self
            .forward_refs
            .equal_range_by(|(ref_src, _ref_dst)| ref_src.cmp(&src));

        &self.forward_refs[range]
    }

    fn danglings(&self) -> Result<Vec<PathBuf>> {
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
            use std::io::BufRead;

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
}

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

fn parse(v: CommandParse) -> Result<()> {
    let sw = Stopwatch::start_new();

    let idx = AssetIndex::from_path(&v.dir)?;

    let danglings = idx.danglings()?;
    let danglings_count = danglings.len();
    let mut file = File::create("dangling.log")?;
    for path in danglings {
        write!(&mut file, "{}\n", path.display())?;
    }

    info!("took={}ms, danglings={}", sw.elapsed_ms(), danglings_count);
    Ok(())
}

fn check_yaml<P: AsRef<Path>>(path: P) -> Result<bool> {
    use std::io::Read;

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

fn list_files0<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>> {
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

fn list_files(v: CommandListFiles) -> Result<()> {
    let path_list = list_files0(&v.dir)?;

    for path in path_list {
        println!("{}", path.display());
    }

    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();

    let args: TopLevel = argh::from_env();

    match args.nested {
        SubCommands::TypeGen(v) => typegen(v),
        SubCommands::Parse(v) => parse(v),
        SubCommands::ListFiles(v) => list_files(v),
    }
}
