PY_OUT = js_parser/parser_tables.py
RS_OUT = client/src/parser_generated.rs
PYTHON = python3

all: $(PY_OUT) $(RS_OUT)

# Incomplete list of files that contribute to the dump file.
SOURCE_FILES = \
jsparagus/gen.py \
jsparagus/emit.py \
js_parser/generate_js_parser_tables.py \
js_parser/parse_esgrammar.py \
js_parser/es-simplified.esgrammar

DUMP_FILE = js_parser/parser_generated.jsparagus_dump

$(DUMP_FILE): $(SOURCE_FILES)
	$(PYTHON) -m js_parser.generate_js_parser_tables --progress -o $@

$(PY_OUT): $(DUMP_FILE)
	$(PYTHON) -m js_parser.generate_js_parser_tables --progress -o $@ $(DUMP_FILE)

$(RS_OUT): $(DUMP_FILE)
	$(PYTHON) -m js_parser.generate_js_parser_tables --progress -o $@ $(DUMP_FILE)

check: $(PY_OUT)
	./test.sh

jsdemo: $(PY_OUT)
	$(PYTHON) -m js_parser.try_it

.PHONY: all check jsdemo
