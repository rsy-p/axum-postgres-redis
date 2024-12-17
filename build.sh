#!/bin/bash
error_exit() { echo echo "$1"; exit 1; }

ENV="${1:-}"
ENV_FILE=".env${ENV:+.$ENV}"

TOML_FILE="Cargo.toml"
BUILD_DIR="./target/release"
DIST_DIR="./dist"
START_SCRIPT="start.sh"

# Check if necessary files exist
[[ ! -f "$ENV_FILE" ]] && error_exit "Error: env file [$ENV_FILE] not found."
[[ ! -f "$TOML_FILE" ]] && error_exit "Error: $TOML_FILE not found."

# Extract the project name from Cargo.toml
PROJECT_NAME=$(grep -oP '(?<=^name = ")[^"]+' "$TOML_FILE") || error_exit "Error: Could not extract project name from $TOML_FILE."

echo "Project name: $PROJECT_NAME"

if [ -z "$ENV" ]; then
    TAR_NAME="$PROJECT_NAME.tar.gz"
else
    TAR_NAME="$PROJECT_NAME-$ENV.tar.gz"
fi

# Build the project
echo "Running cargo build..."
cargo build --release || error_exit "Cargo build failed."

# Clean up and prepare distribution directory
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Copy environment file and compiled project to the distribution directory
cp "$ENV_FILE" "$DIST_DIR/.env"
cp "$BUILD_DIR/$PROJECT_NAME" "$DIST_DIR/$PROJECT_NAME"

# Generate the start script
cat << EOF > "$DIST_DIR/$START_SCRIPT"
#!/bin/bash

set -e

# Kill any existing processes of the project
PROCESS_ID=\$(pgrep -f "$PROJECT_NAME" || echo "")

if [ -n "\$PROCESS_ID" ]; then
    echo "Found process ID: \$PROCESS_ID. Killing process."
    kill -9 "\$PROCESS_ID"
else
    echo "No running process found for project '\$PROJECT_NAME'."
fi

# Start the project
nohup "./$PROJECT_NAME" > app.log 2>&1 &

echo "Start success"

rm -rf "$TAR_NAME"
EOF

chmod +x "$DIST_DIR/$START_SCRIPT"

# Create a tarball of the distribution directory
tar -czvf "./dist/$TAR_NAME" -C dist .env "$PROJECT_NAME" "$START_SCRIPT"

# Clean up the distribution directory
rm "$DIST_DIR/.env" "$DIST_DIR/$PROJECT_NAME" "$DIST_DIR/$START_SCRIPT"

echo "Build success. Tarball created: $DIST_DIR/$TAR_NAME"