#!/bin/bash

cd packages/dart_bridge
rm -rf target

cmake -DCMAKE_BUILD_TYPE=Release -S . -B target/release
cmake --build target/release
cd ../..

flutter pub get
dart run ffigen --config ffigen_waku_dart_bridge.yaml

flutter run -d linux
