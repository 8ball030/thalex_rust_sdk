#!/usr/bin/env python3
"""
Fix broken array types in openapi-generator Rust output.
Usage: python fix_arrays.py ws_spec_updated.json generated/src/models
"""

import json
import sys
import os
from pathlib import Path

def find_array_schemas(spec_path):
    """Find all schemas that are arrays in the OpenAPI spec."""
    with open(spec_path, 'r') as f:
        spec = json.load(f)
    
    array_types = {}
    schemas = spec.get('components', {}).get('schemas', {})
    
    for name, schema in schemas.items():
        if schema.get('type') == 'array':
            items_ref = schema.get('items', {}).get('$ref', '')
            if items_ref:
                # Extract the type name from $ref
                item_type = items_ref.split('/')[-1]
                array_types[name] = item_type
                print(f"Found array type: {name} -> Vec<{item_type}>")
    
    return array_types

def fix_rust_file(file_path, schema_name, item_type):
    """Fix a single Rust model file."""
    content = f"""use serde::{{Deserialize, Serialize}};

/// {schema_name} : Array of {item_type}
pub type {schema_name} = Vec<crate::models::{item_type}>;
"""
    
    with open(file_path, 'w') as f:
        f.write(content)
    print(f"✓ Fixed {file_path}")

def main():
    if len(sys.argv) != 3:
        print("Usage: python fix_arrays.py <spec.json> <models_dir>")
        sys.exit(1)
    
    spec_path = sys.argv[1]
    models_dir = Path(sys.argv[2])
    
    if not os.path.exists(spec_path):
        print(f"Error: {spec_path} not found")
        sys.exit(1)
    
    if not models_dir.exists():
        print(f"Error: {models_dir} not found")
        sys.exit(1)
    
    print("Scanning OpenAPI spec for array types...")
    array_types = find_array_schemas(spec_path)
    
    if not array_types:
        print("No array types found!")
        return
    
    print(f"\nFixing {len(array_types)} array type(s)...")
    for schema_name, item_type in array_types.items():
        # Convert to snake_case for filename
        file_name = ''.join(['_' + c.lower() if c.isupper() else c for c in schema_name]).lstrip('_')
        file_path = models_dir / f"{file_name}.rs"
        
        if file_path.exists():
            fix_rust_file(file_path, schema_name, item_type)
        else:
            print(f"⚠ Warning: {file_path} not found, skipping")
    
    print("\n✨ Done! Array types fixed.")

if __name__ == '__main__':
    main()
