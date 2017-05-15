See `hot` in action:
1. `cargo run` (A green window should pop up)
2. Open up src/lib.rs. Change `target.clear_color(0.0, 1.0, 0.0, 0.0);` to `target.clear_color(1.0, 0.0, 0.0, 0.0);`
3. `cargo build --lib`
4. Your window should now be red.
5. Profit!

You can also run `cargo run --no-default-features` to run the application with all of the code statically linked and with hotloading disabled.
