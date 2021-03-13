use anyhow::Result;
use il2cpp::*;
use log::*;
use std::collections::hash_map::*;
use std::{
    fs,
    io::{self, BufRead},
};
use stopwatch::Stopwatch;

fn main() -> Result<()> {
    env_logger::init();

    let file = fs::File::open("types0")?;
    let lines = io::BufReader::new(file).lines();

    let sw = Stopwatch::start_new();

    let mut skipped = 0;
    let mut n = 0;

    let mut counter = HashMap::<String, usize>::new();

    let input_ignore_niddles = &["_<", "PrivateImplementationDetails"];
    let input_ignore_prefixes = &["<", "Conversion method", "Native definition"];

    let input_ignore = |line: &str| {
        for niddle in input_ignore_niddles {
            if line.contains(niddle) {
                return true;
            }
        }
        for prefix in input_ignore_prefixes {
            if line.starts_with(prefix) {
                return true;
            }
        }
        return false;
    };

    let output_ignore_prefixes = &["System", "T", "!"];
    let output_ignore = |ty: &str| {
        for prefix in output_ignore_prefixes {
            if ty.starts_with(prefix) {
                return true;
            }
        }
        false
    };

    for line in lines {
        let line = line?;
        n += 1;

        if input_ignore(&line) {
            skipped += 1;
            continue;
        }

        match parse_signature(&line) {
            Err(e) => {
                info!("failed to parse, n={}, line={}, err={:?}", n, line, e);
                break;
            }
            Ok(_res) => {
                // eprintln!("{:?}", _res);
                for ty in _res.significant_types() {
                    if output_ignore(&ty) {
                        continue;
                    }

                    match counter.entry(ty) {
                        Entry::Vacant(v) => {
                            v.insert(1);
                        }
                        Entry::Occupied(mut v) => {
                            *v.get_mut() += 1;
                        }
                    }
                    // eprintln!("{}", ty);
                }
            }
        }
    }

    let mut out = counter.into_iter().collect::<Vec<_>>();
    out.sort_by(|a, b| b.1.cmp(&a.1));

    println!("top occurences");
    for (ty, count) in out.iter().take(20) {
        println!("{:8} {}", count, ty);
    }

    out.reverse();
    println!("bottom occurences");
    for (ty, count) in out.iter().take(20) {
        println!("{:8} {}", count, ty);
    }

    info!(
        "skipped={}, total={}, elapsed={}ms",
        skipped,
        n,
        sw.elapsed_ms()
    );

    Ok(())
}
