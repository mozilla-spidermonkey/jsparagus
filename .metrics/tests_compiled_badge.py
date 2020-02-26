#!/usr/bin/python
import json
import os.path

filename = 'badges/test-percentage-compiled.json'
count_passed = float(int(os.environ['count_passed']))
count_tests = float(int(os.environ['count_tests']))
percentage = int((count_passed / count_tests) * 100)

def get_color(percent):
    if percent > 95:
        return "green"
    elif percent > 10:
        return "yellow"
    else:
        return "red"

data = {
    "schemaVersion": 1,
    "label": "Percentage Passing",
    "message": str(percentage) + "%",
    "color": get_color(percentage),
}
with open(filename, 'w') as f:
    json.dump(data, f, indent=4)
