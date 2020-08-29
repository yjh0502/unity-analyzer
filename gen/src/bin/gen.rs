use anyhow::bail;
use argh::FromArgs;
use gen::*;
use log::*;
use rayon::prelude::*;
use serde_gen::*;
use std::fs::File;
use std::{
    collections::HashMap,
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
    dir: String,
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

fn parse_meta_file<P: AsRef<Path>>(path: P) -> Result<FileInfo> {
    let content = std::fs::read_to_string(path)?;
    let parsed = serde_yaml::from_str(&content)?;
    Ok(parsed)
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

fn parse_object(header: ObjectHeader, body: &str) -> Result<Object> {
    let parsed = serde_yaml::from_str::<serde_yaml::Value>(body)?;
    let mut references = Vec::new();
    find_references(&parsed, &mut references)?;

    Ok(Object { header, references })
}

fn parse_asset_file<P: AsRef<Path>>(path: P) -> Result<AssetFile> {
    let mut objects = Vec::new();

    let path = path.as_ref();
    let buf = gen::YamlBuf::from_filename(path)?;
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

    Ok(AssetFile {
        guid: None,
        objects,
    })
}

fn parse(v: CommandParse) -> Result<()> {
    let files_list = list_files0(&v.dir, false)?;
    let sw = Stopwatch::start_new();

    // let files_list = files_list.into_iter().take(10).collect::<Vec<_>>();

    files_list
        .into_iter()
        .filter_map(|file| -> Option<AssetFile> {
            debug!("file={}", file.display());

            match parse_asset_file(&file) {
                Err(e) => {
                    error!("failed to parse file: {:?}", e);
                    None
                }
                Ok(mut parsed) => {
                    let mut filename = std::ffi::OsString::from(file.file_name().unwrap());
                    filename.push(".meta");
                    let mut meta_file = PathBuf::from(&file);
                    meta_file.set_file_name(filename);

                    match parse_meta_file(&meta_file) {
                        Ok(meta) => {
                            parsed.guid = Some(meta.guid);
                        }
                        Err(_e) => {
                            error!(
                                "failed to parse a meta file for file={}, {}",
                                file.display(),
                                _e
                            );
                        }
                    }

                    Some(parsed)
                }
            }
        })
        .collect::<Vec<_>>();

    eprintln!("took={}ms", sw.elapsed_ms());
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

fn list_files0<P: AsRef<Path>>(dir: P, include_meta: bool) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();

    use walkdir::WalkDir;

    for entry in WalkDir::new(&dir) {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            continue;
        }

        let path = entry.path();

        let mut accepted = false;

        if let Some(ext) = path.extension() {
            if ext == "meta" {
                if !include_meta {
                    continue;
                }
                accepted = true;
            }
        }

        if !accepted && check_yaml(path)? {
            accepted = true;
        }

        if !accepted {
            continue;
        }

        out.push(path.into());
    }

    Ok(out)
}

fn list_files(v: CommandListFiles) -> Result<()> {
    let path_list = list_files0(&v.dir, true)?;

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
