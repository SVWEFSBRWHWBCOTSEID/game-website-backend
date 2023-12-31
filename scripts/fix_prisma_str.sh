#!/bin/bash
echo "include_str!(\"../prisma/schema.prisma\");" > "new" && perl -i -p0e 's/include_str!\([^)]+\);/`cat new`/se' ./src/prisma.rs && rm "new"
