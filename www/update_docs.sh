#!/usr/bin/env bash
BMATH=$(realpath $(pwd)/..)

rm -rf "$BMATH"/www/static/doc
(cd "$BMATH"; cargo doc)
mv "$BMATH"/target/doc "$BMATH"/www/static
echo "Done generating docs"
