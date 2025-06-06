import json
import re
from pathlib import Path

xml_path = Path("data/maps/city_giant.osm")

tags = {} 

with open(xml_path, mode="r", encoding="utf8") as f: 
    for line in f: 
        line = line.strip() 
        
        results = re.findall(pattern=r"<tag k=\"(.*?)\" v=\"(.*?)\"", string=line)
        
        if len(results) == 0: 
            continue
        elif len(results) == 1: 
            key, value = results[0] 
            
            if key in tags: 
                tags[key].add(value) 
            else: 
                tags[key] = set([value])
        else: 
            print(results)
            
tag_path = Path("results/tag_analysis.json")

converted_tags = {key: list(value) for key, value in tags.items()}

print(len(converted_tags.keys()))

with open(tag_path, "w") as json_file: 
    json.dump(converted_tags, json_file, indent=4)