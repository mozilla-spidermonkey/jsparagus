#!/usr/bin/env python3

""" Extract opcodes from C++ header.

To use, pipe the output of this command to your clipboard,
then paste it into opcode.rs.
"""

import argparse
import os
import re
import sys

parser = argparse.ArgumentParser(description='Update opcode.rs')
parser.add_argument('PATH_TO_MOZILLA_CENTRAL',
                    help='Path to mozilla-central')
parser.add_argument('PATH_TO_JSPARAGUS',
                    help='Path to jsparagus')
args = parser.parse_args()

opcodes_path = os.path.join(args.PATH_TO_MOZILLA_CENTRAL,
                        'js', 'src', 'vm', 'Opcodes.h')
if not os.path.exists(opcodes_path):
    print('{} does not exist'.format(opcodes_path),
          file=sys.stderr)
    sys.exit(1)

util_path = os.path.join(args.PATH_TO_MOZILLA_CENTRAL,
                        'js', 'src', 'vm', 'BytecodeUtil.h')
if not os.path.exists(util_path):
    print('{} does not exist'.format(util_path),
          file=sys.stderr)
    sys.exit(1)

dest_path = os.path.join(args.PATH_TO_JSPARAGUS,
                         'crates/emitter/src/opcode.rs')
if not os.path.exists(dest_path):
    print('{} does not exist'.format(dest_path),
          file=sys.stderr)
    sys.exit(1)


def extract_opcodes(path):
    opcodes = []

    with open(path, 'r') as f:
        for line in f:
            line = line.strip()
            if line.startswith('MACRO(') and ',' in line:
                line = line[5:]
                if line.endswith(' \\'):
                    line = line[:-2]
                assert line.endswith(')')
                opcodes.append((" " * 16) + line + ",")

    return opcodes


def extract_flags(path):
    pat = re.compile(r'(JOF_[A-Z0-9_]+)\s=\s([^,]+),\s*/\*\s+(.*)\s+\*/')

    flags = []

    with open(path, 'r') as f:
        for line in f:
            m = pat.search(line)
            if not m:
                continue

            name = m.group(1)
            value = m.group(2)
            comment = m.group(3)

            if name == 'JOF_MODEMASK':
                continue

            flags.append({
                'name': name,
                'value': value,
                'comment': comment,
            })

    return flags


def format_opcodes(out, opcodes):
    for opcode in opcodes:
        out.write('{}\n'.format(opcode))


def format_flags(out, flags):
    for flag in flags:
        out.write('/// {}\n'.format(flag['comment']))
        out.write('const {}: u32 = {};\n'.format(flag['name'], flag['value']))
        out.write('\n')


def update(path, opcodes, flags):
    tmppath = '{}.tmp'.format(path)

    with open(path, 'r') as in_f:
        with open(tmppath, 'w') as out_f:
            state = 'normal'
            for line in in_f:
                if '@@@@ BEGIN OPCODES @@@@' in line:
                    state = 'opcodes'
                    out_f.write(line)
                    format_opcodes(out_f, opcodes)
                elif '@@@@ END OPCODES @@@@' in line:
                    assert state == 'opcodes'
                    state = 'normal'
                    out_f.write(line)
                elif '@@@@ BEGIN FLAGS @@@@' in line:
                    state = 'flags'
                    out_f.write(line)
                    format_flags(out_f, flags)
                elif '@@@@ END FLAGS @@@@' in line:
                    assert state == 'flags'
                    state = 'normal'
                    out_f.write(line)
                elif state == 'normal':
                    out_f.write(line)
            assert state == 'normal'

    os.replace(tmppath, path)


opcodes = extract_opcodes(opcodes_path)
flags = extract_flags(util_path)

update(dest_path, opcodes, flags)
