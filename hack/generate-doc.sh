#!/bin/sh

# Base directory for generated docs
DOCS_DIR="target/doc"

# Path for the new index.html
INDEX_PAGE="$DOCS_DIR/index.html"

# Find the CSS files
NORMALIZE_CSS=$(find $DOCS_DIR -type f -name 'normalize-*.css' | head -n 1)
RUSTDOC_CSS=$(find $DOCS_DIR -type f -name 'rustdoc-*.css' | head -n 1)

# Convert absolute paths to relative paths
NORMALIZE_CSS_REL="./${NORMALIZE_CSS#$DOCS_DIR/}"
RUSTDOC_CSS_REL="./${RUSTDOC_CSS#$DOCS_DIR/}"

# Start generating the custom index.html with improved indentation
{
    echo "<!DOCTYPE html>"
    echo "<html lang=\"en\">"
    echo "  <head>"
    echo "    <meta charset=\"utf-8\">"
    echo "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">"
    echo "    <title>Workspace Documentation - Rust</title>"
    echo "    <link rel=\"stylesheet\" href=\"$NORMALIZE_CSS_REL\">"
    echo "    <link rel=\"stylesheet\" href=\"$RUSTDOC_CSS_REL\">"
    echo "  </head>"
    echo "  <body class=\"rustdoc\">"
    echo "    <main>"
    echo "      <div class=\"width-limiter\">"
    echo "        <section id=\"main-content\" class=\"content\">"
    echo "          <div class=\"main-heading\">"
    echo "            <h1>Workspace Documentation</h1>"
    echo "          </div>"
    echo "          <ul>"
} > "$INDEX_PAGE"

# Find all Cargo.toml files in the workspace, excluding the top-level one
find libs apps -name Cargo.toml | while read cargo_toml; do
    CRATE_DIR=$(dirname "$cargo_toml")
    CRATE_NAME=$(basename "$CRATE_DIR")
    CRATE_ID=$(echo "$CRATE_NAME" | tr '-' '_')
    # Link to the crate's documentation with improved indentation
    echo "            <li><a href='./$CRATE_ID/index.html'>$CRATE_NAME</a></li>" >> "$INDEX_PAGE"
done

# Finish the index.html file with improved indentation
{
    echo "          </ul>"
    echo "        </section>"
    echo "      </div>"
    echo "    </main>"
    echo "  </body>"
    echo "</html>"
} >> "$INDEX_PAGE"

echo "Custom index.html generated at $INDEX_PAGE"
