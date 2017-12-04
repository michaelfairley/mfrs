My current little code hotloading solution for faster iteration during development.

Take a look at `example` to see how to use it.

This only works on macOS right now.

You might want to use `-C prefer-dynamic` when you're building with this.
(It'll make your build times faster, and it's less likely to mess up dependencies that use mutable `static`s.)

If you need something fancier, check out [dynamic_reload](https://github.com/emoon/dynamic_reload).
