extern crate notify;
extern crate libloading;

use std::mem;
use std::sync::mpsc;

pub struct State();

pub struct Library {
  dylib_path: String,
  raw: Option<libloading::Library>,
  _watcher: notify::RecommendedWatcher,
  rx: mpsc::Receiver<notify::RawEvent>,
  pub init_fn: libloading::os::unix::Symbol<extern fn() -> Box<State>>,
  pub tick_fn: libloading::os::unix::Symbol<extern fn(&mut State) -> bool>,
}

impl Library {
  pub fn new(dylib_path: &str) -> Library {
    use notify::Watcher;

    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::raw_watcher(tx).unwrap();

    watcher.watch(dylib_path, notify::RecursiveMode::NonRecursive).unwrap();

    let raw = libloading::Library::new(dylib_path).expect("Couldn't find dylib");

    let init_fn = self::init_fn(&raw);
    let tick_fn = self::tick_fn(&raw);

    Library {
      dylib_path: dylib_path.to_string(),
      raw: Some(raw),
      rx: rx,
      _watcher: watcher,
      init_fn: init_fn,
      tick_fn: tick_fn,
    }
  }

  pub fn reload(&mut self) {
    for _ in self.rx.try_recv() {
      println!("Reloading!");

      // Most OSes do internal refcounting on dylibs, so this needs to be explicitly dropped
      // to knock the refcount down to 0 so the old version gets unloaded.
      mem::drop(self.raw.take());

      let library = libloading::Library::new(&self.dylib_path).unwrap();

      self.init_fn = self::init_fn(&library);
      self.tick_fn = self::tick_fn(&library);
      self.raw = Some(library);
    }
  }
}

impl Drop for Library {
  fn drop(&mut self) {
    // Workaround for https://github.com/rust-lang/rust/issues/28794
    mem::forget(self.raw.take());
  }
}

fn init_fn(library: &libloading::Library) -> libloading::os::unix::Symbol<extern fn() -> Box<State>> {
  unsafe{ library.get::<extern fn() -> Box<State>>(b"init").unwrap().into_raw() }
}

fn tick_fn(library: &libloading::Library) -> libloading::os::unix::Symbol<extern fn(&mut State) -> bool> {
  unsafe{ library.get::<extern fn(&mut State) -> bool>(b"tick").unwrap().into_raw() }
}
