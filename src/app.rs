use crate::engine::Engine;

use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::WindowAttributes,
};

pub struct App {
    engine: Option<Engine>,
}

impl App {
    pub fn new() -> Self {
        Self { engine: None }
    }
}

impl ApplicationHandler<Engine> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default().with_title("N Body Sim");
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        self.engine = Some(pollster::block_on(Engine::new(window)));
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: Engine) {
        self.engine = Some(event)
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let engine = match &mut self.engine {
            Some(s) => s,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => engine.handle_resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                engine.update();
                match engine.render() {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Error rendering: {}", e);
                    }
                };
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => engine.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
    }
}
