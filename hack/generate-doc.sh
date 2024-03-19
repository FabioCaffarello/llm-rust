#!/bin/sh

set -e

# Base directory for generated docs
DOCS_DIR="target/doc"

# Path for the new index.html
INDEX_PAGE="$DOCS_DIR/index.html"

# Find the CSS files
NORMALIZE_CSS=$(find "$DOCS_DIR" -type f -name 'normalize-*.css' | head -n 1)
RUSTDOC_CSS=$(find "$DOCS_DIR" -type f -name 'rustdoc-*.css' | head -n 1)

# Exit if CSS files are not found
if [ -z "$NORMALIZE_CSS" ] || [ -z "$RUSTDOC_CSS" ]; then
    echo "Error: CSS files not found."
    exit 1
fi

# Convert absolute paths to relative paths
NORMALIZE_CSS_REL="./${NORMALIZE_CSS#$DOCS_DIR/}"
RUSTDOC_CSS_REL="./${RUSTDOC_CSS#$DOCS_DIR/}"

# Start generating the custom index.html with improved indentation
{
    printf "<!DOCTYPE html>\n"
    printf "<html lang=\"en\">\n"
    printf "  <head>\n"
    printf "    <meta charset=\"utf-8\">\n"
    printf "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n"
    printf "    <title>Workspace Documentation - Rust</title>\n"
    printf "    <link rel=\"stylesheet\" href=\"%s\">\n" "$NORMALIZE_CSS_REL"
    printf "    <link rel=\"stylesheet\" href=\"%s\">\n" "$RUSTDOC_CSS_REL"
    printf "    <base href=\"./\">\n"  # Ensures relative links work from any depth
    printf "  </head>\n"
    printf "  <body class=\"rustdoc\">\n"
    printf "    <main>\n"
    printf "      <div class=\"width-limiter\">\n"
    printf "        <section id=\"main-content\" class=\"content\">\n"
    printf "          <div class=\"main-heading\">\n"
    printf "            <h1>Workspace Documentation</h1>\n"
    printf "          </div>\n"
    printf "          <ul>\n"
} > "$INDEX_PAGE"

# Find all Cargo.toml files in the workspace, excluding the top-level one
find libs apps -name Cargo.toml | while read cargo_toml; do
    CRATE_DIR=$(dirname "$cargo_toml")
    CRATE_NAME=$(basename "$CRATE_DIR")
    CRATE_ID=$(echo "$CRATE_NAME" | tr '-' '_')
    # Link to the crate's documentation with improved indentation
    printf "            <li><a href='%s/index.html'>%s</a></li>\n" "$CRATE_ID" "$CRATE_NAME" >> "$INDEX_PAGE"
done

# Finish the index.html file with improved indentation
{
    printf "          </ul>\n"
    printf "        </section>\n"
    printf "      </div>\n"
    printf "    </main>\n"
    printf "  </body>\n"
    printf "</html>\n"
} >> "$INDEX_PAGE"

echo "Custom index.html generated at $INDEX_PAGE"
