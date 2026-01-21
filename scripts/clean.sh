#! /bin/bash
cd assets/cmp || echo "Directory not found" && exit 1

(cd calculator && cargo clean)
(cd adder && cargo clean)
(cd command && cargo clean)