#!/bin/bash

set -eu

cd $(dirname "$0")
python3 regenerate_parse_pgen_generated.py > parse_pgen_generated_NEW.py
mv parse_pgen_generated_NEW.py parse_pgen_generated.py
python3 parse_pgen.py  # run tests
python3 pgen.py --target=rust pgen.pgen > client/src/parser_generated.rs
(cd client && cargo build)
python3 test.py
