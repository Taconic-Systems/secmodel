#!/usr/bin/env bash
set -x
MODEL=${1:-A model file must be provided}
cargo run -- -m $MODEL mermaid > diagram.m && ./node_modules/.bin/mmdc -i diagram.m -b black && firefox file:///${PWD}/diagram.m.svg
