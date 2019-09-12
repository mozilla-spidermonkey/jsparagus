mod demo;

use jemallocator::Jemalloc;
use std::env;

#[global_allocator]
static ALLOC: Jemalloc = Jemalloc;

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
