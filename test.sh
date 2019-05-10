#!/bin/bash
cargo check 2>&1 | grep -q "internal compiler error"; test $? -eq 1
