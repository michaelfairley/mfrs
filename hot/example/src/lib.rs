extern crate glium;
extern crate glutin;

pub struct State {
  display: glium::backend::glutin::Display,
  events_loop: glutin::EventsLoop,
}

#[no_mangle]
pub extern "C" fn init() -> Box<State> {
  let window = glutin::WindowBuilder::new()
    .with_dimensions(400, 400);
  let context = glutin::ContextBuilder::new();
  let events_loop = glutin::EventsLoop::new();

  let display = glium::Display::new(window, context, &events_loop).unwrap();

  let state = State {
    display,
    events_loop,
  };

  Box::new(state)
}

#[no_mangle]
pub extern "C" fn tick(state: &mut State) -> bool {
  let mut stop = false;

  state.events_loop.poll_events(|event| match event {
    glutin::Event::WindowEvent { event, .. } => match event {
      glutin::WindowEvent::Closed => stop = true,
      glutin::WindowEvent::KeyboardInput { input, .. } => match input.state {
        glutin::ElementState::Pressed => match input.virtual_keycode {
          Some(glutin::VirtualKeyCode::Escape) => stop = true,
          _ => {},
        },
        _ => {},
      },
      _ => {},
    },
    _ => {},
  });

  if stop { return false; }

  {
    use glium::Surface;

    let mut target = state.display.draw();
    target.clear_color(1.0, 0.0, 0.0, 0.0);
    target.finish().unwrap();
  }

  std::thread::sleep(std::time::Duration::from_millis(16));

  true
}

#[no_mangle]
pub extern "C" fn cleanup(_state: Box<State>) {
}
