"""
Script to build RPC components.
"""

import json
from pathlib import Path
from pprint import pprint

from utils import collect_all_tags_from_spec, to_camel_case
from templates.rpc import method_template, file_template, no_param_method_template
import subprocess
GENERATION_OUTPUT = "./generated"

OPEN_API_SPEC = "openapi.json"
NEW_MODELS = dict()
NEW_PATHS = dict()

RPC_SPEC_PATH = Path("rpc_spec_generated.json")
MAP_OPENAPI_TYPE_TO_RUST = {
    "string": "&str",
    "integer": "i64",
    "boolean": "bool",
    "array": "Vec<Value>",
    "number": "Decimal",
}

TAGS_TO_PROCESS = [
    "rpc_session_management",
    "rpc_trading",
    "rpc_market_data",
    "rpc_accounting",
    "rpc_conditional",
]


RPC_RESULT_IMPORT_ALIASES = {
    "InsertRpcResult": "OrderStatus",
    "BuyRpcResult": "OrderStatus",
    "SellRpcResult": "OrderStatus",
    "AmendRpcResult": "OrderStatus",
    "CancelRpcResult": "OrderStatus",
    "CancelAllRpcResult": "f64",
    "CancelSessionRpcResult": "Value",
    "TickerRpcResult": "Ticker",
    "TickersRpcResult": "Ticker",
    "IndexRpcResult": "Index",
    "InstrumentsRpcResult": "Instrument",
    "InstrumentRpcResult": "Instrument",
    "AllInstrumentsRpcResult": "Instrument",
    # accounting
    "RequiredMarginBreakdownRpcResult": "PortfolioMarginBreakdown",
    "AccountSummaryRpcResult": "AccountSummary",
    "PortfolioRpcResult": "PortfolioEntry",
    "RequiredMarginForOrderRpcResult": "MarginBreakdownWithOrder",
    # conditional order
    "CreateConditionalOrderRpcResult": "ConditionalOrder",
}

RETURN_MODEL_TO_VECTOR_ALIASES = {
    "InstrumentsRpcResult": "Vec<Instrument>",
    "TickersRpcResult": "Vec<Ticker>",
    "AllInstrumentsRpcResult": "Vec<Instrument>",
    # accounting
    "OpenOrdersRpcResult": "Vec<OrderStatus>",
    "PortfolioRpcResult": "Vec<PortfolioEntry>",
    "RequiredMarginForOrderRpcResult": "MarginBreakdownWithOrder",
    # conditional order
    "CreateConditionalOrderRpcResult": "ConditionalOrder",
    "ConditionalOrdersRpcResult": "Vec<ConditionalOrder>",
    "CancelConditionalOrderRpcResult": "Value", 
    "CancelAllConditionalOrdersRpcResult": "Value",

}

MODELS_TO_LIFT = [
    "RpcResponse",
    "RpcRequest",
    "RpcErrorResponse",
    "ErrorResponse",
    "EmptyObject",
    "InsertRequest",
    "RfqOrder",
    "OrderStatus",
    "OrderFill",
    "Index",
    "Ticker",
    "Instrument",
    # accounting
    "Rfq",
    "OrderHistory",
    "Trade",
    "PortfolioEntry",
    "DailyMark",
    "AccountSummary",
    "PortfolioMarginBreakdown",
    "MarginBreakdownWithOrder",
    # Conditional order
    "ConditionalOrder"
]


IMPORTS_TO_SKIP = [
    "CancelAllParams",
    "CancelSessionParams",
    "InstrumentsParams",
    "CancelSessionRpcResult",
    "CancelAllRpcResult",
    "f64",
    "Value",
    # accounting
    "OpenOrdersRpcResult",
    "PortfolioRpcResult",
    "RequiredMarginForOrderRpcResult",
    "AccountSummaryRpcResult",
    # Conditional order
    "CancelAllConditionalOrdersRpcResult",
    "CreateConditionalOrderRpcResult",
    "ConditionalOrdersRpcResult",
    "CancelConditionalOrderRpcResult"
]
base_imports = [
        "RpcErrorResponse",
        "OrderStatus",
        "Instrument",
        "Ticker",
        "Index",
    ]

def main():
    spec = json.loads(Path(OPEN_API_SPEC).read_text())
    tags = collect_all_tags_from_spec(spec)
    for tag in tags:
        print(tag)
    functions = []


    for tag in TAGS_TO_PROCESS:
        print(f"Processing tag: {tag}")
        functions, model_imports = process_tag(spec, tag)

        imports = base_imports + model_imports
        imports = list(set([RPC_RESULT_IMPORT_ALIASES.get(imp, imp) for imp in imports]))
        imports = [imp for imp in imports if imp not in IMPORTS_TO_SKIP]
        file_content = file_template.substitute(
            tag=to_camel_case(tag.replace("rpc_", "").capitalize())+"Rpc",
            functions="\n".join(functions),
            models=",\n    ".join(imports)
        )
        output_path = Path("src/rpc") / f"{tag.replace('rpc_', '')}.rs"
        print(f" Writing RPC file to: {output_path}")
        output_path.write_text(file_content)
    extract_all_enums()
    generate_rpc_spec(spec)



def extract_method_from_path_spec(path_spec):
    method = path_spec.get("requestBody", {}).get("content", {}).get("application/json", {}).get("schema", {}).get("allOf", {})[0]['properties']['method']['const']
    return method

def collapse_all_of(schemas):
    """Collapses allOf schemas into a single schema."""

    combined = {
        "type": "object",
        "properties": {},
        "required": []
    }
    for schema in schemas:
        if "properties" in schema:
            combined["properties"].update(schema["properties"])
        if "required" in schema:
            combined["required"].extend(schema["required"])
        if '$ref' in schema:
            ref_schema = get_original_schema_from_ref(schema['$ref'])
            if "properties" in ref_schema:
                combined["properties"].update(ref_schema["properties"])
            if "required" in ref_schema:
                combined["required"].extend(ref_schema["required"])
    return combined

def get_original_schema_from_ref(ref):
    """Given a $ref string, returns the original schema from the spec."""
    spec = json.loads(Path(OPEN_API_SPEC).read_text())
    parts = ref.lstrip("#/").split("/")
    schema_name = parts[-1]
    schema = spec["components"]["schemas"][schema_name]
    return schema


def extract_params_from_path_spec(path_spec):
    all_params = path_spec.get("requestBody", {}).get("content", {}).get("application/json", {}).get("schema", {}).get("allOf", {})
    combine_params = {
        "type": "object",
        "properties": {},
        "required": []
    }
    if len(all_params) ==3:
        params = all_params[2]['properties']['params']
        if 'required' in params:
            combine_params['required'].extend(params['required'])
        if "properties" in params:
            combine_params['properties'].update(params['properties'])
        elif "allOf" in params:
            params = collapse_all_of(params['allOf'])
            if 'required' in params:
                combine_params['required'].extend(params['required'])
            if "properties" in params:
                combine_params['properties'].update(params['properties'])
        elif '$ref' in params:
            ref_schema = get_original_schema_from_ref(params['$ref'])
            if 'required' in ref_schema:
                combine_params['required'].extend(ref_schema['required'])
            if "properties" in ref_schema:
                combine_params['properties'].update(ref_schema['properties'])
        else:
            raise ValueError(f"Unknown params schema structure: {params}")
    else:
        raise ValueError("Unexpected number of allOf entries in requestBody schema")

    return combine_params


def build_function_code(method, 
                        params, 
                        description="No description provided", 
                        response_model="UnknownModel", 
                        result_model="UnknownResultModel",
                        rpc_result_model="UnknownRpcResultModel",
                        return_model="UnknownReturnModel",
                        has_params=True
                        ):

    if return_model in RETURN_MODEL_TO_VECTOR_ALIASES:
        aliased_return_model = RETURN_MODEL_TO_VECTOR_ALIASES[return_model]
    else:
        aliased_return_model = RPC_RESULT_IMPORT_ALIASES.get(return_model, return_model)

    if has_params:
    
        function_code = method_template.substitute(
            response_model=response_model,
            description=description,
            method_name="_".join(method.split("/")[1:]),
            method=method,
            params=params,
            result_model=result_model,
            rpc_result_model=rpc_result_model,
            return_model=aliased_return_model,
            # params_json="{" + params_json_string + "}"
        )
    else:
        IMPORTS_TO_SKIP.append(params)
        function_code = no_param_method_template.substitute(
            response_model=response_model,
            description=description,
            method_name="_".join(method.split("/")[1:]),
            method=method,
            result_model=result_model,
            rpc_result_model=rpc_result_model,
            return_model=aliased_return_model,
            # params_json="{" + params_json_string + "}"
        )
    return function_code


def extract_response_from_path_spec(path_spec):
    responses = path_spec.get("responses", {})['default']['content']['application/json']['schema']['oneOf']
    # Ensure that one is error and the other is success
    error = None
    success = None
    for resp in responses:
        if resp['title'] == 'Error':
            error = resp
        else:
            success = resp
    if not error or not success:
        raise ValueError("Warning: Could not find both error and success responses")
    assert len(responses) == 2, "Expected exactly two response types"
    new_success = success.copy()
    # remove the allOf wrapper
    if 'allOf' in new_success:
        new_success = new_success['allOf'][1]

    # we add a requestId field
    new_success['properties']['id'] = {
        "type": "string",
        "description": "The request ID"
    }
    if '$ref' in new_success['properties']['result']:
        if new_success['properties']['result']['$ref'] == "#/components/schemas/Null":
            new_success['properties']['result']['$ref'] = "#/components/schemas/EmptyObject"

    return new_success


def generate_new_path_spec(response_model_name, path_spec, param_schema_name):
    """We just update the existing path spec to have the new response model."""
    new_path_spec = path_spec.copy()
    new_path_spec["responses"]["default"]["content"]["application/json"]["schema"]["oneOf"] = [
        {"$ref": f"#/components/schemas/{response_model_name}"},
        {"$ref": "#/components/schemas/RpcErrorResponse"}
    ]
    del new_path_spec['tags']
    del new_path_spec['operationId']
    # we update the requestBody to only have RpcRequest
    new_path_spec["requestBody"]["content"]["application/json"]["schema"] = {"$ref": f"#/components/schemas/{param_schema_name}"}

    # We add in a description for the default response
    new_path_spec["responses"]["default"]["description"] = "RPC response, either success or error."
    # We have to simulate that it should be a POST request
    return {"post": new_path_spec}




def generate_rpc_spec(original_spec):
    new_spec = original_spec.copy()
    new_spec["paths"] = NEW_PATHS
    new_spec["components"]["schemas"] = NEW_MODELS
    new_spec['info'] = {
        "title": "Thalex RPC API",
        "version": "1.0.0"
    }
    del new_spec['tags']
    del new_spec['x-tagGroups']
    RPC_SPEC_PATH.write_text(json.dumps(new_spec, indent=2))


def extract_all_enums():
    new_enums = dict()
    for model_name, model_schema in NEW_MODELS.items():
        for prop_name, prop_schema in model_schema.get("properties", {}).items():
            if prop_schema.get("type") == "string" and "enum" in prop_schema:
                enum_name = f"{to_camel_case(prop_name)}Enum"
                if enum_name not in new_enums:
                    new_enums[enum_name] = prop_schema["enum"]
                else:
                    if new_enums[enum_name] != prop_schema["enum"]:
                        enum_name = f"{model_name}{enum_name}"
                        assert enum_name not in new_enums, f"Enum name collision for {enum_name}"
                    new_enums[enum_name] = prop_schema["enum"]
                # Update the property to reference the new enum
                prop_schema.clear()
                prop_schema["$ref"] = f"#/components/schemas/{enum_name}"
    for enum_name, enum_values in new_enums.items():
        NEW_MODELS[enum_name] = {
            "type": "string",
            "enum": enum_values
        }



def process_tag(spec, tag):
    print(f" Building RPC for tag: {tag}")
    paths = {}
    for path_name, path_spec in spec["paths"].items():
        for method, method_spec in path_spec.items():
            if tag in method_spec.get("tags", []):
                paths[path_name] = method_spec
    print(f"  Found {len(paths)} paths for tag {tag}")

    [NEW_MODELS.update({model: spec["components"]["schemas"][model] for model in MODELS_TO_LIFT})]
    functions = []

    model_imports = []
    for path_name, path_spec in paths.items():
        method = extract_method_from_path_spec(path_spec)
        print(f"   Path: {path_name} Method: {method}")
        params = extract_params_from_path_spec(path_spec)
        result_model_schema = extract_response_from_path_spec(path_spec)
        result_model_name = to_camel_case(method.split("/")[-1]) + "Result"

        if result_model_name in NEW_MODELS:
            print(f"   Warning: Response model {result_model_name} already exists. Overwriting.")
            raise ValueError("Duplicate response model name")
        NEW_MODELS[result_model_name] = result_model_schema

        param_schema_name = to_camel_case(method.split("/")[-1]) + "Params"
        if param_schema_name in NEW_MODELS:
            print(f"   Warning: Param schema {param_schema_name} already exists. Overwriting.")
            raise ValueError("Duplicate param schema name")
        NEW_MODELS[param_schema_name] = params

        new_path_spec = generate_new_path_spec(result_model_name, path_spec, param_schema_name)
        NEW_PATHS["/" + path_name] = new_path_spec

        param_obj_name = to_camel_case(method.split("/")[-1]) + "Params"

        model_imports.append(result_model_name)
        model_imports.append(param_obj_name)
        model_imports.append(result_model_name.replace("Result", "Response"))
        model_imports.append(result_model_name.replace("Result", "RpcResult"))


        function_code = build_function_code(
            method, 
            param_obj_name,
            description=path_spec.get("summary", "No description provided"),
            response_model=result_model_name.replace("Result", "Response"),
            result_model=result_model_name,
            rpc_result_model="RpcResponse",
            return_model=result_model_name.replace("Result", "RpcResult"),
            has_params = len(params['properties']) > 0,
        )
        functions.append(function_code)
    # generate_rpc_spec(spec)
    return functions, model_imports




def generate_from_spec():
    """
    openapi-generator-cli generate \
           -i rpc_spec_generated.json \
           -g rust \
           -o ./generated --additional-properties=supportAsync=false,useSingleRequestParameter=true --global-property models
    """
    cmd = [
        "openapi-generator-cli",
        "generate",
        "-i",
        str(RPC_SPEC_PATH),
        "-g",
        "rust",
        "-o",
        GENERATION_OUTPUT,
        "--generate-alias-as-model",
        "--additional-properties=supportAsync=false,useSingleRequestParameter=true,avoidBoxedModels=true",
        "--global-property",
        "models"
    ]
    print("Running command:", " ".join(cmd))
    subprocess.run(cmd, check=True)
    
def clean_useless_files():
    gen_path = Path(GENERATION_OUTPUT) / "src" / "models"
    for file_path in gen_path.rglob("*.rs"):
        print("Cleaning file:", file_path)
        if file_path.name.startswith("_"):
            print("  renaming req file file")
            new_name = file_path.name.replace("_public_", "").replace("_private_", "")
            file_path.rename(file_path.parent / new_name)
            file_path = file_path.parent / new_name
        if file_path.name.endswith("_response_result.rs"):
            new_name = file_path.name.replace("_response_result", "_result")
            print("  renaming response file:", file_path)
            file_path.rename(file_path.parent / new_name)
            file_path = file_path.parent / new_name
        if file_path.name.endswith("_default_response.rs"):
            new_name = file_path.name.replace("_default_response", "_response")
            print("  renaming file:", file_path)
            file_path.rename(file_path.parent / new_name)
            file_path = file_path.parent / new_name
        if "_post_" in file_path.name:
            new_name = file_path.name.replace("_post_", "_")
            print("  renaming post file:", file_path)
            file_path.rename(file_path.parent / new_name)
            file_path = file_path.parent / new_name
        if "_result_result" in file_path.name:
            new_name = file_path.name.replace("_result_result", "_rpc_result")
            print("  renaming result_result file:", file_path)
            file_path.rename(file_path.parent / new_name)
            file_path = file_path.parent / new_name
        if file_path.name.endswith("_request.rs") or file_path.name.endswith("all_of_params.rs"):
            print("  deleting unused file:", file_path)
            file_path.unlink()
            continue
        # smole fix for the id content
        if file_path.name.endswith("_result.rs"):
            content = file_path.read_text()
            if "pub id: Option<String>" in content:
                content = content.replace("pub id: Option<String>", "pub id: Option<u64>")
                file_path.write_text(content)

    for file_path in (Path(GENERATION_OUTPUT) / "src" / "models").rglob("*.rs"):
        content = file_path.read_text()
        content = content \
            .replace("ResultResult", "RpcResult") \
            .replace("Public", "") \
            .replace("Private", "") \
            .replace("PostDefaultResponse", "Response")
            # .replace("PostRequest", "") \
        file_path.write_text(content)




if __name__ == "__main__":
    main()
    generate_from_spec()
    clean_useless_files()
