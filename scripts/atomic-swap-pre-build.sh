#!/bin/bash
set -e

echo -e "\nDownloading trusted setup file..."
curl "https://storage.googleapis.com/zkevm/ptau/powersOfTau28_hez_final_17.ptau" --output "./powers.dev/17.ptau"

./scripts/compile-circuit.sh atomic_swap
./scripts/export-keys.sh atomic_swap 17