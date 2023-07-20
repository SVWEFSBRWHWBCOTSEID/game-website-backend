#!/bin/bash
sed -i '/include_str!/c\include_str!("../prisma/schema.prisma");' ./src/prisma.rs
