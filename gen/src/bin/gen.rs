use log::*;
use serde_gen::*;
use std::fs::File;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, BufWriter, Write},
};
use stopwatch::Stopwatch;

use rayon::prelude::*;

use gen::*;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn filter_ty(ty: Ty, spec: &FilterSpec) -> Ty {
    match ty {
        Ty::Map(inner) => {
            let mut matched = true;
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

struct FilterSpec {
    keys: Vec<String>,
    alt: Ty,
}

fn main() -> Result<()> {
    env_logger::init();

    let files_list = File::open("filelist")?;
    let files_list = BufReader::new(files_list);

    let outfile = File::create("out/out.rs")?;
    let mut outfile = BufWriter::new(outfile);

    let mut all_types = HashMap::<String, Ty>::new();

    let mut files = Vec::new();
    for file in files_list.lines() {
        files.push(file?);
    }

    let sw = Stopwatch::start_new();
    let types_list = files
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

    let specs = &[
        FilterSpec {
            keys: vec![
                "r".to_owned(),
                "g".to_owned(),
                "b".to_owned(),
                "a".to_owned(),
            ],
            alt: Ty::Map(vec![
                ("r".to_owned(), Ty::F),
                ("g".to_owned(), Ty::F),
                ("b".to_owned(), Ty::F),
                ("a".to_owned(), Ty::F),
            ]),
        },
        FilterSpec {
            keys: vec![
                "x".to_owned(),
                "y".to_owned(),
                "z".to_owned(),
                "w".to_owned(),
            ],
            alt: Ty::Map(vec![
                ("x".to_owned(), Ty::F),
                ("y".to_owned(), Ty::F),
                ("z".to_owned(), Ty::F),
                ("w".to_owned(), Ty::F),
            ]),
        },
        FilterSpec {
            keys: vec!["x".to_owned(), "y".to_owned(), "z".to_owned()],
            alt: Ty::Map(vec![
                ("x".to_owned(), Ty::F),
                ("y".to_owned(), Ty::F),
                ("z".to_owned(), Ty::F),
            ]),
        },
        FilterSpec {
            keys: vec!["x".to_owned(), "y".to_owned()],
            alt: Ty::Map(vec![("x".to_owned(), Ty::F), ("y".to_owned(), Ty::F)]),
        },
        FilterSpec {
            keys: vec!["fileID".to_owned()],
            alt: Ty::Map(vec![
                ("fileID".to_owned(), Ty::Str(String::new())),
                (
                    "guid".to_owned(),
                    Ty::Some(Box::new(Ty::Str(String::new()))),
                ),
                ("type".to_owned(), Ty::Some(Box::new(Ty::U))),
            ]),
        },
    ];

    for (_k, ty) in all_types.iter_mut() {
        for spec in specs {
            *ty = filter_ty(ty.clone(), spec);
        }
    }

    let mut builder = TyBuilder::new();
    builder.set_any_ty("::serde_yaml::Value");

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
