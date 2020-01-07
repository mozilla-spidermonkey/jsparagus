extern crate clap;
extern crate syn;

use std::fs::File;
use std::io::Read;
use clap::{ App, Arg };

fn main() {
    let matches = App::new("Collect information about handler functions")
        .args(&[
            Arg::with_name("INPUT.rs")
                .required(true)
                .help("Source file of handler struct definition."),
        ])
    .get_matches();

    let source_path = matches.value_of("INPUT.rs")
        .expect("Expected INPUT.rs");

    let mut source = String::new();
    File::open(source_path)
        .unwrap()
        .read_to_string(&mut source)
        .unwrap();

    let file = syn::parse_file(&source).expect("Parsing handler");

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

    println!("{{");
    println!("\"fallible-methods\": [");
    println!("{}", names.join(",\n"));
    println!("]");
    println!("}}");
}
