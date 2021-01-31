use log::*;

use parse::*;
use stopwatch::Stopwatch;

fn main() -> Result<()> {
    env_logger::init();

    let sw = Stopwatch::start_new();

    let filename = "../files/Art02_Humans.unity";
    let file = std::fs::read_to_string(filename)?;

    let parsed = parse_str(&file)?;
    info!("objects={:?}", parsed.objects.len());

    let elapsed = sw.elapsed_ms();
    info!(
        "file={}, size={}, elapsed={}ms, {}/sec",
        filename,
        bytesize::ByteSize(file.len() as u64),
        elapsed,
        bytesize::ByteSize(file.len() as u64 * 1000 / elapsed as u64)
    );

    Ok(())
}
