#!/bin/bash

# Apply migrations, skipping the baseline migration
# https://www.prisma.io/docs/orm/prisma-migrate/workflows/baselining
./prisma-cli migrate resolve --applied 20230930182401_baseline
./prisma-cli migrate deploy

# Run backend executable
./game-backend
