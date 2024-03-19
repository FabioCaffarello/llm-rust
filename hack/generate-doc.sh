#!/bin/sh

# Base directory for generated docs
DOCS_DIR="target/doc"

# Path for the new index.html
INDEX_PAGE="$DOCS_DIR/index.html"

# Start generating the custom index.html
echo "<!DOCTYPE html><html><head><title>Workspace Documentation</title></head><body>" > "$INDEX_PAGE"
echo "<h1>Workspace Documentation</h1><ul>" >> "$INDEX_PAGE"

# Find all Cargo.toml files in the workspace, excluding the top-level one
find libs apps -name Cargo.toml | while read cargo_toml; do
    CRATE_DIR=$(dirname "$cargo_toml")
    CRATE_NAME=$(basename "$CRATE_DIR")
    CRATE_ID=$(echo "$CRATE_NAME" | tr '-' '_')
    # Link to the crate's documentation
    echo "<li><a href='./$CRATE_ID/index.html'>$CRATE_NAME</a></li>" >> "$INDEX_PAGE"
done

echo "</ul></body></html>" >> "$INDEX_PAGE"

echo "Custom index.html generated at $INDEX_PAGE"
