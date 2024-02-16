#!/bin/bash

# Apply migrations, skipping the baseline migration
# https://www.prisma.io/docs/orm/prisma-migrate/workflows/baselining
./target/release/prisma-cli migrate resolve --applied 20230930182401_baseline
./target/release/prisma-cli migrate deploy

# Run backend executable
./target/release/game-backend
