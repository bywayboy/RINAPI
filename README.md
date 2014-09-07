RINAPI
======

Make a set of Windows APIs bridge for Rust
創建一個 Rust 可使用的 Windows APIs 函數訪問集

```rust
rustc wmain.rs -C link-args="-Wl,--subsystem,windows"
```

to compile test code. hello world application.
