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
import re
from typing import Any, Dict, List, Optional, Tuple, Iterable


HTTP_METHODS = {"GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"}


# ----------------------------- basic traversal ----------------------------- #

def extract_response_schemas(openapi_spec: Dict[str, Any]) -> Dict[str, Any]:
    """Extract response schemas per operation."""
    schemas: Dict[str, Any] = {}
    paths = openapi_spec.get("paths", {}) or {}

    for path, path_item in paths.items():
        if not isinstance(path_item, dict):
            continue

        for method, operation in path_item.items():
            if not isinstance(operation, dict):
                continue
            if method.upper() not in HTTP_METHODS:
                continue

            operation_id = operation.get("operationId", f"{method}_{path}")
            responses = operation.get("responses", {}) or {}
            response_schemas: Dict[str, Any] = {}

            for status_code, response in responses.items():
                if not isinstance(response, dict):
                    continue
                content = response.get("content", {}) or {}
                for media_type, media_schema in content.items():
                    if not isinstance(media_schema, dict):
                        continue
                    schema = (media_schema.get("schema") or {})
                    response_schemas.setdefault(status_code, {})
                    response_schemas[status_code][media_type] = {
                        "description": response.get("description", ""),
                        "schema": schema,
                    }

            if response_schemas:
                schemas[operation_id] = {
                    "path": path,
                    "method": method.upper(),
                    "tags": operation.get("tags", []) or [],
                    "summary": operation.get("summary", "") or "",
                    "responses": response_schemas,
                }

    return schemas


def resolve_ref(ref: str, openapi_spec: Dict[str, Any]) -> Any:
    """Resolve a local #/... $ref pointer."""
    if not isinstance(ref, str) or not ref.startswith("#/"):
        return None
    parts = ref[2:].split("/")
    cur: Any = openapi_spec
    for part in parts:
        if not isinstance(cur, dict):
            return None
        cur = cur.get(part)
    return cur


def expand_schema(schema: Any, openapi_spec: Dict[str, Any], visited: Optional[set] = None) -> Any:
    """Recursively expand $ref references so we can compare / transform structures."""
    if visited is None:
        visited = set()

    if not isinstance(schema, dict):
        return schema

    if "$ref" in schema:
        ref = schema["$ref"]
        if ref in visited:
            return {"$ref": ref, "_circular": True}
        visited.add(ref)
        resolved = resolve_ref(ref, openapi_spec)
        if resolved is None:
            return schema
        expanded = expand_schema(copy.deepcopy(resolved), openapi_spec, visited.copy())
        # preserve siblings next to $ref
        for k, v in schema.items():
            if k != "$ref":
                expanded[k] = v
        return expanded

    out: Dict[str, Any] = {}
    for k, v in schema.items():
        if isinstance(v, dict):
            out[k] = expand_schema(v, openapi_spec, visited.copy())
        elif isinstance(v, list):
            out[k] = [
                expand_schema(it, openapi_spec, visited.copy()) if isinstance(it, dict) else it
                for it in v
            ]
        else:
            out[k] = v
    return out


# ----------------------------- canonicalisation ---------------------------- #

def clean_schema(schema: Any) -> Any:
    """
    Remove non-structural fields so two schemas can be compared/deduped.

    Keep titles by default (helps readability in generated code), but ignore:
      - example/description
      - vendor extensions (x-*)
      - common custom keys ("with", "serde_as")
    """
    if not isinstance(schema, dict):
        return schema

    cleaned: Dict[str, Any] = {}
    skip = {"example", "description", "with", "serde_as"}

    for k, v in schema.items():
        if k in skip or k.startswith("x-"):
            continue
        if isinstance(v, dict):
            cleaned[k] = clean_schema(v)
        elif isinstance(v, list):
            cleaned[k] = [clean_schema(it) if isinstance(it, dict) else it for it in v]
        else:
            cleaned[k] = v

    return cleaned


def schemas_match(schema1: Dict[str, Any], schema2: Dict[str, Any]) -> bool:
    a = clean_schema(copy.deepcopy(schema1))
    b = clean_schema(copy.deepcopy(schema2))
    return json.dumps(a, sort_keys=True) == json.dumps(b, sort_keys=True)


def find_matching_schema(schema: Dict[str, Any], existing: Dict[str, Dict[str, Any]]) -> Optional[str]:
    for name, sch in (existing or {}).items():
        if not isinstance(sch, dict):
            continue
        if schemas_match(schema, sch):
            return name
    return None


def generate_schema_name(operation_id: str, suffix: str) -> str:
    name = (operation_id or "").replace("rest_", "").replace("/", "_")
    name = "".join(w.capitalize() for w in name.split("_") if w)
    return f"{name}{suffix}" if name else suffix


def is_component_ref(schema: Any) -> bool:
    """Check if a schema is a pure component reference."""
    if not isinstance(schema, dict):
        return False
    if "$ref" not in schema:
        return False
    ref = schema["$ref"]
    if not isinstance(ref, str):
        return False
    if not ref.startswith("#/components/schemas/"):
        return False
    # Pure ref: only has $ref, no other properties
    return len(schema) == 1


def preserve_component_refs(schema: Any) -> Any:
    """
    Deep copy a schema while preserving component $refs.
    This prevents existing component references from being expanded/lost.
    """
    if not isinstance(schema, dict):
        return schema
    
    # If this is a component ref, preserve it as-is
    if is_component_ref(schema):
        return copy.copy(schema)
    
    # Otherwise, recursively process
    out: Dict[str, Any] = {}
    for k, v in schema.items():
        if isinstance(v, dict):
            out[k] = preserve_component_refs(v)
        elif isinstance(v, list):
            out[k] = [
                preserve_component_refs(it) if isinstance(it, dict) else it
                for it in v
            ]
        else:
            out[k] = v
    return out


class SchemaInterner:
    """Deduplicate schemas by structural fingerprint, resolving local component $refs.

    Key features:
    - Fingerprint is computed on a fully-resolved view of the schema (local #/components/schemas refs expanded).
    - Non-structural keys are dropped (except title for success schemas) so cosmetic differences do not create duplicates.
    - Tag-aware interning prevents cross-category merges (e.g. success vs error) when the spec encodes both under 200.
    - For tag == "error", title is dropped from fingerprint to enable deduplication, and if an equivalent schema 
      already exists in components, we reuse it (preferring names like ErrorResponse).
    - For tag == "success", title is kept in fingerprint to prevent false deduplication of semantically different schemas.
    - PRESERVES existing component references to avoid breaking schema relationships.
    """

    _DROP_KEYS = {
        "example",
        "examples",
        "description",
        "deprecated",
        "with",
        "serde_as",
    }

    def __init__(self, components_schemas: Dict[str, Dict[str, Any]]):
        self.components = components_schemas
        self.created: Dict[str, Dict[str, Any]] = {}

        # (tag, fp) -> chosen name
        self._fp_to_name: Dict[Tuple[str, str], str] = {}

        # fp -> list of existing component schema names
        self._existing_by_fp: Dict[str, List[str]] = {}

        # caches - now keyed by (object_id, tag)
        self._schema_fp_cache: Dict[Tuple[int, str], str] = {}

        # index existing components by fingerprint
        for name, schema in (self.components or {}).items():
            if not isinstance(schema, dict):
                continue
            # We don't know if existing schemas are errors or not, so compute both
            fp = self.fingerprint(schema, tag="")
            self._existing_by_fp.setdefault(fp, []).append(name)
            self._fp_to_name.setdefault(("", fp), name)

    @staticmethod
    def _pick_existing_error_name(candidates: List[str]) -> str:
        # Prefer canonical names
        prefer = [n for n in candidates if "ErrorResponse" in n]
        if prefer:
            return sorted(prefer, key=len)[0]
        prefer = [n for n in candidates if "Error" in n or "error" in n]
        if prefer:
            return sorted(prefer, key=len)[0]
        return sorted(candidates, key=len)[0]

    def intern(self, schema: Dict[str, Any], preferred_name: str, tag: Optional[str] = None) -> str:
        # If the schema is already a direct component ref, preserve that identity.
        if is_component_ref(schema):
            ref = schema["$ref"]
            return ref.split("/")[-1]

        fp = self.fingerprint(schema, tag=tag)
        tag_key = (tag or "")

        # Hard rule for errors: reuse an equivalent existing error schema if present.
        if tag_key == "error":
            existing = self._existing_by_fp.get(fp)
            if existing:
                chosen = self._pick_existing_error_name(existing)
                self._fp_to_name[(tag_key, fp)] = chosen
                return chosen
            # Also: don't mint operation-derived names for errors
            preferred_name = "ErrorResponse"

        key = (tag_key, fp)
        existing = self._fp_to_name.get(key)
        if existing:
            return existing

        # Allocate a unique name
        name = preferred_name
        if name in self.components or name in self.created:
            i = 2
            while f"{preferred_name}{i}" in self.components or f"{preferred_name}{i}" in self.created:
                i += 1
            name = f"{preferred_name}{i}"

        # Store a cleaned copy (no title) for output
        stored = clean_schema(copy.deepcopy(schema))
        if isinstance(stored, dict):
            stored.pop("title", None)

        self.created[name] = stored
        # Make newly-created schemas available for $ref resolution during this run,
        # otherwise future fingerprints become ref-name sensitive and we mint duplicates.
        self.components[name] = stored
        self._existing_by_fp.setdefault(fp, []).append(name)
        self._fp_to_name[key] = name
        return name

    def fingerprint(self, schema: Any, tag: Optional[str] = None) -> str:
        # cache by object identity for speed within a run
        if isinstance(schema, dict):
            sid = id(schema)
            cache_key = (sid, tag or "")
            if cache_key in self._schema_fp_cache:
                return self._schema_fp_cache[cache_key]

        is_error = (tag == "error")
        norm = self._normalize(schema, ref_stack=(), is_error=is_error)
        s = json.dumps(norm, sort_keys=True, separators=(",", ":"))
        fp = hashlib.sha256(s.encode("utf-8")).hexdigest()

        if isinstance(schema, dict):
            cache_key = (id(schema), tag or "")
            self._schema_fp_cache[cache_key] = fp
        return fp

    def _normalize(self, node: Any, ref_stack: Tuple[str, ...], is_error: bool = False):
        if isinstance(node, list):
            return [self._normalize(x, ref_stack, is_error) for x in node]

        if not isinstance(node, dict):
            return node

        # resolve local component refs
        ref = node.get("$ref")
        if isinstance(ref, str) and ref.startswith("#/components/schemas/"):
            name = ref.split("/")[-1]
            if name in ref_stack:
                return {"$ref": ref}  # break cycles
            target = self.components.get(name)
            if isinstance(target, dict):
                return self._normalize(target, ref_stack + (name,), is_error)
            return {"$ref": ref}

        out: Dict[str, Any] = {}
        for k, v in node.items():
            # Drop title for errors (to enable deduplication), keep it for success schemas
            if k == "title" and is_error:
                continue
            if k in self._DROP_KEYS or k.startswith("x-"):
                continue
            if k == "required" and isinstance(v, list):
                out[k] = sorted([x for x in v if isinstance(x, str)])
                continue
            if k == "enum" and isinstance(v, list):
                try:
                    out[k] = sorted(v)
                except TypeError:
                    out[k] = v
                continue
            if k in ("oneOf", "anyOf", "allOf") and isinstance(v, list):
                branches = [self._normalize(x, ref_stack, is_error) for x in v]
                # order-insensitive canonicalization: sort branches by their json hash
                branches_sorted = sorted(
                    branches,
                    key=lambda b: hashlib.sha256(
                        json.dumps(b, sort_keys=True, separators=(",", ":")).encode("utf-8")
                    ).hexdigest(),
                )
                out[k] = branches_sorted
                continue
            out[k] = self._normalize(v, ref_stack, is_error)

        return out


def _slugify(name: str) -> str:
    s = re.sub(r"[^a-zA-Z0-9_]+", "_", (name or "").strip())
    s = re.sub(r"_+", "_", s).strip("_")
    if not s:
        return "field"
    if s[0].isdigit():
        s = f"f_{s}"
    return s.lower()


def _title_to_field_names(title: str, n: int) -> List[str]:
    if not title:
        return [f"v{i}" for i in range(n)]
    t = title.replace(" and ", ", ")
    parts = [p.strip() for p in t.split(",") if p.strip()]
    if len(parts) != n:
        return [f"v{i}" for i in range(n)]
    names = [_slugify(p) for p in parts]

    seen: Dict[str, int] = {}
    out: List[str] = []
    for nm in names:
        if nm in seen:
            seen[nm] += 1
            out.append(f"{nm}{seen[nm]}")
        else:
            seen[nm] = 1
            out.append(nm)
    return out


def _is_tuple_array(schema: Any) -> bool:
    if not isinstance(schema, dict):
        return False
    if schema.get("type") != "array":
        return False
    if not isinstance(schema.get("prefixItems"), list):
        return False
    mi = schema.get("minItems")
    ma = schema.get("maxItems")
    if isinstance(mi, int) and isinstance(ma, int) and mi == ma:
        return True
    return True  # treat presence of prefixItems as tuple-like


def _tuple_array_to_object(schema: Dict[str, Any]) -> Dict[str, Any]:
    prefix = schema.get("prefixItems")
    if not isinstance(prefix, list) or not prefix:
        return schema

    new_prefix = []
    for item in prefix:
        new_prefix.append(_transform_tuple_arrays(copy.deepcopy(item)) if isinstance(item, dict) else item)

    n = len(new_prefix)
    names = _title_to_field_names(schema.get("title", ""), n)

    props: Dict[str, Any] = {}
    required: List[str] = []
    for nm, item_schema in zip(names, new_prefix):
        props[nm] = item_schema
        required.append(nm)

    out: Dict[str, Any] = {
        "type": "object",
        "properties": props,
        "required": required,
        "additionalProperties": False,
    }
    if schema.get("title"):
        out["title"] = schema["title"]
    return out


def _transform_tuple_arrays(schema: Any) -> Any:
    if isinstance(schema, list):
        return [_transform_tuple_arrays(x) for x in schema]
    if not isinstance(schema, dict):
        return schema

    # Preserve component refs
    if is_component_ref(schema):
        return copy.copy(schema)

    if _is_tuple_array(schema):
        return _tuple_array_to_object(schema)

    out: Dict[str, Any] = {}
    for k, v in schema.items():
        if isinstance(v, (dict, list)):
            out[k] = _transform_tuple_arrays(v)
        else:
            out[k] = v
    return out


# -------------------------- general hoisting helper ------------------------ #

def _walk_schema_paths(schema: Any, path: Tuple[Any, ...] = ()) -> Iterable[Tuple[Tuple[Any, ...], Any]]:
    """Yield (path, node) for all dict/list nodes."""
    yield path, schema
    if isinstance(schema, dict):
        for k, v in schema.items():
            yield from _walk_schema_paths(v, path + (k,))
    elif isinstance(schema, list):
        for i, v in enumerate(schema):
            yield from _walk_schema_paths(v, path + (i,))


def _set_in_tree(root: Any, path: Tuple[Any, ...], value: Any) -> Any:
    """Return a deep-copied root with node at path replaced by value."""
    root2 = copy.deepcopy(root)
    cur = root2
    for step in path[:-1]:
        cur = cur[step]
    cur[path[-1]] = value
    return root2


def _should_hoist(node: Any) -> bool:
    """
    Decide whether an inline schema node is worth hoisting.
    We hoist only dict schemas that are not trivial and not already component refs.
    """
    if not isinstance(node, dict):
        return False
    if is_component_ref(node):
        return False
    # Don't hoist tiny scalar schemas.
    t = node.get("type")
    if t in {"string", "integer", "number", "boolean", "null"} and len(node.keys()) <= 2:
        return False
    return True


def _context_name(op_id: str, path: Tuple[Any, ...]) -> str:
    # Create a stable name from a traversal path like properties/mark/oneOf/0/items
    parts = []
    for p in path:
        if isinstance(p, int):
            parts.append(str(p))
        else:
            parts.append(_slugify(str(p)))
    tail = "_".join([p for p in parts if p])
    tail = tail[:80] if tail else "schema"
    return generate_schema_name(op_id, f"Inline{''.join(w.capitalize() for w in tail.split('_') if w)}")


def hoist_inline_schemas(
    schema: Any,
    op_id: str,
    interner: SchemaInterner,
) -> Any:
    """
    Hoist inline schemas inside unions/arrays into components and replace with $ref.
    This is conservative: it targets common hotspots:
      - items schemas under arrays
      - entries inside oneOf/anyOf/allOf lists
    PRESERVES existing component references.
    """
    if not isinstance(schema, (dict, list)):
        return schema

    # Preserve component refs
    if is_component_ref(schema):
        return copy.copy(schema)

    out = copy.deepcopy(schema)

    # Hoist array.items that are inline
    for p, node in list(_walk_schema_paths(out)):
        if not isinstance(node, dict):
            continue

        # items
        if node.get("type") == "array" and isinstance(node.get("items"), dict):
            items = node["items"]
            if _should_hoist(items):
                preferred = _context_name(op_id, p + ("items",))
                name = interner.intern(items, preferred)
                node["items"] = {"$ref": f"#/components/schemas/{name}"}

        # union branches
        for union_key in ("oneOf", "anyOf", "allOf"):
            branches = node.get(union_key)
            if isinstance(branches, list) and branches:
                new_branches = []
                changed = False
                for i, br in enumerate(branches):
                    if isinstance(br, dict) and _should_hoist(br):
                        preferred = _context_name(op_id, p + (union_key, i))
                        name = interner.intern(br, preferred)
                        new_branches.append({"$ref": f"#/components/schemas/{name}"})
                        changed = True
                    else:
                        new_branches.append(br)
                if changed:
                    node[union_key] = new_branches

    return out


# ----------------------- discriminator union rewrite ----------------------- #

def _norm_key(s: str) -> str:
    return re.sub(r"[^a-z0-9]+", "", (s or "").lower())


def _enum_values(schema: Dict[str, Any]) -> List[str]:
    ev = schema.get("enum")
    if isinstance(ev, list) and all(isinstance(x, str) for x in ev):
        return ev
    return []


def _find_discriminator_field(base_obj: Dict[str, Any]) -> Optional[str]:
    """
    Choose a discriminator-like field name in an object schema.
    Heuristic: a required string property with enum/const.
    """
    props = base_obj.get("properties") or {}
    required = set(base_obj.get("required") or [])
    best = None

    for name, sch in props.items():
        if name not in required:
            continue
        if not isinstance(sch, dict):
            continue
        if sch.get("type") != "string":
            continue
        if _enum_values(sch):
            return name  # strongest
        if "const" in sch and isinstance(sch["const"], str):
            best = name
    return best


def _find_union_property(base_obj: Dict[str, Any]) -> Optional[str]:
    """
    Find a property that is oneOf of arrays (or generally, complex oneOf).
    """
    props = base_obj.get("properties") or {}
    for name, sch in props.items():
        if not isinstance(sch, dict):
            continue
        oneof = sch.get("oneOf")
        if isinstance(oneof, list) and len(oneof) >= 2:
            # Prefer the exact problematic form: oneOf branches are arrays
            if all(isinstance(o, dict) and o.get("type") == "array" for o in oneof if isinstance(o, dict)):
                return name
            return name
    return None


def rewrite_object_property_union_to_discriminated_union(
    base_obj: Dict[str, Any],
    op_id: str,
    interner: SchemaInterner,
) -> Any:
    """
    If base_obj is an object with:
      - discriminator-like string field D (required)
      - property U whose schema has oneOf branches
    then rewrite to:
      oneOf: [Variant1, Variant2, ...]
      discriminator: { propertyName: D, mapping: {...} }

    Variant schemas are hoisted (interned) so the top-level result is stable.
    """
    if not isinstance(base_obj, dict) or base_obj.get("type") != "object":
        return base_obj

    props = base_obj.get("properties") or {}
    if not isinstance(props, dict) or not props:
        return base_obj

    disc = _find_discriminator_field(base_obj)
    union_prop = _find_union_property(base_obj)
    if not disc or not union_prop:
        return base_obj

    disc_schema = props.get(disc) if isinstance(props.get(disc), dict) else None
    union_schema = props.get(union_prop) if isinstance(props.get(union_prop), dict) else None
    if disc_schema is None or union_schema is None:
        return base_obj

    enum_vals = _enum_values(disc_schema)
    oneof = union_schema.get("oneOf")
    if not isinstance(oneof, list) or len(oneof) < 2:
        return base_obj

    # Map oneOf branches -> discriminator values.
    # 1) By title match (branch.title matches an enum value).
    # 2) Else, if enum values count matches branch count, map by order.
    mapping: Dict[str, str] = {}
    variant_refs: List[str] = []

    # Pre-compute title->branch
    titles = []
    for br in oneof:
        if isinstance(br, dict):
            titles.append(_norm_key(str(br.get("title", ""))))
        else:
            titles.append("")

    matched_by_title = False
    if enum_vals:
        for ev in enum_vals:
            nev = _norm_key(ev)
            if nev and nev in titles:
                matched_by_title = True
                break

    if not enum_vals and len(oneof) >= 2:
        # No enum -> can't safely build discriminator mapping
        return base_obj

    if not matched_by_title and enum_vals and len(enum_vals) != len(oneof):
        # Cannot map safely
        return base_obj

    # Base for naming
    base_variant = generate_schema_name(op_id, "ResultVariant")

    # Build variants
    for i, br in enumerate(oneof):
        if not isinstance(br, dict):
            return base_obj

        if matched_by_title:
            title = br.get("title") or f"Variant{i+1}"
            # pick enum value whose normalized form matches the title
            ev = None
            ntitle = _norm_key(str(title))
            for candidate in enum_vals:
                if _norm_key(candidate) == ntitle:
                    ev = candidate
                    break
            if ev is None:
                continue
        else:
            ev = enum_vals[i]

        variant_obj = copy.deepcopy(base_obj)
        vprops = copy.deepcopy(variant_obj.get("properties") or {})
        # Constrain discriminator
        vprops[disc] = {"type": "string", "enum": [ev]}
        # Replace union prop schema with the chosen branch
        vprops[union_prop] = br
        variant_obj["properties"] = vprops

        # Ensure required contains discriminator and union_prop (plus existing)
        req = list(variant_obj.get("required") or [])
        if disc not in req:
            req.append(disc)
        if union_prop not in req:
            req.append(union_prop)
        variant_obj["required"] = req

        # Make variants strict if base is strict
        if "additionalProperties" not in variant_obj:
            variant_obj["additionalProperties"] = False

        preferred = f"{base_variant}{i+1}"
        vname = interner.intern(variant_obj, preferred)
        vref = f"#/components/schemas/{vname}"
        variant_refs.append(vref)
        mapping[ev] = vref

    if len(variant_refs) < 2:
        return base_obj

    return {
        "oneOf": [{"$ref": r} for r in variant_refs],
        "discriminator": {
            "propertyName": disc,
            "mapping": mapping,
        },
    }


# ---------------------- extract result + error schemas ---------------------- #

def extract_result_and_error_schemas(openapi_spec: Dict[str, Any]) -> Tuple[Dict, Dict, Dict]:
    """
    Extract result and error schemas from all endpoints.
    Returns: (result_schemas, error_schemas, endpoint_mapping)
    PRESERVES existing component references.
    """
    operations = extract_response_schemas(openapi_spec)
    existing_schemas = (openapi_spec.get("components", {}) or {}).get("schemas", {}) or {}
    interner = SchemaInterner(existing_schemas)

    result_names: set[str] = set()
    error_names: set[str] = set()

    result_schemas: Dict[str, Any] = {}
    error_schemas: Dict[str, Any] = {}
    endpoint_mapping: Dict[str, Tuple[str, str]] = {}

    for op_id, op_data in operations.items():
        responses = op_data.get("responses", {}) or {}

        # Pick the "primary" success status in order.
        success_response = None
        for status_code in ("200", "201", "202", "204"):
            if status_code in responses:
                success_response = responses[status_code]
                break
        if not success_response:
            continue

        json_content = (success_response.get("application/json") or {})
        schema = (json_content.get("schema") or {})
        
        # *** KEY FIX: Don't expand the schema - work with raw structure to preserve refs ***
        # This keeps existing component $refs intact (e.g., Trade, OrderFill schemas)
        
        # Extract success and error from a conventional wrapper:
        # response.schema.oneOf: [{title:Success, properties:{result:...}}, {title:Error, ...}]
        success_schema = None
        error_schema = None

        if isinstance(schema, dict) and isinstance(schema.get("oneOf"), list):
            for option in schema["oneOf"]:
                if not isinstance(option, dict):
                    continue
                if option.get("title") == "Success":
                    props = option.get("properties") or {}
                    if isinstance(props, dict) and "result" in props:
                        success_schema = props["result"]
                elif option.get("title") == "Error":
                    error_schema = option
        
        # Also handle direct ref case (no oneOf wrapper)
        elif isinstance(schema, dict) and "$ref" in schema:
            # If the response is directly a ref, check if it's an error response
            ref_name = schema["$ref"].split("/")[-1] if isinstance(schema["$ref"], str) else ""
            if "error" in ref_name.lower():
                error_schema = schema
            else:
                success_schema = schema

        if not success_schema or not error_schema:
            continue

        # Preserve component refs throughout transformations
        success_schema = preserve_component_refs(success_schema)

        # Generalised transforms for codegen:
        # - tuple arrays -> objects
        # - hoist inline schemas inside unions/arrays
        # - discriminator union rewrite (property union -> object union)
        success_schema = _transform_tuple_arrays(success_schema)
        success_schema = hoist_inline_schemas(success_schema, op_id, interner)
        if isinstance(success_schema, dict):
            success_schema = rewrite_object_property_union_to_discriminated_union(
                success_schema, op_id, interner
            )
        # Re-run hoisting after rewrite (variants were interned; this catches nested leftovers)
        success_schema = hoist_inline_schemas(success_schema, op_id, interner)

        # Process result schema (interned)
        result_ref = None
        if isinstance(success_schema, dict):
            preferred = generate_schema_name(op_id, "Result")
            name = interner.intern(success_schema, preferred, tag="success")
            result_names.add(name)
            result_ref = f"#/components/schemas/{name}"

        # Process error schema (interned)
        error_ref = None
        if isinstance(error_schema, dict):
            preferred = generate_schema_name(op_id, "Error")
            name = interner.intern(error_schema, preferred, tag="error")
            error_names.add(name)
            error_ref = f"#/components/schemas/{name}"

        if result_ref and error_ref:
            endpoint_mapping[op_id] = (result_ref, error_ref)

    # materialize created schemas (shared pool) into result/error buckets for reporting
    for n in sorted(result_names):
        if n in interner.created:
            result_schemas[n] = interner.created[n]
    for n in sorted(error_names):
        if n in interner.created:
            error_schemas[n] = interner.created[n]

    # Ensure all hoisted/variant schemas are emitted even if not top-level result/error
    for n, sch in interner.created.items():
        if n not in result_schemas and n not in error_schemas:
            result_schemas[n] = sch

    return result_schemas, error_schemas, endpoint_mapping


# ----------------------------- write updated spec --------------------------- #

def write_updated_spec(openapi_spec: Dict[str, Any], output_file: str) -> None:
    updated_spec = copy.deepcopy(openapi_spec)

    result_schemas, error_schemas, endpoint_mapping = extract_result_and_error_schemas(openapi_spec)

    updated_spec.setdefault("components", {})
    updated_spec["components"].setdefault("schemas", {})

    # Add created schemas
    for name, schema in result_schemas.items():
        updated_spec["components"]["schemas"][name] = schema
    for name, schema in error_schemas.items():
        updated_spec["components"]["schemas"][name] = schema

    # Update endpoint responses to reference extracted schemas
    for path, path_item in (updated_spec.get("paths", {}) or {}).items():
        if not isinstance(path_item, dict):
            continue
        for method, operation in path_item.items():
            if not isinstance(operation, dict) or method.upper() not in HTTP_METHODS:
                continue
            op_id = operation.get("operationId")
            if not op_id or op_id not in endpoint_mapping:
                continue

            result_ref, error_ref = endpoint_mapping[op_id]
            responses = operation.get("responses", {}) or {}

            for status_code in ("200", "201", "202"):
                resp = responses.get(status_code)
                if not isinstance(resp, dict):
                    continue
                content = resp.get("content")
                if not isinstance(content, dict) or "application/json" not in content:
                    continue
                if not isinstance(content["application/json"], dict):
                    continue

                refs = [result_ref, error_ref]
                # drop duplicates while preserving order
                seen = set()
                oneof = []
                for r in refs:
                    if r in seen:
                        continue
                    seen.add(r)
                    oneof.append({"$ref": r})

                if len(oneof) == 1:
                    content["application/json"]["schema"] = oneof[0]
                else:
                    content["application/json"]["schema"] = {"oneOf": oneof}

    with open(output_file, "w") as f:
        json.dump(updated_spec, f, indent=2)

    print("=" * 80)
    print("UPDATED SPEC WRITTEN")
    print("=" * 80)
    print(f"Output file: {output_file}")
    print(f"Added {len(result_schemas)} new result schemas")
    print(f"Added {len(error_schemas)} new error schemas")
    print(f"Updated {len(endpoint_mapping)} endpoint responses")
    if result_schemas:
        print("\nNew Result Schemas:")
        for name in sorted(result_schemas.keys()):
            print(f"  - {name}")
    if error_schemas:
        print("\nNew Error Schemas:")
        for name in sorted(error_schemas.keys()):
            print(f"  - {name}")


def main() -> None:
    import os
    import sys

    in_file = "openapi.json"
    out_file = "openapi_updated.json"

    if len(sys.argv) >= 2:
        in_file = sys.argv[1]
    if len(sys.argv) >= 3:
        out_file = sys.argv[2]

    if not os.path.exists(in_file):
        print(f"Error: {in_file} not found")
        sys.exit(1)

    try:
        with open(in_file, "r") as f:
            openapi_spec = json.load(f)
    except json.JSONDecodeError as e:
        print(f"Error parsing JSON: {e}")
        sys.exit(1)

    print(f"Reading spec from: {in_file}\n")
    write_updated_spec(openapi_spec, out_file)


if __name__ == "__main__":
    main()