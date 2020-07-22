use log::*;
use std::fs::File;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, BufWriter, Read, Write},
};
use stopwatch::Stopwatch;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn extract_types(filename: &str) -> Result<HashMap<String, serde_gen::Ty>> {
    let sw = Stopwatch::start_new();
    let file = File::open(&filename)?;
    let mut file = BufReader::new(file);

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let mut s = std::str::from_utf8(&mut buf)?;
    let mut types = HashMap::<String, serde_gen::Ty>::new();

    while !s.is_empty() {
        let start = s.find("\n---").unwrap();
        s = &s[(start + 1)..];

        let head_end_of_line = s.find("\n").unwrap();
        let _tag = &s[4..head_end_of_line];

        let end = match s.find("\n---") {
            Some(idx) => idx,
            None => s.len(),
        };

        // info!("{}", &s[..end]);
        let parsed: serde_gen::Ty = serde_yaml::from_str(&s[..end])?;
        let (k, ty) = match parsed {
            serde_gen::Ty::Map(mut v) => {
                assert_eq!(v.len(), 1);
                v.pop().unwrap()
            }
            _ => {
                todo!();
            }
        };

        let prev_ty = match types.get(&k) {
            Some(v) => v.clone(),
            None => serde_gen::Ty::Unit,
        };

        let next_ty = prev_ty + ty;
        types.insert(k, next_ty);

        s = &s[end..];
    }

    info!("filename={}, took={}ms", filename, sw.elapsed_ms());

    Ok(types)
}

fn main() -> Result<()> {
    env_logger::init();

    let files_list = File::open("filelist")?;
    let files_list = BufReader::new(files_list);

    let outfile = File::create("out/out.rs")?;
    let mut outfile = BufWriter::new(outfile);

    let mut all_types = HashMap::<String, serde_gen::Ty>::new();

    for file in files_list.lines() {
        let file = file?;
        let types = match extract_types(&file) {
            Ok(types) => types,
            Err(_e) => {
                log::error!("failed to parse file, filename={}, e={}", file, _e);
                continue;
            }
        };

        for (k, ty) in types.into_iter() {
            let prev_ty = match all_types.get(&k) {
                Some(v) => v.clone(),
                None => serde_gen::Ty::Unit,
            };

            let next_ty = prev_ty + ty;
            all_types.insert(k, next_ty);
        }
    }

    let mut builder = serde_gen::TyBuilder::new();
    for (k, v) in all_types.into_iter() {
        let out_src = builder.build(&k, v);
        write!(outfile, "// {}\n", k)?;
        write!(outfile, "{}\n", out_src)?;
    }

    Ok(())
}
