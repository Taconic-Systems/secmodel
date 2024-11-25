#!/bin/bash
MODEL=~/thejug/security_model.toml
cargo run -- -m $MODEL report > report.md && ./node_modules/.bin/mmdc -i report.md -e pdf -o out.md && pandoc --toc out.md -o report.pdf && open report.pdf
