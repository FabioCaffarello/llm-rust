#!/bin/sh

CARGO_CMD="cargo"
project="$1"

if [ -z "$project" ]; then
    # No specific project specified; run tests for all projects
    echo "Testing all projects:"
    for PROJECT_DIR in apps/*/ libs/*/; do
        PROJECT_NAME=$(basename "$PROJECT_DIR")
        "$CARGO_CMD" test --test integration_test
    done
else
    echo "Testing specified project: $PROJECT_NAME"
    "$CARGO_CMD" test -p "$project" --test integration_test
fi
