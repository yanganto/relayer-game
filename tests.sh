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
echo ""

echo "======== no patch ==========="
cargo run -- -v scenario/treasury_last.yml
echo "== patch reward_split.P=0.7 =="
cargo run -- -p reward_split.P=0.7 -- scenario/basic.yml
echo ""

echo "======== no patch ==========="
cargo run -- -v scenario/treasury_last.yml
echo "== patch reward_treasury_last.C=90 =="
cargo run -- -p reward_treasury_last.C=9.0 -- scenario/treasury_last.yml
echo ""

echo "==================================="
echo "==== Test some scenario on cli ===="
echo "==================================="
echo ""
cargo run -- -v scenario/sometimes_lie.yml
echo ""
cargo run -- -v scenario/challenger.yml
echo ""
cargo run -- -v scenario/multi-challengers.yml
echo ""
cargo run -- -v scenario/multi-challengers2.yml
echo ""
