#!/bin/bash
cargo build --release
cp ./target/release/git-api /home/combustiblemon/startup/git-api
