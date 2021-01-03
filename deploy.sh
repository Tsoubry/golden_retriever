#!/usr/bin/env bash

file="lambda_retriever.zip"
if [ -f "$file" ] ; then
    rm "$file"
fi

cargo build -p golden_retriever --bin golden_retriever --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/golden_retriever ./bootstrap

zip lambda_retriever.zip bootstrap .env
