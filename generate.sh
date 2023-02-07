#!/bin/sh
set -e

export OUTPUT_FILE_PATH=bindings

PROJECTS=('simple' 'vectors')

LANGUAGES=('kotlin' 'swift' 'python' 'ruby')

for project in "${PROJECTS[@]}"
do
   cd bindings-$project

   for language in "${LANGUAGES[@]}"
   do
      cargo run -p uniffi-bindgen generate src/$project.udl --language $language \
      --docs --out-dir=$OUTPUT_FILE_PATH/$language --config=uniffi.toml 
   done

   cd ..
done
