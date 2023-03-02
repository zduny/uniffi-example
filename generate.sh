#!/bin/sh
set -e

export OUTPUT_FILE_PATH=bindings

PROJECTS=('simple' 'vectors')

# temporarily disable Ruby and Python.
LANGUAGES=('kotlin' 'swift')

for project in "${PROJECTS[@]}"
do
   cd bindings-$project

   for language in "${LANGUAGES[@]}"
   do
      cargo run -p uniffi-bindgen generate src/$project.udl --language $language \
      --out-dir=$OUTPUT_FILE_PATH/$language --config=uniffi.toml 
   done

   cd ..
done
