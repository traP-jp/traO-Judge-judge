#!/bin/bash

sudo yum install -y git gcc >> /log.txt 2>&1

# TODO: バイナリだけ提供したほうが高速化できそう
git clone https://github.com/traP-jp/traO-judge-infra.git >> /log.txt 2>&1
cd traO-judge-infra >> /log.txt 2>&1
git checkout exec-http-server >> /log.txt 2>&1
cd exec-http-server >> /log.txt 2>&1

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y >> /log.txt 2>&1
/root/.cargo/bin/rustup update >> /log.txt 2>&1

/root/.cargo/bin/cargo run --release --bin exec-http-server >> /log.txt 2>&1