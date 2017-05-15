#[cfg(feature = "hotload")]
extern crate hot;

#[cfg(not(feature = "hotload"))]
extern crate example;

#[cfg(feature = "hotload")]
fn main() {
  // This probably needs some tweaking to work on non-Macs.
  // It'll also need some tweaking if you're running from somewhere other
  // than your crate's root directory.
  let mut hot = hot::Library::new("target/debug/libexample.dylib");

  let mut state = (hot.init_fn)();

  'main: loop {
    hot.reload();

    let continue_ = (hot.tick_fn)(&mut state);

    if !continue_ { break 'main; }
  }
}

#[cfg(not(feature = "hotload"))]
fn main() {
  let mut state = example::init();

  'main: loop {
    let continue_ = example::tick(&mut state);

    if !continue_ { break 'main; }
  }
}
