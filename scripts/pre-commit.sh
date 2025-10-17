#!/bin/bash

set -e
echo "🎨 Running cargo fmt..."
if ! cargo +nightly fmt --all -- --check; then
  echo "❌ Code formatting issues found. Run 'cargo +nightly fmt --all' to fix them."
  exit 1
fi
echo "📎 Running cargo clippy..."
if ! cargo clippy --all-targets -- -D warnings; then
  echo "❌ Clippy found issues. Please fix them before committing."
  exit 1
fi
./scripts/cargo-sort.sh
cargo machete
echo "✅ All pre-commit checks passed!"
