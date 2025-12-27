#!/usr/bin/env python3
"""
Pre-process an OpenAPI spec to produce codegen-friendly response schemas.

What it does (generalised):
- Extracts "Success.result" and "Error" schemas from response oneOf wrappers.
- Interns (deduplicates) extracted schemas into components/schemas.
- Converts JSON-Schema tuple arrays (type=array + prefixItems) into objects so Rust generators emit structs.
- Hoists inline schemas found inside unions (oneOf/anyOf/allOf), including inline array items, into components and replaces with $ref.
- Rewrites a common problematic pattern for Rust generators:
    object property X is oneOf of arrays (or other complex schemas) and the object has a discriminator-like string field D.
  It lifts the union to the object level using a discriminator on D, producing stable, valid enums.
- PRESERVES existing component $refs to avoid breaking existing schema relationships.

Usage:
  ./pre-process.py            # reads openapi.json, writes openapi_updated.json
  ./pre-process.py in.json out.json
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

    # Remove required fields that cause issues
    updated_spec['components']['schemas']['AccountSummary']['properties']['cash']['items']['required'] \
        .remove('collateral_index_price')
    
    for item in [
        "start_price",
        "average_price"
    ]:
        updated_spec['paths']['private/account_breakdown']['rpc']['responses']['default']['content']['application/json']['schema'] \
            ['oneOf'][0]['allOf'][1]['properties']['result']['properties']['portfolio']['items']['required'] \
            .remove(item)
    # rename `session_perpetual_funding` to `realised_perpetual_funding` for `priveate/account_breakdown` RPC
    for specced, actual in [["session_perpetual_funding", "realised_perpetual_funding"]]:
        definition = updated_spec['paths']['private/account_breakdown']['rpc']['responses']['default']['content']['application/json']['schema'] \
            ['oneOf'][0]['allOf'][1]['properties']['result']['properties']['portfolio']['items']['properties'].pop(specced)
        updated_spec['paths']['private/account_breakdown']['rpc']['responses']['default']['content']['application/json']['schema'] \
            ['oneOf'][0]['allOf'][1]['properties']['result']['properties']['portfolio']['items']['properties'][actual] = definition
    
    # make `collateral_index_price` optional for `private/account_breakdown` RPC and remove from required
    updated_spec['paths']['private/account_breakdown']['rpc']['responses']['default']['content']['application/json']['schema'] \
            ['oneOf'][0]['allOf'][1]['properties']['result']['properties']['cash']['items']['properties']['collateral_index_price']['nullable'] = True
    updated_spec['paths']['private/account_breakdown']['rpc']['responses']['default']['content']['application/json']['schema'] \
            ['oneOf'][0]['allOf'][1]['properties']['result']['properties']['cash']['items']['required'] \
            .remove('collateral_index_price')
    

    # make `reject_reason` optional for `ConditionalOrder` model
    updated_spec['components']['schemas']['ConditionalOrder']['required'].remove('reject_reason')

    return updated_spec



def main():

    spec = load_spec(INPUT_SPEC_PATH)
    updated_spec = fix_schema(spec)
    save_spec(updated_spec, INPUT_SPEC_PATH)



if __name__ == "__main__":
    main()