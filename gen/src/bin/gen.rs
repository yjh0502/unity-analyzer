use argh::FromArgs;
use gen::*;
use log::*;
use rayon::prelude::*;
use serde_gen::*;
use std::fs::File;
use std::{
    collections::HashMap,
    io::{BufWriter, Write},
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
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "type-gen", description = "type-gen")]
struct CommandTypeGen {}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "parse", description = "parse")]
struct CommandParse {}

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

fn typegen() -> Result<()> {
    let files_list = gen::files_list("filelist")?;

    let outfile = File::create("out/out.rs")?;
    let mut outfile = BufWriter::new(outfile);

    let mut all_types = HashMap::<String, Ty>::new();

    let sw = Stopwatch::start_new();
    let types_list = files_list
        .into_par_iter()
        .filter_map(|file| match extract_types(&file) {
            Ok(types) => Some(types),
            Err(_e) => {
                log::error!("failed to parse file, filename={}, e={}", file, _e);
                None
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

    write!(outfile, "{}", enum_str)?;

    Ok(())
}

fn parse() -> Result<()> {
    let files_list = gen::files_list("filelist")?;
    let sw = Stopwatch::start_new();

    files_list
        .into_par_iter()
        .map(|file| -> Result<()> {
            debug!("file={}", file);

            let buf = gen::YamlBuf::from_filename(&file)?;
            for res in buf.iter() {
                let (_key, body) = res?;
                let _parsed = serde_yaml::from_str::<serde_yaml::Value>(body);
                if let Err(e) = _parsed {
                    error!("filename={}\ncontent={}\nerr={}", file, body, e);
                }
                //info!("parsed: {:?}", parsed);
            }
            Ok(())
        })
        .collect::<Vec<_>>();

    eprintln!("took={}ms", sw.elapsed_ms());
    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();

    let args: TopLevel = argh::from_env();

    match args.nested {
        SubCommands::TypeGen(_) => typegen(),
        SubCommands::Parse(_) => parse(),
    }
}
