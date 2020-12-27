#!/usr/bin/env bash

file="lambda_retriever.zip"
if [ -f "$file" ] ; then
    rm "$file"
fi

cargo build -p golden_retriever --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/golden_retriever ./bootstrap

zip lambda_retriever.zip bootstrap

#aws lambda create-function --function-name rustTest \
#  --handler doesnt.matter \
#  --zip-file fileb://./lambda_retriever.zip \
#  --runtime provided \
#  --role arn:aws:iam::082140834855:role/service-role/TestLambda-role-d5g8xbc6 \
#  --environment Variables={RUST_BACKTRACE=1} \
#  --tracing-config Mode=Active \
#  --cli-binary-format raw-in-base64-out
