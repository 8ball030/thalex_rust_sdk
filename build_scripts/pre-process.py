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

def fix_schema(spec: Dict[str, Any]) -> Dict[str, Any]:
    """
    Apply all pre-processing fixes to the OpenAPI spec.
    """
    updated_spec = copy.deepcopy(spec)
    return updated_spec



def main():

    spec = load_spec(INPUT_SPEC_PATH)
    updated_spec = fix_schema(spec)
    save_spec(updated_spec, INPUT_SPEC_PATH)



if __name__ == "__main__":
    main()
