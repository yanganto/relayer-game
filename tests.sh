#!/usr/bin/env bash

echo "==========================="
echo "==== Test patch on cli ===="
echo "==========================="
echo ""
echo "======== no patch ==========="
cargo run -- -v scenario/linear_bond.yml
echo ""
echo "== patch bond_function=1.234 =="
cargo run -- -p bond_function=1.234 -v scenario/linear_bond.yml
echo ""
echo "======== no patch ==========="
cargo run -- -v scenario/basic.yml
echo ""
echo "== patch challenge_function=100 =="
cargo run -- -p challenge_function=100 -v scenario/linear_bond.yml