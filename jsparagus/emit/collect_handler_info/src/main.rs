extern crate clap;
extern crate syn;

use std::fs::File;
use std::io::{Error, Read, Write};
use clap::{ App, Arg };

fn get_method_names(mut in_file: File) -> Vec<String> {
    let mut source = String::new();
    in_file.read_to_string(&mut source).unwrap();

    let file = syn::parse_file(&source)
        .expect("Syntax error found while parsing handler file");

    let mut names = Vec::new();

    for item in file.items {
        if let syn::Item::Impl(item) = item {
            for item in item.items {
                if let syn::ImplItem::Method(method) = item {
                    if let syn::Visibility::Public(_) = method.vis {
                        if let syn::ReturnType::Type(_, t) = method.sig.output {
                            if let syn::Type::Path(path) = *t {
                                if path.path.segments.first().unwrap().ident.to_string() == "Result" {
                                    names.push(format!("\"{}\"", method.sig.ident.to_string()));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    names
}

fn write_json(mut out_file: File, names: Vec<String>) -> Result<(), Error> {
    writeln!(out_file, "{{")?;
    writeln!(out_file, "\"fallible-methods\": [")?;
    writeln!(out_file, "{}", names.join(",\n"))?;
    writeln!(out_file, "],")?;
    writeln!(out_file, "\"parser-traits\": []")?;
    writeln!(out_file, "}}")?;

    Ok(())
}

fn main() {
    let matches = App::new("Collect information about handler functions")
        .args(&[
            Arg::with_name("INPUT.rs")
                .required(true)
                .help("Source file of handler struct definition."),
            Arg::with_name("OUTPUT.json")
                .required(true)
                .help("Target file to write the information."),
        ])
    .get_matches();

    let source_path = matches.value_of("INPUT.rs")
        .expect("Expected INPUT.rs");

    let target_path = matches.value_of("OUTPUT.json")
        .expect("Expected OUTPUT.json");

    let in_file = File::open(source_path).unwrap();
    let names = get_method_names(in_file);

    let out_file = File::create(target_path).unwrap();
    write_json(out_file, names).expect("Failed to write");
}
