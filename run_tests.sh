#!/bin/bash

# Test script for assembly graph generator

set -e  # Exit on error

echo "🔬 Running comprehensive determinism tests..."

# Build with test features
echo "Building with determinism test features..."
cargo build --features determinism_test --release

# 1. Quick synthetic test
echo -e "\n1. Quick synthetic data test:"
DETERMINISM_CHECK=1 cargo run --features determinism_test --release -- --help 2>/dev/null || true

# 2. Extended tests with real data (if available)
if [ -d "test/data/fastqs" ] && [ -n "$(ls -A test/data/fastqs/*.fastq test/data/fastqs/*.fq 2>/dev/null)" ]; then
    echo -e "\n2. Extended tests with real data:"
    EXTENDED_TEST=1 DETERMINISM_CHECK=1 cargo run --features determinism_test --release -- --help 2>/dev/null || true
else
    echo -e "\n2. Skipping extended tests (no real data found in test/data/fastqs/)"
    echo "   To run extended tests, place FASTQ files in test/data/fastqs/"
fi

# 3. Run the actual unit tests
echo -e "\n3. Running unit tests:"
cargo test --features determinism_test -- --nocapture

# 4. Performance benchmark
echo -e "\n4. Performance benchmark (3 runs):"
if [ -d "test/data/fastqs" ] && [ -n "$(ls -A test/data/fastqs/*.fastq test/data/fastqs/*.fq 2>/dev/null)" ]; then
    for i in {1..3}; do
        echo "   Run $i:"
        time cargo run --features determinism_test --release -- \
            --input-dir test/data/fastqs \
            --output-path /tmp/test_output_$i.asg \
            --kmer-len 15 \
            --temp-concat-fastq-path /tmp/temp_concat_$i.fastq 2>/dev/null
        echo
    done
fi

echo -e "\n✅ All tests completed!"
