#!/bin/bash

# Current date
DATE=$(date '+%Y-%m-%d')

# Source and target directories
SRC_DIR="/workspace/.soroban"
TARGET_DIR="/workspace/public"

# File names
declare -a FILES=("all_tokens.json" "pairs.json" "factory.json")

# Copy files from .soroban to public/
for FILE in "${FILES[@]}"
do
    cp "$SRC_DIR/$FILE" "$TARGET_DIR"
    git add "$TARGET_DIR/$FILE" 
done

# Git commands
git commit -m "data: $DATE updated contract addresses"
git push