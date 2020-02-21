mod demo;

use std::env;

// jemalloc is temporarily disabled due to a known upstream bug (macOS crashes
// in release builds): <https://github.com/gnzlbg/jemallocator/issues/136>
//
// use jemallocator::Jemalloc;
//
// #[global_allocator]
// static ALLOC: Jemalloc = Jemalloc;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => demo::read_print_loop(),
        2 => match demo::parse_file_or_dir(&args[1]) {
            Ok(stats) => {
                println!("{:#?}", stats);
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        },
        _ => eprintln!("usage: parser [FILE/DIR]"),
    }
}
