use anyhow::Result;
use log::*;
use std::{
    fs,
    io::{self, BufRead},
};

fn main() -> Result<()> {
    env_logger::init();

    let file = fs::File::open("types0")?;
    let lines = io::BufReader::new(file).lines();

    let mut skipped = 0;
    let mut n = 0;

    for line in lines {
        let line = line?;
        n += 1;

        if line.contains("_<")
            || line.starts_with("<")
            || line.starts_with("Conversion method")
            || line.starts_with("Native definition")
            || line.contains("PrivateImplementationDetails")
        {
            skipped += 1;
            continue;
        }

        if let Err(e) = il2cpp::parse_signature(&line) {
            info!("failed to parse, n={}, line={}, err={:?}", n, line, e);
            break;
        }
    }

    info!("skipped={}, total={}", skipped, n);

    Ok(())
}
