extern crate notify;
extern crate libloading;
extern crate tempdir;

use std::sync::mpsc;
use std::ffi::OsString;

pub struct State();

pub struct Library {
  tempdir: tempdir::TempDir,
  dylib_path: OsString,
  _watcher: notify::RecommendedWatcher,
  rx: mpsc::Receiver<notify::RawEvent>,
  pub instance: Instance,
}

pub struct Instance {
  name: OsString,
  _raw: libloading::Library,
  pub init_fn: libloading::os::unix::Symbol<extern fn() -> Box<State>>,
  pub tick_fn: libloading::os::unix::Symbol<extern fn(&mut State) -> bool>,
  pub cleanup_fn: libloading::os::unix::Symbol<extern fn(Box<State>)>,
}

impl Instance {
  fn load(
    dylib_path: &OsString,
    tempdir: &tempdir::TempDir,
  ) -> Self {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    let now_in_ms = now.as_secs() * 1000 + now.subsec_nanos() as u64 / 1_000_000;
    let name = format!("{}.dylib", now_in_ms).into();

    let new_path = tempdir.path().join(&name);

    std::fs::copy(dylib_path, &new_path).unwrap();

    let output = std::process::Command::new("install_name_tool")
      .arg("-id").arg(&new_path).arg(&new_path)
      .output()
      .unwrap();

    if !output.status.success() { panic!(output); }

    let raw = libloading::Library::new(new_path).expect("Couldn't find dylib");

    let init_fn = init_fn(&raw);
    let tick_fn = tick_fn(&raw);
    let cleanup_fn = cleanup_fn(&raw);

    Self{
      name,
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

    let tempdir = tempdir::TempDir::new_in(
      AsRef::<std::path::Path>::as_ref(&dylib_path).parent().unwrap(),
      AsRef::<std::path::Path>::as_ref(&dylib_path).file_name().unwrap().to_str().unwrap(),
    ).unwrap();

    let instance = Instance::load(&dylib_path, &tempdir);

    Library {
      dylib_path: dylib_path,
      instance: instance,
      rx: rx,
      _watcher: watcher,
      tempdir: tempdir,
    }
  }

  pub fn reload(&mut self) {
    for _ in self.rx.try_recv() { self.do_reload(); }
  }

  fn do_reload(&mut self) {
    println!("Reloading!");

    let prev_instance = std::mem::replace(&mut self.instance, Instance::load(&self.dylib_path, &self.tempdir));

    let prev_path = self.tempdir.path().join(&prev_instance.name);

    std::fs::remove_file(prev_path).unwrap();
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
