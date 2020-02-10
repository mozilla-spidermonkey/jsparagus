#!/usr/bin/python
import json
import os.path
from datetime import datetime

read_filename = 'count/fuzzbug.json'
write_since = 'badges/since-last-fuzzbug.json'

days_since = None
with open(read_filename, 'r') as f:
    filedata = json.load(f)
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
    "message": days_since if days_since else "Forever",
    "color": "green" if days_since == None else "yellow",
    "cacheSeconds": 1800,
}

