"""
Read in the raw spec and generate the ws subscriptions file.
"""
import json

from pathlib import Path

import re

SCHEMA_PATH = Path("openapi.json")

TRANSIENT_WS_SPEC_PATH = Path("ws_spec.json")

PROCESSED_OPENAPI_PATH = Path("openapi.json")

CHANNEL_NAME_PATTERN = re.compile(r'Channel name: `([^`]+)`')
SCHEMA_PATTERN = re.compile(r'Notification payload:\n<SchemaDefinition schemaRef=\"#/components/schemas/([^"]+)"')


def extract_all_ws_tags():
    raw_spec = json.loads(SCHEMA_PATH.read_text())
    tags = raw_spec.get("tags", [])
    ws_tags = [tag for tag in tags if tag.get("name", "").startswith("subs_") and tag.get("description", None)]

    contents = []
    for tag in ws_tags:
        print(f"Tag: {tag['name']}")
        # print(f"Description: {tag['description']}")
        description = tag['description']
        # We have to use regex to extract the schema references
        channels = CHANNEL_NAME_PATTERN.findall(description)
        schemas = SCHEMA_PATTERN.findall(description)

        for channel, schema in zip(channels, schemas):
            contents.append({
                "tag": tag['name'],
                "channel": channel,
                "schema_ref": schema
            })

    return contents

def from_channel_to_path(channel_name):
    """
    ticker.<instrument>.<delay>  -> /ticker/{instrument}/{delay}
    """
    parts = channel_name.split('.')
    path_parts = []
    params = []
    for part in parts:
        if part.startswith('<') and part.endswith('>'):
            path_parts.append('{' + part[1:-1] + '}')
            params.append(part[1:-1])
        else:
            path_parts.append(part)
    return '/' + '/'.join(path_parts), params



def strip_and_combine_name(s):
    """Ensure reasonable name."""
    # first remove and join on .
    sub_name = ".".join(i for i in s.split("/") if i and not (i.startswith("{") and i.endswith("}")))
    # We also split on _ and capitalize
    sub_name = sub_name.replace("_", ".")

    sub_name = "".join(i.capitalize() for i in sub_name.split("."))

    return sub_name.split("<")[0]

def from_schema_ref_to_notification_schema_name_and_schema(schema_ref):

    raw_spec = json.loads(SCHEMA_PATH.read_text())
    components = raw_spec.get("components", {})
    schemas = components.get("schemas", {})
    if schema_ref not in schemas:
        raise(f"Schema ref {schema_ref} not found in components.schemas")
    notification_schema = schemas[schema_ref]

    # notification_schema_name = "".join([i.capitalize() for i in schema_ref.split(".")[0].split("_")]) + "Notification"

    notification_schema_name = strip_and_combine_name(schema_ref).replace("Payload", "Notification")
    if not notification_schema_name.endswith("Notification"):
        notification_schema_name += "Notification"


    extracted_payload_schema = notification_schema.get("properties", {})["notification"]
    extracted_payload_name   =  strip_and_combine_name(schema_ref )
    if not extracted_payload_name.endswith("Payload"):
        extracted_payload_name += "Payload"

    # we check if it is in our main schemas, if so can use that.
    # We replace the notification property with the actual payload schema
    processed_spec = json.loads(PROCESSED_OPENAPI_PATH.read_text())
    processed_components = processed_spec.get("components", {})
    processed_schemas = processed_components.get("schemas", {})
    if extracted_payload_name in processed_schemas:
        extracted_payload_schema = {"$ref": f"./openapi.json#/components/schemas/{extracted_payload_name}"}
    else:
        notification_schema["properties"]["notification"] = {"$ref": f"#/components/schemas/{extracted_payload_name}"}

    
    return notification_schema_name, notification_schema, extracted_payload_name, extracted_payload_schema


def update_ws_spec_with_path_and_schemas(path, path_spec, notification, notification_schema, 
                                         extracted_payload, extracted_payload_schema):
    to_update = json.loads(TRANSIENT_WS_SPEC_PATH.read_text())

    to_update["paths"][path] = path_spec

    for k, v in [[notification, notification_schema], [extracted_payload, extracted_payload_schema]]:
        if k not in to_update["components"]["schemas"]:
            to_update["components"]["schemas"][k] = v
    TRANSIENT_WS_SPEC_PATH.write_text(json.dumps(to_update, indent=4))


def from_path_and_params_to_path_spec(params, notification_schema_name, channel_name, tag):
    """
    Given params = ['instrument', 'delay']
    return the path spec for /ticker/{instrument}/{delay}
    """
    existing_spec = json.loads(TRANSIENT_WS_SPEC_PATH.read_text())
    path_spec = {
        "get": {
            "tags": [
                tag
            ],
            "summary": f"Subscribe to {channel_name} channel",
            "operationId": f"subscribe_{channel_name.replace('.', '_')}",
            "parameters": [],
            "responses": {
                "200": {
                    "description": "Successful subscription",
                    "content": {
                        "application/json": {
                            "schema": {
                                "$ref": f"#/components/schemas/{notification_schema_name}"
                            }
                        }
                    }
                }
            }
        }
    }
    # We check if there is a ref for the param already
    for param in params:
        if param.capitalize() in existing_spec.get("components", {}).get("schemas", {}):
            path_spec["get"]["parameters"].append({
                "name": param,
                "in": "path",
                "required": True,
                "schema": {
                    "$ref": f"#/components/schemas/{param.capitalize()}"
                }
            })
            continue
        path_spec["get"]["parameters"].append({
            "name": param,
            "in": "path",
            "required": True,
            "schema": {
                "type": "string"
            }
        })
    return path_spec


def scan_ws_spec_for_existing_paths():

    base_spec = json.loads(PROCESSED_OPENAPI_PATH.read_text())
    base_schemas = base_spec.get("components", {}).get("schemas", {})

    existing_spec = TRANSIENT_WS_SPEC_PATH.read_text()
    existing_spec_json = json.loads(existing_spec)
    all_spec_schemas = existing_spec_json.get("components", {}).get("schemas", {})

    ref_pattern = re.compile(r'#\/components\/schemas\/([^"]+)"')

    all_refs = set(ref_pattern.findall(existing_spec))
    replacements = []
    for ref_name in all_refs:
        schema_name = ref_name.split("/")[-1]
        if schema_name not in all_spec_schemas:
            if schema_name in base_schemas:
                print(f"Schema {schema_name} is missing in WS spec, adding from base spec.")
                existing_spec_json = json.loads(existing_spec)
                existing_spec_json["components"]["schemas"][schema_name] = base_schemas[schema_name]
                replacements.append(ref_name)
            else:
                print(f"Schema {schema_name} is missing in WS spec and not found in base spec!")
                raise Exception(f"Missing schema ref: {schema_name}")
    TRANSIENT_WS_SPEC_PATH.write_text(json.dumps(existing_spec_json, indent=4))
    # we now do a find and replace for all refs that were replaced
    updated_spec = TRANSIENT_WS_SPEC_PATH.read_text()
    for ref_name in replacements:
        updated_spec = updated_spec.replace(f'"#/components/schemas/{ref_name}"', f'"./openapi.json#/components/schemas/{ref_name}"')
    TRANSIENT_WS_SPEC_PATH.write_text(updated_spec)


def remove_consts_from_spec():
    """Read the spec, parse the specs, and remove any 'const' fields from schemas."""
    existing_spec = json.loads(TRANSIENT_WS_SPEC_PATH.read_text())
    for schema in existing_spec.get("components", {}).get("schemas", {}).values():
        for prop_name, prop_spec in schema.get("properties", {}).items():
            if "const" in prop_spec:
                print(f"Removing const from property {prop_name}")
                del prop_spec["const"]
    TRANSIENT_WS_SPEC_PATH.write_text(json.dumps(existing_spec, indent=4))

if __name__ == "__main__":
    all_data = extract_all_ws_tags()
    print(f"""Found {len(all_data)} WebSocket subscription channels in the schema:""")

    processed = 0
    for ix, data in enumerate(all_data):
        # we just process the subs_market_data for now
        print(f" Channel:     {data['channel']}")
        path, params = from_channel_to_path(data['channel'])
        print(f"   path:        {path}")
        print(f"   params:      {params}")
        print(f"   schema_ref:  {data['schema_ref']}")
        notification_schema_name, notification_schema, extracted_payload_name, extracted_payload_schema = from_schema_ref_to_notification_schema_name_and_schema(data['schema_ref'])
        print(f"   notification schema name: {notification_schema_name}")
        print(f"   extracted payload name: {extracted_payload_name}")
        # print(f"   schema:      {json.dumps(notification_schema, indent=4)}")
        path_spec = from_path_and_params_to_path_spec(params, notification_schema_name, data['channel'], data['tag'])

        update_ws_spec_with_path_and_schemas(path, path_spec, 
                                            notification_schema_name, 
                                            notification_schema,
                                            extracted_payload_name,
                                            extracted_payload_schema)
        processed += 1
    print(f"Processed {processed} WebSocket subscription channels.")
    scan_ws_spec_for_existing_paths()
    print("Scanned existing WS spec for missing schema refs.")
    remove_consts_from_spec()
    print("Removed const fields from WS spec.")






    
