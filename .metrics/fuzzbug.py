#!/usr/bin/python
import json
import os.path
from datetime import datetime

read_filename = 'count/fuzzbug.json'
write_since = 'badges/fuzzbug.json'
write_count = 'badges/fuzzbug_count.json'

days_since = None
open_fuzzbugs = 0
with open(read_filename, 'r+') as f:
    filedata = json.load(f)
    # the open fuzzbug count. Can be deleted
    open_fuzzbugs = len([x for x in filedata if x['closed_at'] == None])
    count = len(filedata)

    # the last time we saw a fuzzbug regardless of status
    if count > 0:
        dt_format =  "%Y-%m-%dT%H:%M:%SZ"
        fuzzbug_opened = filedata[count - 1]["created_at"]
        fuzzbug_date = datetime.strptime(fuzzbug_opened, dt_format)
        today = datetime.today()
        days_since = (today - fuzzbug_date).days

# Write days since last fuzzbug
data = {
    "schemaVersion": 1,
    "label": "Days since last FuzzBug",
    "message": days_since if days_since else "∞",
    "color": "green" if days_since == None else "yellow",
    "cacheSeconds": 1800,
}

with open(write_since, 'w') as f:
    json.dump(data, f, indent=4)

# Write fuzzbug count
data = {
    "schemaVersion": 1,
    "label": "Open FuzzBugs",
    "message": open_fuzzbugs,
    "color": "green" if open_fuzzbugs > 0 else "yellow",
    "cacheSeconds": 1800,
}

with open(write_count, 'w') as f:
    json.dump(data, f, indent=4)
