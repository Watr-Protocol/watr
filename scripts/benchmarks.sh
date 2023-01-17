#!/usr/bin/env bash

__dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

${__dir}/benchmarks-ci.sh devnet
${__dir}/benchmarks-ci.sh mainnet
