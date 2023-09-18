#!/bin/bash

# Check if the correct number of arguments is provided
if [ "$#" -ne 3 ]; then
    echo "Usage: $0 <binary_location> <chain_spec_location> <output_file_name>"
    exit 1
fi

binary_location="$1"
chain_spec_location="$2"
output_file_name="$3"

# Check if the binary location exists
if [ ! -f "$binary_location" ]; then
    echo "Error: Binary location '$binary_location' does not exist."
    exit 1
fi

# Check if the chain spec location exists
if [ ! -f "$chain_spec_location" ]; then
    echo "Error: Chain spec location '$chain_spec_location' does not exist."
    exit 1
fi

# Export the genesis state
export_output="$("$binary_location" export-genesis-state --chain="$chain_spec_location" "$output_file_name" 2>&1 | tee /dev/tty)"

echo ""

# Check if the export command produced an error
if [[ "$export_output" == *"No specific runtime was recognized for ChainSpec's Id"* || "$export_output" == *"Error"* ]]; then
    echo "Error exporting genesis state. Please check your chain spec file."
    echo "Deleting '$output_file_name'..."
    rm "$output_file_name"
    exit 1
else
    echo "Genesis state exported successfully to '$output_file_name'"
fi



