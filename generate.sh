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
      uniffi-bindgen generate src/$project.udl --language \
      $language --out-dir=$OUTPUT_FILE_PATH/$language --config=uniffi.toml
   done

   cd ..
done