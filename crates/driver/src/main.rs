mod demo;

use env_logger;
use std::path::PathBuf;
use structopt::StructOpt;

// jemalloc is temporarily disabled due to a known upstream bug (macOS crashes
// in release builds): <https://github.com/gnzlbg/jemallocator/issues/136>
//
// use jemallocator::Jemalloc;
//
// #[global_allocator]
// static ALLOC: Jemalloc = Jemalloc;

#[derive(StructOpt, Debug)]
#[structopt(name = "driver")]
struct Opt {
    /// Print AST
    #[structopt(long)]
    ast: bool,

    /// Disassemble bytecode
    #[structopt(short = "D", long)]
    bytecode: bool,

    /// Debug print EmitResult
    #[structopt(long)]
    emit_result: bool,

    /// JavaScript (.js) file or directory to execute
    #[structopt(name = "PATH", parse(from_os_str))]
    path: Option<PathBuf>
}


fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    if let Some(path) = opt.path {
        match demo::parse_file_or_dir(&path) {
            Ok(stats) => {
                println!("{:#?}", stats);
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
        return;
    }

    demo::read_print_loop(demo::Verbosity {
        ast: opt.ast,
        bytecode: opt.bytecode,
        emit_result: opt.emit_result
    });
}
