#!/bin/bash

cd packages/dart_bridge
rm -rf target

cargo build --release --lib
cd ../..

flutter pub get
dart run ffigen --config ffigen_waku_dart_bridge.yaml

flutter run -d linux
