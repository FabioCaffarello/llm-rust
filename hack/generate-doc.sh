#!/bin/sh

#!/bin/bash

# Base directory for generated docs
DOCS_DIR="target/doc"

# Path for the new index.html
INDEX_PAGE="$DOCS_DIR/index.html"

# Start generating the custom index.html
echo "<!DOCTYPE html><html><head><title>Workspace Documentation</title></head><body>" > "$INDEX_PAGE"
echo "<h1>Workspace Documentation</h1><ul>" >> "$INDEX_PAGE"

# Assuming your workspace members are located under 'libs' and 'apps'
for CRATE in libs/* apps/*; do
    if [ -d "$CRATE" ]; then
        CRATE_NAME=$(basename "$CRATE")
        echo "<li><a href='./$CRATE_NAME/index.html'>$CRATE_NAME</a></li>" >> "$INDEX_PAGE"
    fi
done

echo "</ul></body></html>" >> "$INDEX_PAGE"

echo "Custom index.html generated at $INDEX_PAGE"

