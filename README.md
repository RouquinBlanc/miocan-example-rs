# Test app for using Metal I/O (mio) with socketcan

Socketcan is a great package for reading from CAN bus in Rust.
But it only support synchronous calls.

On the other side, Metal I/O provides a nice support for asynchronous loop,
for TCP, UDP, and... raw unix socket.

This is a quick example using [miocan](https://github.com/RouquinBlanc/miocan-rs)
for making the glue and reading from CAN bus with async IO.
