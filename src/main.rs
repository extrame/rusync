use anyhow::Error;
use clap::Parser;
use rusync::console_info::ConsoleProgressInfo;
use rusync::sync::SyncOptions;
use rusync::Syncer;
use std::path::PathBuf;
use std::process;

#[derive(Debug, Parser)]
#[clap(name = "rusync")]
struct Opt {
    #[clap(
        long = "no-perms",
        help = "Do not preserve permissions (no-op on Windows)"
    )]
    no_preserve_permissions: bool,

    #[clap(long = "err-list", help = "Write errors to the given file")]
    error_list_path: Option<PathBuf>,

    #[clap(parse(from_os_str))]
    source: PathBuf,

    #[clap(parse(from_os_str))]
    destination: PathBuf,
}

fn main() -> Result<(), Error> {
    let opt = Opt::parse();
    let mut source = opt.source.clone();
    let options;
    if !source.is_dir() {
        //exclude *
        //include source file name
        options = SyncOptions {
            preserve_permissions: !opt.no_preserve_permissions,
            file_names: vec![source.file_name().unwrap().to_str().unwrap().to_string()],
            exclude_list: vec![],
            include_list: vec![],
        };
        //change source to source parent
        source = source.parent().unwrap().to_path_buf();
    }else{
        options = SyncOptions {
            preserve_permissions: !opt.no_preserve_permissions,
            exclude_list: vec![],
            include_list: vec![],
            file_names: vec![],
        };
    }
    let destination = &opt.destination;

    let console_info = match opt.error_list_path {
        Some(err_file) => ConsoleProgressInfo::with_error_list_path(&err_file)?,
        None => ConsoleProgressInfo::new(),
    };
    let syncer = Syncer::new(&source, destination, options, Box::new(console_info));
    let stats = syncer.sync();
    match stats {
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
        Ok(stats) if stats.errors > 0 => {
            process::exit(1);
        }
        _ => {
            process::exit(0);
        }
    }
}
