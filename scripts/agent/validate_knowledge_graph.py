#!/usr/bin/env python3
"""Validate the repository knowledge graph with no third-party dependencies."""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys
from typing import Any


NODE_TYPES = {
    "requirement", "component", "interface", "invariant", "decision",
    "risk", "control", "test", "runbook", "external",
}
EDGE_TYPES = {
    "owns", "depends_on", "calls", "publishes", "consumes", "implements",
    "constrained_by", "supersedes", "verifies", "mitigates", "monitors",
    "recovers", "exposes", "stores", "authenticates", "authorizes",
    "documented_by", "operated_by",
}
STATUSES = {"active", "deprecated", "proposed"}
CONFIDENCES = {"high", "medium", "low"}
PLACEHOLDER_RE = re.compile(r"\{\{[^{}]+\}\}")
NODE_ID_RE = re.compile(r"^[a-z][a-z0-9-]*:[a-z0-9][a-z0-9._/-]*$")
EDGE_ID_RE = re.compile(r"^edge:[a-z0-9][a-z0-9._/-]*$")


def fail(message: str) -> None:
    raise ValueError(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def has_placeholder(value: Any) -> bool:
    if isinstance(value, str):
        return PLACEHOLDER_RE.search(value) is not None
    if isinstance(value, list):
        return any(has_placeholder(item) for item in value)
    if isinstance(value, dict):
        return any(has_placeholder(item) for item in value.values())
    return False


def validate_evidence(value: Any, context: str) -> None:
    require(isinstance(value, list) and bool(value), f"{context}.evidence must be non-empty")
    require(all(isinstance(item, str) and item.strip() for item in value), f"{context}.evidence entries must be non-empty strings")
    require(len(value) == len(set(value)), f"{context}.evidence contains duplicates")


def validate(path: Path) -> tuple[int, int]:
    try:
        graph = json.loads(path.read_text(encoding="utf-8"))
    except OSError as error:
        fail(f"cannot read {path}: {error}")
    except json.JSONDecodeError as error:
        fail(f"invalid JSON in {path}: {error}")

    require(isinstance(graph, dict), "graph root must be an object")
    require(not has_placeholder(graph), "graph contains unresolved {{PLACEHOLDER}} values")
    require(graph.get("schemaVersion") == 1, "schemaVersion must be 1")
    require(isinstance(graph.get("project"), str) and bool(graph["project"].strip()), "project must be non-empty")
    nodes = graph.get("nodes")
    edges = graph.get("edges")
    require(isinstance(nodes, list), "nodes must be an array")
    require(isinstance(edges, list), "edges must be an array")

    node_ids: set[str] = set()
    for index, node in enumerate(nodes):
        context = f"nodes[{index}]"
        require(isinstance(node, dict), f"{context} must be an object")
        node_id = node.get("id")
        node_type = node.get("type")
        require(isinstance(node_id, str) and NODE_ID_RE.fullmatch(node_id) is not None, f"{context}.id is invalid")
        require(node_id not in node_ids, f"duplicate node id: {node_id}")
        node_ids.add(node_id)
        require(node_type in NODE_TYPES, f"{context}.type is invalid")
        require(node_id.startswith(f"{node_type}:"), f"{context}.id prefix must match type {node_type!r}")
        require(isinstance(node.get("name"), str) and bool(node["name"].strip()), f"{context}.name is required")
        require(isinstance(node.get("description"), str) and bool(node["description"].strip()), f"{context}.description is required")
        require(node.get("status") in STATUSES, f"{context}.status is invalid")
        if "confidence" in node:
            require(node["confidence"] in CONFIDENCES, f"{context}.confidence is invalid")
        validate_evidence(node.get("evidence"), context)

    edge_ids: set[str] = set()
    for index, edge in enumerate(edges):
        context = f"edges[{index}]"
        require(isinstance(edge, dict), f"{context} must be an object")
        edge_id = edge.get("id")
        require(isinstance(edge_id, str) and EDGE_ID_RE.fullmatch(edge_id) is not None, f"{context}.id is invalid")
        require(edge_id not in edge_ids, f"duplicate edge id: {edge_id}")
        edge_ids.add(edge_id)
        require(edge.get("type") in EDGE_TYPES, f"{context}.type is invalid")
        require(edge.get("from") in node_ids, f"{context}.from references unknown node {edge.get('from')!r}")
        require(edge.get("to") in node_ids, f"{context}.to references unknown node {edge.get('to')!r}")
        require(edge.get("from") != edge.get("to"), f"{context} must not be a self-edge")
        require(edge.get("status") in STATUSES, f"{context}.status is invalid")
        if "confidence" in edge:
            require(edge["confidence"] in CONFIDENCES, f"{context}.confidence is invalid")
        validate_evidence(edge.get("evidence"), context)

    return len(nodes), len(edges)


def main() -> int:
    if len(sys.argv) != 2:
        print(f"Usage: {Path(sys.argv[0]).name} PATH_TO_GRAPH_JSON", file=sys.stderr)
        return 2
    path = Path(sys.argv[1])
    try:
        node_count, edge_count = validate(path)
    except ValueError as error:
        print(f"Knowledge graph validation failed: {error}", file=sys.stderr)
        return 1
    print(f"Knowledge graph valid: {node_count} nodes, {edge_count} edges")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
