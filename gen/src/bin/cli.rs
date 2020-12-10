use argh::FromArgs;
use gen::*;
use log::*;
use serde_gen::*;
use std::fs::File;
use std::{
    collections::HashMap,
    io::{BufWriter, Write},
    path::PathBuf,
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
    Danglings(CommandDanglings),
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
#[argh(
    subcommand,
    name = "danglings",
    description = "find all dangling resources"
)]
struct CommandDanglings {
    #[argh(switch, short = 'c', description = "conservative")]
    conservative: bool,

    #[argh(option, short = 'i', description = "include")]
    include: Vec<String>,

    #[argh(option, short = 'o', description = "output filename")]
    output: String,

    #[argh(positional)]
    dir: PathBuf,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "parse", description = "parse")]
struct CommandParse {
    #[argh(switch, short = 'c', description = "conservative")]
    conservative: bool,

    #[argh(positional)]
    dir: PathBuf,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "ls", description = "ls")]
struct CommandListFiles {
    #[argh(positional)]
    dir: String,
}

fn cmd_typegen(cmd: CommandTypeGen) -> Result<()> {
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

fn cmd_danglings(v: CommandDanglings) -> Result<()> {
    info!("include={:?}", v.include);
    let idx = assetindex::AssetIndex::from_path(&v.dir)?;

    let sw = Stopwatch::start_new();
    let danglings = idx.danglings(v.include)?;
    info!(
        "danglings: count={}, took={}ms",
        danglings.len(),
        sw.elapsed_ms()
    );

    let file = File::create(v.output)?;
    let mut file = std::io::BufWriter::new(file);
    for path in danglings {
        write!(&mut file, "{}\n", path.display())?;
    }

    // it could took a long time to drop a full index, so just forget it
    std::mem::forget(idx);

    Ok(())
}

fn cmd_parse(v: CommandParse) -> Result<()> {
    let sw = Stopwatch::start_new();

    let idx = assetindex::AssetIndex::from_path(&v.dir)?;

    if true {
        let danglings = idx.danglings(Vec::new())?;
        let danglings_count = danglings.len();
        let mut file = File::create("dangling.log")?;
        for path in danglings {
            write!(&mut file, "{}\n", path.display())?;
        }
        info!("danglings.len()={}", danglings_count);
    }

    if false {
        idx.dbg_print_reverse_deps("50ce252a11bf8814899e9e09ae6a86c6");
    }

    if false {
        idx.dbg_print_deps("1d61e9e0099917e48895931752dc2d78");
    }

    if false {
        // prefab: progress
        idx.dbg_print_hierarchy("1ea564b91575e4d2092ac28fe9dd255e");
        // prefab: passage
        idx.dbg_print_hierarchy("335ed0c77db9c4e4087786cb78732ff4");
        // scene: main
        idx.dbg_print_hierarchy("1d61e9e0099917e48895931752dc2d78");
    }

    info!("took={}ms", sw.elapsed_ms());
    Ok(())
}

fn cmd_list_files(v: CommandListFiles) -> Result<()> {
    let path_list = assetindex::list_files(&v.dir)?;

    for path in path_list {
        println!("{}", path.display());
    }

    Ok(())
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("gen=info,cli=info"),
    )
    .init();

    let args: TopLevel = argh::from_env();

    match args.nested {
        SubCommands::TypeGen(v) => cmd_typegen(v),
        SubCommands::Danglings(v) => cmd_danglings(v),
        SubCommands::Parse(v) => cmd_parse(v),
        SubCommands::ListFiles(v) => cmd_list_files(v),
    }
}
