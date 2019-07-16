#!/bin/bash

set -eu

cd $(dirname "$0")/..

python3 -m js_parser.generate_js_parser_tables --target=rust --progress -o client/src/parser_generated.rs
