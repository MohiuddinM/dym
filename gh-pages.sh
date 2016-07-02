#!/bin/bash
cargo doc
cp ./target/doc /tmp/rs-pages
git checkout gh-pages
mv /tmp/rs-pages/* ./
git add -A
git commit -m "Generated docs"
