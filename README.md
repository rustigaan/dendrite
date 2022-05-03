# Dendrite

A Rust library to connect to AxonServer.

See the [crate readme](/dendrite/README.md).

Note that if protoc is not installed, then the tonic part of the build will try to build it from source.

On Mac OS
* This might fail because `cmake` is not installed
* Install protoc with Homebrew:

  `brew install protobuf`