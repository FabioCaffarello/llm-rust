#!/bin/sh

CARGO_CMD="cargo"
project="$1"

run_tests() {
    local project_dir="$1"
    # Extract the package name from Cargo.toml
    local project_name=$(grep '^name = ' "$project_dir/Cargo.toml" | head -1 | cut -d '"' -f 2)
    echo "Testing project: $project_name"
    if [ -d "$project_dir/tests" ]; then
        for test_file in "$project_dir"/tests/*_test.rs; do
            local test_name=$(basename "$test_file")
            test_name="${test_name%_test.rs}" # Remove the '_test.rs' part to get the test module name
            echo "Running $test_name for $project_name"
            # Use the extracted package name here
            "$CARGO_CMD" test -p "$project_name" --test "$test_name"_test
        done
    else
        echo "No tests directory found for $project_name"
    fi
}

if [ -z "$project" ]; then
    # No specific project specified; run tests for all projects
    echo "Testing all projects:"
    for PROJECT_DIR in apps/*/ libs/resources/*/; do
        if [ -f "$PROJECT_DIR/Cargo.toml" ]; then
            run_tests "$PROJECT_DIR"
        fi
    done
else
    PROJECT_DIR=$(find apps/ libs/resources/ -maxdepth 2 -type d -name "$project")
    if [ -n "$PROJECT_DIR" ] && [ -f "$PROJECT_DIR/Cargo.toml" ]; then
        run_tests "$PROJECT_DIR"
    else
        echo "Specified project '$project' not found or lacks a Cargo.toml file."
    fi
fi




# CARGO_CMD="cargo"
# project="$1"

# if [ -z "$project" ]; then
#     # No specific project specified; run tests for all projects
#     echo "Testing all projects:"
#     for PROJECT_DIR in apps/*/ libs/*/; do
#         PROJECT_NAME=$(basename "$PROJECT_DIR")
#         "$CARGO_CMD" test --test integration_test
#     done
# else
#     echo "Testing specified project: $PROJECT_NAME"
#     "$CARGO_CMD" test -p "$project" --test integration_test
# fi
