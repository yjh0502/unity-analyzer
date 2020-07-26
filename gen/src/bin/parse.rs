use std::fs::File;
use std::io::{BufRead, BufReader};

use log::*;
use rayon::prelude::*;
use stopwatch::Stopwatch;

type Result<T> = std::result::Result<T, anyhow::Error>;
fn main() -> Result<()> {
    env_logger::init();

    let files_list = BufReader::new(File::open("filelist")?);
    let files_list = files_list
        .lines()
        .map(|v| v.map(|v| v.to_owned()))
        .collect::<std::io::Result<Vec<String>>>()?;

    let sw = Stopwatch::start_new();

    files_list
        .into_par_iter()
        .map(|file| -> Result<()> {
            debug!("file={}", file);

            let buf = gen::YamlBuf::from_filename(&file)?;
            for res in buf.iter() {
                let (_key, body) = res?;
                let _parsed = unity_yaml::Root::from_str(body);
                if let Err(e) = _parsed {
                    error!("filename={}\n{}\nerr={}", file, body, e);
                }
                //info!("parsed: {:?}", parsed);
            }
            Ok(())
        })
        .collect::<Vec<_>>();
    eprintln!("took={}ms", sw.elapsed_ms());

    Ok(())
}
