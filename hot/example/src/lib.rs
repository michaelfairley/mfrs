extern crate glium;
extern crate glutin;

pub struct State {
  display: glium::backend::glutin_backend::GlutinFacade,
}

#[no_mangle]
pub extern "C" fn init() -> Box<State> {
  use glium::DisplayBuild;

  let display = glutin::WindowBuilder::new()
    .with_dimensions(400, 400)
    .build_glium()
    .unwrap();

  let state = State {
    display: display,
  };

  Box::new(state)
}

#[no_mangle]
pub extern "C" fn tick(state: &State) -> bool {
  for event in state.display.poll_events() {
    match event {
      glutin::Event::Closed
        | glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Escape))
        => return false,
      _ => ()
    }
  }

  {
    use glium::Surface;

    let mut target = state.display.draw();
    target.clear_color(1.0, 0.0, 0.0, 0.0);
    target.finish().unwrap();
  }

  std::thread::sleep(std::time::Duration::from_millis(16));

  true
}

pub extern "C" fn cleanup(_state: Box<State>) {
}
