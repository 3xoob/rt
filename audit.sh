#!/bin/bash

set -e

RESULTS_DIR="Result"
mkdir -p "$RESULTS_DIR"

# ANSI color codes
GREEN='\033[1;32m'
YELLOW='\033[1;33m'
BLUE='\033[1;34m'
CYAN='\033[1;36m'
RED='\033[1;31m'
RESET='\033[0m'

print_header() {
  echo -e "${CYAN}========================================${RESET}"
  echo -e "${BLUE}$1${RESET}"
  echo -e "${CYAN}========================================${RESET}"
}

print_case() {
  echo -e "${YELLOW}Running test case: ${GREEN}$1${RESET}"
}

print_success() {
  echo -e "${GREEN}✔ $1${RESET}"
}

print_fail() {
  echo -e "${RED}✗ $1${RESET}"
}

run_case() {
  CASE_NAME="$1"
  shift
  # Remove Output directory before each run
  rm -rf Output
  mkdir -p "$RESULTS_DIR/$CASE_NAME"
  print_case "$CASE_NAME"
  echo "Running: cargo run --release -- $*" | tee "$RESULTS_DIR/$CASE_NAME/command.txt"
  if cargo run --release -- "$@"; then
    print_success "Completed $CASE_NAME"
  else
    print_fail "Failed $CASE_NAME"
  fi
  cp Output/*.ppm "$RESULTS_DIR/$CASE_NAME/" 2>/dev/null || true
  echo -e "${CYAN}----------------------------------------${RESET}"
}

print_header "Raytracer Automated Audit"

# Test all scenes and features
run_case "all_objects_default" --scene all
run_case "plane_sphere" --scene plane_sphere
run_case "plane_cube" --scene plane_cube
run_case "plane_cylinder" --scene plane_cylinder
run_case "all_objects_camera_moved" --scene all --camera 0.0,2.0,-10.0
run_case "reflection" --scene reflection
run_case "refraction" --scene refraction
run_case "textures" --scene all --textures
run_case "all_particles" --scene all_particles --particles 200
run_case "particles" --scene particles --particles 200

rm -rf Output

echo -e "${CYAN}========================================${RESET}"
echo -e "${GREEN}Audit complete. Results in $RESULTS_DIR/${RESET}"
echo -e "${CYAN}========================================${RESET}"
