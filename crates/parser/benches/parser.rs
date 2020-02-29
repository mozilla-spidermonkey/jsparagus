use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Read;

use bumpalo::Bump;
use jsparagus_parser::{parse_script, ParseOptions};

fn parser_bench(c: &mut Criterion) {
    let tests = &["simple", "__finStreamer-proto"];
    let mut programs = HashMap::new();

    programs.insert("simple", include_str!("./simple.js"));
    programs.insert(
        "__finStreamer-proto",
        include_str!("./__finStreamer-proto.js"),
    );

    c.bench_function_over_inputs(
        "parser_bench",
        move |b, &name| {
            let program = &programs[name];
            b.iter(|| {
                let allocator = &Bump::new();
                let options = ParseOptions::new();
                let _ = parse_script(allocator, program, &options);
            });
        },
        tests,
    );
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn real_js_parse_bench(c: &mut Criterion) {
    let real_js_path = "./benches/real-js";
    let count = 10;

    let paths = fs::read_dir(real_js_path).unwrap();
    let tests = paths
        .into_iter()
        .take(count)
        .map(|p| {
            let p = p.unwrap().path();
            let path = p.to_str().unwrap();
            read_file(path).unwrap()
        })
        .collect::<Vec<_>>();

    let options = ParseOptions::new();
    let mut allocator = Bump::new();

    c.bench_function("parser_real_js_bench", move |b| {
        b.iter(|| {
            for program in &tests {
                let _ = parse_script(&allocator, black_box(program), &options);
                allocator.reset();
            }
        })
    });
}

criterion_group!(benches, parser_bench, real_js_parse_bench);
criterion_main!(benches);
