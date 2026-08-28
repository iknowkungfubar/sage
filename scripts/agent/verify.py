#!/usr/bin/env python3
"""Run repository-defined verification gates deterministically.

The configuration uses argv arrays: commands are not interpreted by a shell.
If shell composition is genuinely required, configure it explicitly as, for
example, ["bash", "-lc", "command && command"]. Reports are ephemeral under
.agent/reports by default and should remain gitignored.
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys
import time
from typing import Any


PLACEHOLDER_RE = re.compile(r"\{\{[^{}]+\}\}")
SECRET_PATTERNS = (
    re.compile(r"(?i)(authorization\s*:\s*(?:bearer|basic)\s+)[^\s]+"),
    re.compile(r"(?i)((?:api[_-]?key|access[_-]?token|client[_-]?secret|password)\s*[=:]\s*)[^\s,;]+"),
    re.compile(r"\b(?:ghp|github_pat|sk|xox[baprs]|AKIA)[A-Za-z0-9_\-]{12,}\b"),
)

ALLOWED_COMMANDS = {
    ("python3", "scripts/agent/validate_knowledge_graph.py", "docs/knowledge/graph.json"),
    ("cargo", "fmt", "--all", "--", "--check"),
    ("cargo", "check", "--workspace"),
    ("cargo", "test", "--workspace"),
    ("cargo", "build", "--workspace"),
    ("cargo", "clippy", "--workspace", "--all-targets", "--all-features", "--", "-D", "warnings"),
}

EXPECTED_PROFILES = {
    "quick": ["knowledge-graph", "format", "check", "unit"],
    "pr": ["knowledge-graph", "format", "check", "unit", "build", "clippy"],
}

TRUSTED_EXECUTABLES = {
    "python3": ("/usr/bin/python3", "/usr/local/bin/python3"),
    "cargo": ("/usr/bin/cargo", "/usr/local/bin/cargo", str(Path.home() / ".cargo/bin/cargo")),
}
EXPECTED_GATE_COMMANDS = {
    "knowledge-graph": ("python3", "scripts/agent/validate_knowledge_graph.py", "docs/knowledge/graph.json"),
    "format": ("cargo", "fmt", "--all", "--", "--check"),
    "check": ("cargo", "check", "--workspace"),
    "unit": ("cargo", "test", "--workspace"),
    "build": ("cargo", "build", "--workspace"),
    "clippy": ("cargo", "clippy", "--workspace", "--all-targets", "--all-features", "--", "-D", "warnings"),
}

class ConfigError(Exception):
    """Configuration is invalid or unsafe to execute."""


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--config", default="governance/project.json", help="Path to project governance JSON")
    parser.add_argument("--profile", default="quick", help="Verification profile name")
    parser.add_argument("--list", action="store_true", help="List configured gates and profiles without executing")
    parser.add_argument("--report", help="Override JSON report path")
    return parser.parse_args()


def redact(text: str) -> str:
    result = text
    for pattern in SECRET_PATTERNS:
        if pattern.groups:
            result = pattern.sub(lambda match: f"{match.group(1)}[REDACTED]", result)
        else:
            result = pattern.sub("[REDACTED]", result)
    return result


def find_placeholders(value: Any, path: str = "$") -> list[str]:
    found: list[str] = []
    if isinstance(value, str) and PLACEHOLDER_RE.search(value):
        found.append(path)
    elif isinstance(value, list):
        for index, item in enumerate(value):
            found.extend(find_placeholders(item, f"{path}[{index}]"))
    elif isinstance(value, dict):
        for key, item in value.items():
            found.extend(find_placeholders(item, f"{path}.{key}"))
    return found


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ConfigError(message)


def load_and_validate(config_path: Path) -> dict[str, Any]:
    try:
        raw = config_path.read_bytes()
    except OSError as error:
        raise ConfigError(f"Cannot read config {config_path}: {error}") from error
    try:
        config = json.loads(raw)
    except json.JSONDecodeError as error:
        raise ConfigError(f"Invalid JSON in {config_path}: {error}") from error

    require(isinstance(config, dict), "Configuration root must be an object")
    placeholders = find_placeholders(config)
    require(not placeholders, f"Unresolved template placeholders at: {', '.join(placeholders)}")
    require(config.get("schemaVersion") == 1, "schemaVersion must be 1")
    require(isinstance(config.get("project"), str) and bool(config["project"].strip()), "project must be non-empty")
    require(isinstance(config.get("root"), str) and bool(config["root"].strip()), "root must be non-empty")
    require(isinstance(config.get("gates"), list) and bool(config["gates"]), "gates must be a non-empty array")
    require(isinstance(config.get("profiles"), dict) and bool(config["profiles"]), "profiles must be a non-empty object")

    gate_ids: set[str] = set()
    for index, gate in enumerate(config["gates"]):
        prefix = f"gates[{index}]"
        require(isinstance(gate, dict), f"{prefix} must be an object")
        gate_id = gate.get("id")
        require(isinstance(gate_id, str) and re.fullmatch(r"[a-z][a-z0-9-]*", gate_id) is not None, f"{prefix}.id is invalid")
        require(gate_id not in gate_ids, f"Duplicate gate id: {gate_id}")
        gate_ids.add(gate_id)
        require(isinstance(gate.get("description"), str) and bool(gate["description"].strip()), f"{prefix}.description is required")
        command = gate.get("command")
        require(isinstance(command, list) and bool(command), f"{prefix}.command must be a non-empty argv array")
        require(gate_id in EXPECTED_GATE_COMMANDS and tuple(command) == EXPECTED_GATE_COMMANDS[gate_id], f"{prefix} does not match the approved gate for {gate_id!r}")
        require(isinstance(gate.get("required"), bool), f"{prefix}.required must be boolean")
        timeout = gate.get("timeoutSeconds")
        require(isinstance(timeout, int) and not isinstance(timeout, bool) and 1 <= timeout <= 7200, f"{prefix}.timeoutSeconds must be 1..7200")
        if "workingDirectory" in gate:
            require(isinstance(gate["workingDirectory"], str) and bool(gate["workingDirectory"].strip()), f"{prefix}.workingDirectory is invalid")
        require(gate.get("workingDirectory", ".") == ".", f"{prefix}.workingDirectory must be repository root")
        if "environment" in gate:
            environment = gate["environment"]
            require(isinstance(environment, dict), f"{prefix}.environment must be an object")
            require(not environment, f"{prefix}.environment must be empty; gate environments are not trusted")

    require(config["profiles"] == EXPECTED_PROFILES, "profiles must match the approved repository verification profiles")
    require({tuple(gate["command"]) for gate in config["gates"]} == ALLOWED_COMMANDS, "gates must match the approved repository verification commands")
    require(all(gate["required"] for gate in config["gates"]), "all approved repository gates must be required")
    gate_by_id = {gate["id"]: gate for gate in config["gates"]}
    for profile_name, profile_gate_ids in config["profiles"].items():
        require(isinstance(profile_name, str) and bool(profile_name), "Profile names must be non-empty strings")
        require(isinstance(profile_gate_ids, list) and bool(profile_gate_ids), f"Profile {profile_name!r} must be non-empty")
        require(len(profile_gate_ids) == len(set(profile_gate_ids)), f"Profile {profile_name!r} contains duplicate gates")
        unknown = [gate_id for gate_id in profile_gate_ids if gate_id not in gate_ids]
        require(not unknown, f"Profile {profile_name!r} references unknown gates: {', '.join(unknown)}")
        require(all(gate_by_id[gate_id]["required"] for gate_id in profile_gate_ids), f"Profile {profile_name!r} contains advisory gates")

    protected_paths = config.get("protectedPaths", [])
    require(isinstance(protected_paths, list) and protected_paths, "protectedPaths must be a non-empty array")
    required_protected_paths = {"governance/project.json", "scripts/agent/verify.py"}
    require(required_protected_paths.issubset(protected_paths), "protectedPaths must include governance/project.json and scripts/agent/verify.py")
    for protected_path in protected_paths:
        require(isinstance(protected_path, str) and protected_path, "protectedPaths entries must be non-empty strings")
        path = Path(protected_path)
        require(not path.is_absolute() and ".." not in path.parts, f"protected path escapes repository: {protected_path!r}")
        require(not any(character in protected_path for character in ":*?["), f"protected path must be literal: {protected_path!r}")
    integrity = config.get("integrity", {})
    require(isinstance(integrity, dict) and integrity.get("failOnTrackedChangeDuringVerification") is True, "integrity must require tracked-change failure")

    config["_digest"] = hashlib.sha256(raw).hexdigest()
    return config


def run_git(repo_root: Path, args: list[str]) -> subprocess.CompletedProcess[str] | None:
    try:
        return subprocess.run(
            ["git", *args],
            cwd=repo_root,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=30,
            check=False,
        )
    except (OSError, subprocess.SubprocessError):
        return None


def candidate_state(repo_root: Path) -> dict[str, Any]:
    head = run_git(repo_root, ["rev-parse", "HEAD"])
    status = run_git(repo_root, ["status", "--porcelain=v1", "-uall"])
    return {
        "head": head.stdout.strip() if head and head.returncode == 0 else None,
        "dirty": bool(status and status.returncode == 0 and status.stdout),
        "status": redact(status.stdout.rstrip()) if status and status.returncode == 0 else None,
    }


def repository_state_digest(repo_root: Path) -> str | None:
    diff = run_git(repo_root, ["diff", "--binary", "HEAD", "--"])
    status = run_git(repo_root, ["status", "--porcelain=v1", "-uall"])
    if not diff or not status or diff.returncode != 0 or status.returncode != 0:
        return None
    hasher = hashlib.sha256()
    hasher.update(diff.stdout.encode("utf-8", errors="replace"))
    hasher.update(b"\0STATUS\0")
    hasher.update(status.stdout.encode("utf-8", errors="replace"))
    return hasher.hexdigest()


def protected_state(repo_root: Path, protected_paths: list[str]) -> str | None:
    diff = run_git(repo_root, ["diff", "--binary", "HEAD", "--", *protected_paths])
    status = run_git(repo_root, ["status", "--porcelain=v1", "-uall", "--", *protected_paths])
    if not diff or not status or diff.returncode != 0 or status.returncode != 0:
        return None
    hasher = hashlib.sha256()
    hasher.update(diff.stdout.encode("utf-8", errors="replace"))
    hasher.update(b"\0PROTECTED-STATUS\0")
    hasher.update(status.stdout.encode("utf-8", errors="replace"))
    return hasher.hexdigest()


def trim_output(output: str, limit: int) -> tuple[str, bool]:
    clean = redact(output)
    encoded = clean.encode("utf-8", errors="replace")
    if len(encoded) <= limit:
        return clean, False
    marker = b"\n...[output truncated by governed verifier]...\n"
    remaining = max(0, limit - len(marker))
    head_size = remaining // 3
    tail_size = remaining - head_size
    trimmed = encoded[:head_size] + marker + encoded[-tail_size:]
    return trimmed.decode("utf-8", errors="replace"), True

def resolve_repo_root(config_path: Path, config: dict[str, Any]) -> Path:
    base = config_path.parent.parent.resolve()
    root = (base / config["root"]).resolve()
    require(root == base, "Configured repository root must be the directory containing governance/project.json")
    require(root.exists() and root.is_dir(), f"Resolved repository root is not a directory: {root}")
    return root


def list_configuration(config: dict[str, Any]) -> None:
    print(f"Project: {config['project']}")
    print("Profiles:")
    for name, gate_ids in config["profiles"].items():
        print(f"  {name}: {', '.join(gate_ids)}")
    print("Gates:")
    for gate in config["gates"]:
        requirement = "required" if gate["required"] else "advisory"
        print(f"  {gate['id']} ({requirement}, {gate['timeoutSeconds']}s): {shlex.join(gate['command'])}")


def run_gate(gate: dict[str, Any], repo_root: Path, output_limit: int) -> dict[str, Any]:
    working_directory = (repo_root / gate.get("workingDirectory", ".")).resolve()
    try:
        working_directory.relative_to(repo_root)
    except ValueError as error:
        raise ConfigError(f"Gate {gate['id']} workingDirectory escapes repository root") from error
    require(working_directory.exists() and working_directory.is_dir(), f"Gate {gate['id']} workingDirectory does not exist: {working_directory}")
    command = gate["command"]
    executable_candidates = TRUSTED_EXECUTABLES.get(command[0], ())
    executable_path = next((Path(path) for path in executable_candidates if Path(path).is_file() and os.access(path, os.X_OK)), None)
    require(executable_path is not None, f"Gate {gate['id']} executable is unavailable: {command[0]}")
    resolved_executable = executable_path.resolve()
    try:
        resolved_executable.relative_to(repo_root)
    except ValueError:
        pass
    else:
        raise ConfigError(f"Gate {gate['id']} executable resolves inside repository root")
    command = [str(executable_path), *command[1:]]
    environment = {
        "PATH": f"{executable_path.parent}{os.pathsep}{os.defpath}",
        "CI": "true",
        "CARGO_TERM_COLOR": "always",
    }
    display_command = shlex.join(command)
    print(f"\n[{gate['id']}] {gate['description']}")
    print(f"$ {display_command}")
    started = time.monotonic()
    started_at = dt.datetime.now(dt.timezone.utc).isoformat()
    result: dict[str, Any] = {
        "id": gate["id"],
        "description": gate["description"],
        "command": command,
        "required": gate["required"],
        "workingDirectory": str(working_directory.relative_to(repo_root)) or ".",
        "startedAt": started_at,
    }
    try:
        process = subprocess.run(
            command,
            cwd=working_directory,
            env=environment,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            errors="replace",
            timeout=gate["timeoutSeconds"],
            check=False,
        )
        output, truncated = trim_output(process.stdout or "", output_limit)
        result.update(
            {
                "status": "passed" if process.returncode == 0 else "failed",
                "exitCode": process.returncode,
                "output": output,
                "outputTruncated": truncated,
            }
        )
    except subprocess.TimeoutExpired as error:
        combined = ""
        if isinstance(error.stdout, bytes):
            combined = error.stdout.decode("utf-8", errors="replace")
        elif isinstance(error.stdout, str):
            combined = error.stdout
        output, truncated = trim_output(combined, output_limit)
        result.update({"status": "timed_out", "exitCode": None, "output": output, "outputTruncated": truncated})
    except OSError as error:
        result.update({"status": "blocked", "exitCode": None, "output": redact(str(error)), "outputTruncated": False})
    result["durationSeconds"] = round(time.monotonic() - started, 3)

    print(f"=> {result['status']} ({result['durationSeconds']}s)")
    if result["status"] != "passed" and result["output"]:
        print(result["output"][-4000:])
    return result


def write_report(path: Path, report: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    temporary.replace(path)


def main() -> int:
    args = parse_args()
    config_path = Path(args.config).resolve()
    try:
        config = load_and_validate(config_path)
        if args.list:
            list_configuration(config)
            return 0
        require(args.profile in config["profiles"], f"Unknown profile {args.profile!r}; available: {', '.join(config['profiles'])}")
        repo_root = resolve_repo_root(config_path, config)
        gate_by_id = {gate["id"]: gate for gate in config["gates"]}
        gates = [gate_by_id[gate_id] for gate_id in config["profiles"][args.profile]]
        output_limit = int(config.get("integrity", {}).get("reportOutputLimitBytes", 20000))
        require(1000 <= output_limit <= 1000000, "integrity.reportOutputLimitBytes must be 1000..1000000")

        before_state = candidate_state(repo_root)
        before_digest = repository_state_digest(repo_root)
        before_protected = protected_state(repo_root, config["protectedPaths"])
        started_at = dt.datetime.now(dt.timezone.utc).isoformat()
        results = [run_gate(gate, repo_root, output_limit) for gate in gates]
        after_state = candidate_state(repo_root)
        after_digest = repository_state_digest(repo_root)
        after_protected = protected_state(repo_root, config["protectedPaths"])

        integrity_required = bool(config.get("integrity", {}).get("failOnTrackedChangeDuringVerification", True))
        integrity_checked = before_digest is not None and after_digest is not None
        repository_changed = integrity_checked and before_digest != after_digest
        head_changed = before_state["head"] != after_state["head"]
        protected_checked = before_protected is not None and after_protected is not None
        protected_changed = protected_checked and before_protected != after_protected
        integrity_result = {
            "checked": integrity_checked,
            "required": integrity_required,
            "changedDuringVerification": repository_changed,
            "headChangedDuringVerification": head_changed,
            "protectedPathsChecked": protected_checked,
            "protectedPathsChangedDuringVerification": protected_changed,
            "before": before_state,
            "after": after_state,
        }

        required_failures = [result for result in results if result["required"] and result["status"] != "passed"]
        advisory_failures = [result for result in results if not result["required"] and result["status"] != "passed"]
        if integrity_required and (
            not integrity_checked
            or not protected_checked
            or repository_changed
            or head_changed
            or protected_changed
        ):
            verdict = "failed"
        elif required_failures:
            verdict = "failed"
        elif advisory_failures:
            verdict = "passed_with_advisories"
        else:
            verdict = "passed"

        report = {
            "schemaVersion": 1,
            "project": config["project"],
            "profile": args.profile,
            "verdict": verdict,
            "configDigest": config["_digest"],
            "repositoryRoot": str(repo_root),
            "startedAt": started_at,
            "completedAt": dt.datetime.now(dt.timezone.utc).isoformat(),
            "candidate": before_state,
            "results": results,
            "repositoryIntegrity": integrity_result,
        }
        report_path = Path(args.report).resolve() if args.report else repo_root / ".agent" / "reports" / f"verification-{args.profile}.json"
        write_report(report_path, report)

        print(f"\nVerdict: {verdict}")
        print(f"Report: {report_path}")
        if repository_changed:
            print("Repository state changed during verification; inspect the diff. The verifier did not clean or revert it.")
        if integrity_required and not integrity_checked:
            print("Repository integrity could not be checked in Git; the profile failed closed.")
        return 0 if verdict in {"passed", "passed_with_advisories"} else 1
    except ConfigError as error:
        print(f"Configuration error: {error}", file=sys.stderr)
        return 2
    except OSError as error:
        print(f"Verification I/O error: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
