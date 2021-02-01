#!/bin/bash
mkdir build
if [[ $1 == "" ]] || [[ $1 == "all" ]]; then
    for dart in bin/*.dart; do
        dart2native $dart -o build/${dart%.*}
    done
else
    dart2native bin/$1.dart -o build/$1
fi
echo Done!