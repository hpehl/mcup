#!/bin/bash

# Generate the man page using asciidoctor
#
# Prerequisite:
#   asciidoctor

if ! command -v asciidoctor &> /dev/null
then
    echo "asciidoctor not found"
    exit 1
fi

asciidoctor \
  --doctype manpage \
  --backend manpage \
  --out-file mcup.1 \
  mcup.adoc
