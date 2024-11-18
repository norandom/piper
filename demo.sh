#!/bin/bash

# Build the piper application
echo "Building piper..."
cargo build --release

# Create a test directory and initial log file
mkdir -p test_data
touch test_data/test.log

# Generate some test log data
echo "Generating test log data..."
for i in {1..100}; do
    echo "[$(date)] Log entry $i: This is a test message" >> test_data/test.log
    echo "[$(date)] Log entry $i: This is an error message" >> test_data/test.log
    echo "[$(date)] Log entry $i: This is a debug message" >> test_data/test.log
done

# Now generate some continuous data in the background
(
    for i in {101..200}; do
        echo "[$(date)] Log entry $i: This is a test message" >> test_data/test.log
        echo "[$(date)] Log entry $i: This is an error message" >> test_data/test.log
        echo "[$(date)] Log entry $i: This is a debug message" >> test_data/test.log
        sleep 0.1
    done
) &
GENERATOR_PID=$!

# Let's demonstrate the pipe functionality
echo "Starting piper with 1MB buffer size..."
echo "Piping test.log through piper and filtering for 'error'..."
echo "Press Ctrl+C after a few seconds to see the backup functionality"

# Run piper for 10 seconds or until Ctrl+C
timeout 10s tail -f test_data/test.log | ./target/release/piper -s 1 | grep "error" | tee test_data/filtered.log || true

# Kill the background process
kill $GENERATOR_PID 2>/dev/null || true

# After completion, show the backup
echo -e "\nShowing contents of the backup file:"
cat .piper/buffer_backup

# Show some stats
echo -e "\nStats:"
echo "Original log size: $(wc -l < test_data/test.log) lines"
echo "Filtered log size: $(wc -l < test_data/filtered.log) lines"
echo "Backup file size: $(wc -l < .piper/buffer_backup) lines"

# Cleanup
echo -e "\nCleaning up..."
rm -rf test_data

echo "Demo complete!"
