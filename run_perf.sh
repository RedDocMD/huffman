#!/bin/bash

# Command line args
BUF_SIZE=$1
DATA_FILE=$2

DATA_DIR=data
PERF_DIR=perf

INPUT_FILE_NAME=$DATA_DIR/"$DATA_FILE".txt
OUTPUT_FILE_NAME=$DATA_DIR/"$DATA_FILE".huff
PERF_DATA_FILE=$PERF_DIR/"$DATA_FILE"_"$BUF_SIZE".perf
SVG_NAME=$PERF_DIR/"$DATA_FILE"_"$BUF_SIZE".svg

echo "Input file: $INPUT_FILE_NAME"
echo "Output file: $OUTPUT_FILE_NAME"
echo "Perf data file: $PERF_DATA_FILE"
echo "SVG output file: $SVG_NAME"

FLAMEGRAPH_DIR=/home/deep/software/FlameGraph

EXEC=target/release/encoder_rig

sudo perf record -o $PERF_DATA_FILE -g --call-graph dwarf $EXEC $BUF_SIZE $INPUT_FILE_NAME $OUTPUT_FILE_NAME
sudo perf script -i $PERF_DATA_FILE | $FLAMEGRAPH_DIR/stackcollapse-perf.pl | $FLAMEGRAPH_DIR/flamegraph.pl > $SVG_NAME

exit 0