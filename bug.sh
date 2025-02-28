#!/bin/sh
set -e

[ $0 = ./bug.sh ] || { echo "Usage: ./bug.sh"; exit 1; }
[ $1 = listen ] || { cargo run --bin=hello | ./bug.sh listen; exit 0; }
last=
while read line; do
  [ -z "$last" ] && { echo "Starting at $line"; last=$line; continue; }
  next=$(( last + 1 ))
  [ $line -eq $next ] || echo "Expected $next got $line"
  last=$line
done
echo "Finished with $last"
