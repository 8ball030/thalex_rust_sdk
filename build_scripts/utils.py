"""
Utils for building the Thalex Rust SDK from the OpenAPI spec.
"""

from pathlib import Path


WS_SPEC = Path("ws_spec_updated.json")
OUTPUT_PATH = Path("src/channels")


ALIASES = {
    "PriceIndexPayload": "Index",
    "PriceIndex": "Index",
    "AccountSummaryPayload": "AccountSummary",
    "TickerPayload": "Ticker",
    "SystemPayload": "SystemEvent",
    "UserInboxNotificationsPayload": "NotificationsNotificationsInner"
}

ENUMS = [
    "Delay",
]

PUBLIC_TAGS = [
    "subs_market_data",
    "subs_system",
]

def collect_all_tags_from_spec(spec):
    tags = set()
    for _, path_spec in spec["paths"].items():
        path_tags = path_spec.get("get", {}).get("tags", [])
        print("   Found tags:", path_tags)
        for tag in path_tags:
            tags.add(tag)
    return tags

def to_camel_case(snake_str):
    components = snake_str.split('_')
    return ''.join(x.title() for x in components)