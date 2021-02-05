use log::*;

use parse::*;
use stopwatch::Stopwatch;

fn main() -> Result<()> {
    env_logger::init();

    let sw = Stopwatch::start_new();

    let args: Vec<String> = std::env::args().collect();
    let filename = &args[0];

    let file = std::fs::read_to_string(filename)?;

    let parsed = parse_str(&file)?;
    info!("objects={:?}", parsed.objects.len());
    for object in parsed.objects {
        for child in object.iter() {
            debug_print_item(child, 0);
        }
    }

    let elapsed = sw.elapsed_ms();
    info!(
        "file={}, size={}, elapsed={}ms, {}/sec",
        filename,
        bytesize::ByteSize(file.len() as u64),
        elapsed,
        bytesize::ByteSize(file.len() as u64 * 1000 / elapsed.max(1) as u64)
    );

    Ok(())
}
