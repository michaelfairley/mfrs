extern crate notify;
extern crate libloading;

use std::sync::mpsc;
use std::ffi::OsString;

pub struct State();

pub struct Library {
  dylib_path: OsString,
  _watcher: notify::RecommendedWatcher,
  rx: mpsc::Receiver<notify::RawEvent>,
  pub instance: Instance,
}

pub struct Instance {
  _raw: libloading::Library,
  pub init_fn: libloading::os::unix::Symbol<extern fn() -> Box<State>>,
  pub tick_fn: libloading::os::unix::Symbol<extern fn(&mut State) -> bool>,
  pub cleanup_fn: libloading::os::unix::Symbol<extern fn(Box<State>)>,
}

impl Instance {
  fn load(dylib_path: &OsString) -> Self {
    let raw = libloading::Library::new(dylib_path).expect("Couldn't find dylib");

    let init_fn = init_fn(&raw);
    let tick_fn = tick_fn(&raw);
    let cleanup_fn = cleanup_fn(&raw);

    Self{
      _raw: raw,
      init_fn,
      tick_fn,
      cleanup_fn,
    }
  }
}

impl Library {
  pub fn new(dylib_path: OsString) -> Library {
    use notify::Watcher;

    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::raw_watcher(tx).unwrap();

    watcher.watch(&dylib_path, notify::RecursiveMode::NonRecursive).unwrap();

    let instance = Instance::load(&dylib_path);

    Library {
      dylib_path: dylib_path,
      instance: instance,
      rx: rx,
      _watcher: watcher,
    }
  }

  pub fn reload(&mut self) {
    for _ in self.rx.try_recv() { self.do_reload(); }
  }

  fn do_reload(&mut self) {
    println!("Reloading!");

    self.instance = Instance::load(&self.dylib_path);
  }

  pub fn reload_block(&mut self) {
    self.rx.recv().unwrap();
    self.do_reload();
  }
}

fn init_fn(library: &libloading::Library) -> libloading::os::unix::Symbol<extern fn() -> Box<State>> {
  unsafe{ library.get::<extern fn() -> Box<State>>(b"init").unwrap().into_raw() }
}

fn tick_fn(library: &libloading::Library) -> libloading::os::unix::Symbol<extern fn(&mut State) -> bool> {
  unsafe{ library.get::<extern fn(&mut State) -> bool>(b"tick").unwrap().into_raw() }
}

fn cleanup_fn(library: &libloading::Library) -> libloading::os::unix::Symbol<extern fn(Box<State>)> {
  unsafe{ library.get::<extern fn(Box<State>)>(b"cleanup").unwrap().into_raw() }
}
