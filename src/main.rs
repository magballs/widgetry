use winit::{
    application::ApplicationHandler,
    dpi::{LogicalPosition, LogicalSize},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};

struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let monitors: Vec<_> = event_loop.available_monitors().collect();
        let second_monitor = monitors.get(1).expect("No second monitor found!");
        let monitor_pos = second_monitor.position();

        let window_attributes = Window::default_attributes()
            .with_title("Widgetry")
            .with_inner_size(LogicalSize::new(500.0, 200.0))
            .with_position(LogicalPosition::new(
                monitor_pos.x as f64 + 100.0,
                monitor_pos.y as f64 + 100.0,
            ))
            .with_resizable(false)
            .with_decorations(false);

        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create window.");

        self.window = Some(window);
    }

    fn window_event(
            &mut self,
            _event_loop: &ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: WindowEvent,
        ) {
            if let Some(ref window) = self.window {
                if window.id() == window_id {
                    match event {
                        WindowEvent::CloseRequested => {
                            println!("Close requested");
                        }
                        _ => {}
                    }
                }
            }
        }
    }

fn main() -> Result<(), winit::error::EventLoopError> {
    let event_loop = EventLoop::new()?;
    let mut app = App { window: None };
    event_loop.run_app(&mut app)
}
