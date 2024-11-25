MODEL=~/thejug/security_model.toml
cargo run -- -m $MODEL mermaid > diagram.m && ./node_modules/.bin/mmdc -i diagram.m -b black && /Applications/Firefox.app/Contents/MacOS/firefox file:///Users/craig/taconic/lab/secmodel/diagram.m.svg
