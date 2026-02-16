#!/usr/bin/env python3
"""
Pre-process an OpenAPI spec to produce codegen-friendly response schemas.
Resolves small issues with the original spec.
"""

import copy
import hashlib
import json
from pathlib import Path
import re
from typing import Any, Dict, List, Optional, Tuple, Iterable


INPUT_SPEC_PATH = Path("openapi.json")

def load_spec(path: Path) -> Dict[str, Any]:
    return json.loads(path.read_text())


def save_spec(spec: Dict[str, Any], path: Path) -> None:
    path.write_text(json.dumps(spec, indent=2))



def convert_numbers_integers(spec: Dict[str, Any]) -> Dict[str, Any]:
    """
    convert specific numbers to integers in the openapi spec as some of the numbers are decimal allowable and others
    are not, which causes issues with codegen. This function converts the numbers that are not decimal allowable to integers.
    """
    updated_spec = copy.deepcopy(spec)

    vars_types_to_be_set_to_integer = [
        "time_high",
        "time_low",
    ]
    
    # scan paths and find all the variables that are in the vars_types_to_be_set_to_integer list and set their type to integer
    for path, path_item in updated_spec.get("paths", {}).items():
        for method, operation in path_item.items():
            for parameter in operation.get("parameters", []):
                if parameter.get("name") in vars_types_to_be_set_to_integer:
                    parameter["schema"]["type"] = "integer"
                    parameter["schema"]["format"] = "int64"
                    print(f"Updated parameter '{parameter['name']}' in path '{path}' and method '{method}' to type 'integer'.")

        if "rpc" in path_item:
            if "requestBody" in path_item["rpc"]:
                for content_type, content in path_item["rpc"]["requestBody"].get("content", {}).items():
                    schema = content.get("schema", {})

                    for part in schema.get("allOf", []):
                        props = part.get("properties", {})
                        params = props.get("params", {})
                        if params.get("type") != "object":
                            continue
                        
                        for param_name, param_value in params.get("properties", {}).items():
                            if param_name in vars_types_to_be_set_to_integer:
                                param_value["type"] = "integer"
                                param_value["format"] = "int64"
                                print(
                                    f"Updated parameter '{param_name}' in rpc request body of path '{path}' to type 'integer'."
                                )

    for component_name, component in updated_spec.get("components", {}).get("schemas", {}).items():
        for property_name, property in component.get("properties", {}).items():
            if property_name in vars_types_to_be_set_to_integer:
                property["type"] = "integer"
                property["format"] = "int64"
                print(f"Updated property '{property_name}' in component '{component_name}' to type 'integer'.")

    return updated_spec
    



def fix_schema(spec: Dict[str, Any]) -> Dict[str, Any]:
    """
    Apply all pre-processing fixes to the OpenAPI spec.
    """
    updated_spec = copy.deepcopy(spec)

    updated_spec = convert_numbers_integers(updated_spec)
    return updated_spec



def main():

    spec = load_spec(INPUT_SPEC_PATH)
    updated_spec = fix_schema(spec)
    save_spec(updated_spec, INPUT_SPEC_PATH)



if __name__ == "__main__":
    main()
