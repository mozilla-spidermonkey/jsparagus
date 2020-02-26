#!/usr/bin/python
import json
import os.path
import datetime

filename = 'count/tests_compiled.json'
if not os.path.isfile(filename):
    with open(filename, 'w') as f:
        json.dump([], f, indent=4) # initialize with an empty list

with open(filename, 'r+') as f:
    data = json.load(f)
    count_passed = float(int(os.environ['count_passed']))
    count_tests = float(int(os.environ['count_tests']))
    percentage = int((count_passed / count_tests) * 100)
    if len(data) == 0 or data[-1]["commit"] != os.environ['current_commit']:
        data.append({
            "commit": os.environ['current_commit'],
            "percentage": str(percentage) + "%"
        })
        f.seek(0)
        json.dump(data, f, indent=4)
        f.truncate()
