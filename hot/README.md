My current little code hotloading solution for faster iteration during development.

Take a look at `example` to see how to use it.

This has only been tested on Mac. Windows in particular might take some additional effort to get worked, as IIRC Windows won't let you delete currently loaded `.dll`s.

You probaly want to use `-C prefer-dynamic` when you're building with this. (It'll make your build times faster, and it's less likely to mess up dependencies that use mutable `static`s.)

If you need something fancier, check out [dynamic_reload](https://github.com/emoon/dynamic_reload).
