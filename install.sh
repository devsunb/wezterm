#!/usr/bin/env bash

set -Eeuo pipefail

rm -rf WezTerm-macos-* || true
cargo build --release && ./ci/deploy.sh && rsync -av --delete WezTerm-macos-*/WezTerm.app/ /Applications/WezTerm.app/
