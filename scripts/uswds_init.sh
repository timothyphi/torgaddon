#!/bin/bash

SCRIPT_PATH="$(readlink -f "$0")"
SCRIPT_DIR="$(dirname "$SCRIPT_PATH")"
PROJECT_DIR="$(readlink -f "$SCRIPT_DIR/../")"

pushd $PROJECT_DIR
npx gulp init

# We need to revert the changes from `uswds-compile`
git restore styles/vendors/uswds/_uswds-theme.scss
git restore styles/vendors/uswds/_uswds-theme-custom-styles.scss
git restore styles/vendors/uswds/styles.scss
popd
