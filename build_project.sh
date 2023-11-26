#!/bin/bash

# Function to display script usage
usage() {
  echo "Usage: $0 [--run] [-c]"
  echo "Optional arguments:"
  echo "  --run   Run the compiled server executable"
  echo "  -c      Delete the build directory after running the server (only if --run is used)"
  echo "  --release Compile in release mode"
  exit 1
}

# Variables to track flags
RUN_SERVER=false
DELETE_BUILD=false
RELEASE_MODE=false

# Check for optional --run and -c flags
while [[ "$#" -gt 0 ]]; do
  case $1 in
    --run)
      RUN_SERVER=true
      ;;
    -c)
      DELETE_BUILD=true
      ;;
    --release)
      RELEASE_MODE=true
      ;;
    *)
      usage
      ;;
  esac
  shift
done

# Step 1: Create a new directory called `build` in the project root
mkdir -p build/static

# Step 2: Compile `server` with `cargo build --release` and copy the executable into `build`
if [ "$RELEASE_MODE" = true ]; then
  echo "Compiling server in release mode..."
  cd server
  cargo build --release
  cp target/release/server ../build/
else
  echo "Compiling server..."
  cd server
  cargo build
  cp target/debug/server ../build/
fi
cd ..

# Step 3: Compile `wasm` with `trunk build --release` and copy the necessary files into `build/static`
if [ "$RELEASE_MODE" = true ]; then
  echo "Compiling wasm in release mode..."
  cd wasm
  trunk build --release
else
  echo "Compiling wasm..."
  cd wasm
  trunk build
fi
cp -r dist/* ../build/static/
cd ..

# Step 4: Run the server if the --run flag is provided
if [ "$RUN_SERVER" = true ]; then
  echo "Running server..."
  cd build
  ./server

  # Step 5: Delete the build directory if -c flag is used
  if [ "$DELETE_BUILD" = true ]; then
    echo "Deleting build directory..."
    cd ..
    rm -rf build
  fi
fi

echo "Build process completed successfully!"
