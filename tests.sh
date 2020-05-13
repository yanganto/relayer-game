#!/usr/bin/env bash

echo "==========================="
echo "==== Test patch on cli ===="
echo "==========================="
echo ""
echo "======== no patch ==========="
cargo run -- -v scenario/linear_fee.yml
echo ""
echo "== patch fee_function=1.234 =="
cargo run -- -p fee_function=1.234 -v scenario/linear_fee.yml
echo ""
echo "======== no patch ==========="
cargo run -- -v scenario/basic.yml
echo ""
echo "== patch wait_function=100 =="
cargo run -- -p wait_function=100 -v scenario/linear_fee.yml
