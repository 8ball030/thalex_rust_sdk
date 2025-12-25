"""
Script to build RPC components.
"""

import json
from pathlib import Path
from pprint import pprint

from utils import collect_all_tags_from_spec

OPEN_API_SPEC = "openapi.json"

TAGS_TO_PROCESS = [
    "rpc_session_management",
    "rpc_trading",
]

def main():
    spec = json.loads(Path(OPEN_API_SPEC).read_text())
    tags = collect_all_tags_from_spec(spec)
    for tag in tags:
        print(tag)
    for tag in TAGS_TO_PROCESS:
        print(f"Processing tag: {tag}")
        process_tag(spec, tag)

def process_tag(spec, tag):
    print(f" Building RPC for tag: {tag}")
    paths = {}
    for path_name, path_spec in spec["paths"].items():
        for method, method_spec in path_spec.items():
            if tag in method_spec.get("tags", []):
                paths[path_name] = method_spec
    print(f"  Found {len(paths)} paths for tag {tag}")
    for path_name, path_spec in paths.items():
        print(f"   Path: {path_name} Method: {list(path_spec.keys())}")
        pprint(path_spec['requestBody'])
    
if __name__ == "__main__":
    main()
