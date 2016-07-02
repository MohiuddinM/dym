#!/bin/bash
cargo doc
cp -r ./doc /tmp/gh-pages
git checkout gh-pages
git rm -rf .
mv /tmp/gh-pages/* .
