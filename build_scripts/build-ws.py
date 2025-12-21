"""
Script to build the WebSocket Subscriptions for the Thalex Rust SDK.
"""

import json
from pathlib import Path

from templates.subscriptions import func_template, file_template

WS_SPEC = Path("ws_spec_updated.json")
OUTPUT_PATH = Path("src/channels")


ALIASES = {
    "PriceIndex": "Index",
}

ENUMS = [
    "Delay",
]

def load_ws_spec():
    return json.loads(WS_SPEC.read_text())

def build_functions(spec, tag):
    functions = []
    for path_name, path_spec in spec["paths"].items():
        if tag not in path_spec.get("get", {}).get("tags", []):
            continue
        print(f"  path: {path_name}")
        split = [i for i in path_name.split("/") if i]
        channel_name = "_".join([f for f in split
                                 if not (f.startswith("{") and f.endswith("}"))
                                 ])
        print("     Extracted channel name:", channel_name)
        arg_names = []
        arg_types = []
        for part in split:
            if part.startswith("{") and part.endswith("}"):
                arg_name = part[1:-1]
                arg_names.append(arg_name)
                if arg_name.capitalize() in spec.get("components", {}).get("schemas", {}) and arg_name.capitalize() in ENUMS:
                    arg_types.append(arg_name.capitalize())
                else:
                    arg_types.append("&str")
        print("     Extracted args:", arg_names)
        response_model_ref = path_spec.get("get", {}).get("responses", {}).get("200", {}).get("content", {}).get("application/json", {}).get("schema", {}).get("$ref", "")
        response_model = response_model_ref.split("/")[-1] if response_model_ref else "UnknownModel"

        notification_model_ref = spec.get("components", {}).get("schemas", {}).get(response_model).get("properties", {}).get("notification", {}).get("$ref", "")
        notification_model = notification_model_ref.split("/")[-1] if notification_model_ref else "UnknownNotificationModel"

        if notification_model in ALIASES:
            notification_model = ALIASES[notification_model]

        func_args_string = ", ".join(f"{arg}: {arg_type}" for arg, arg_type in zip(arg_names, arg_types))
        if func_args_string:
            func_args_string += ","

        subscriptions_code = func_template.substitute(
            channel=channel_name,
            func_args=func_args_string,
            channel_args=".".join(["{" + i + "}" for i in arg_names]),
            response_model=response_model,
            notification_model=notification_model
        )
        functions.append(subscriptions_code)
    return "\n".join(functions)

def build_namespace_file(spec, functions, tag):
    models = set(
            spec.get("components", {}).get("schemas", {}).keys()
        )
    # replace with aliases
    for alias in ALIASES:
        if alias in models:
            models.remove(alias)
            models.add(ALIASES[alias])
    namespace_name = "".join(i.capitalize() for i in tag.replace("subs_", "").split("_"))

    file_content = file_template.substitute(
        functions=functions,
        models=", ".join(sorted(models)),
        tag=namespace_name + "Subscriptions"
    )
    return file_content

def collect_all_tags(spec):
    tags = set()
    for _, path_spec in spec["paths"].items():
        path_tags = path_spec.get("get", {}).get("tags", [])
        print("   Found tags:", path_tags)
        for tag in path_tags:
            tags.add(tag)
    return tags

if __name__ == "__main__":
    spec = load_ws_spec()
    tags = collect_all_tags(spec)
    print("Collected tags:", tags)
    for tag in tags:
        print("Processing tag:", tag)
        functions = build_functions(spec, tag)
        file_content = build_namespace_file(spec, functions, tag)
        file_suffix = tag.replace("subs_", "")
        (OUTPUT_PATH / f"namespaces/{file_suffix}.rs").write_text(file_content)

