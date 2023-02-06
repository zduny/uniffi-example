#!/bin/sh
set -e

if ! [ -f jna.jar ]; then
  curl -o jna.jar https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.8.0/jna-5.8.0.jar 
fi

if ! [ -f kotlin-test-1.8.10.jar ]; then
  curl -o kotlin-test-1.8.10.jar \
  https://repo1.maven.org/maven2/org/jetbrains/kotlin/kotlin-test/1.8.10/kotlin-test-1.8.10.jar
fi

export CLASSPATH="$CLASSPATH:$(pwd)/jna.jar"
export CLASSPATH="$CLASSPATH:$(pwd)/kotlin-test-1.8.10.jar"

cargo test
