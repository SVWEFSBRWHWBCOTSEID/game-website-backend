#!/bin/bash
./scripts/fix_prisma_str.sh
./target/release/prisma-cli db push
./target/release/game-backend
