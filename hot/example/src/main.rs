#[cfg(feature = "hotload")]
extern crate hot;

#[cfg(not(feature = "hotload"))]
extern crate example;

#[cfg(feature = "hotload")]
fn main() {
  // This probably needs some tweaking to work on non-Macs.
  let dylib_path = {
    let mut dylib_path = std::env::current_exe().unwrap();
    dylib_path.pop();
    dylib_path.push("libexample.dylib");
    dylib_path.into_os_string()
  };

  let mut hot = hot::Library::new(dylib_path);

  let mut state = (hot.instance.init_fn)();

  'main: loop {
    hot.reload();

    let continue_ = (hot.instance.tick_fn)(&mut state);

    if !continue_ { break 'main; }
  }

  (hot.instance.cleanup_fn)(state);
}

#[cfg(not(feature = "hotload"))]
fn main() {
  let mut state = example::init();

  'main: loop {
    let continue_ = example::tick(&mut state);

    if !continue_ { break 'main; }
  }

  example::cleanup(state);
}
