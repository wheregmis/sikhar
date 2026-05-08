#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXAMPLE="${1:-}"
MODE="${2:-release}"
CARGO_HOME="${CARGO_HOME:-$ROOT_DIR/.cargo-home}"

if [[ -z "$EXAMPLE" ]]; then
  echo "usage: $0 <example> [release|debug]" >&2
  exit 1
fi

EXAMPLE_DIR="$ROOT_DIR/examples/$EXAMPLE"
if [[ ! -d "$EXAMPLE_DIR" ]]; then
  echo "unknown example: $EXAMPLE" >&2
  exit 1
fi

DX_ARGS=(build --platform web --package "$EXAMPLE")
PROFILE_DIR="debug"
if [[ "$MODE" != "debug" ]]; then
  DX_ARGS+=(--release)
  PROFILE_DIR="release"
fi

mkdir -p "$CARGO_HOME"

cd "$ROOT_DIR"
CARGO_HOME="$CARGO_HOME" NO_COLOR=true dx "${DX_ARGS[@]}"

DX_PUBLIC="$ROOT_DIR/target/dx/$EXAMPLE/$PROFILE_DIR/web/public"
if [[ ! -d "$DX_PUBLIC" ]]; then
  echo "Dioxus build did not produce expected public output: $DX_PUBLIC" >&2
  exit 1
fi

rm -rf "$EXAMPLE_DIR/dist"
mkdir -p "$EXAMPLE_DIR/dist"
cp -R "$DX_PUBLIC"/. "$EXAMPLE_DIR/dist"/
