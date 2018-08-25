
use winit;
use winit::{ VirtualKeyCode, Event, WindowEvent };

use structures::Dimension2D;
use constant::window;

use procedure::workflow::ProgramProc;
use procedure::error::RuntimeError;

struct WindowInfo {
    window_size:  Dimension2D,
    window_title: String,
}

impl WindowInfo {
    fn build(&self, event_loop: &winit::EventsLoop) -> Result<winit::Window, winit::CreationError> {
        winit::WindowBuilder::new()
            .with_title(self.window_title.clone())
            .with_dimensions((self.window_size.width, self.window_size.height).into())
            .build(event_loop)
    }
}

pub struct ProgramBuilder<T> {

    window_info: WindowInfo,
    procedure: T,
}

impl <T> ProgramBuilder<T> where T: ProgramProc {

    pub fn new(procedure: T) -> ProgramBuilder<T> {
        ProgramBuilder {
            window_info: WindowInfo {
                window_size:  window::WINDOW_SIZE,
                window_title: window::WINDOW_TITLE.to_owned(),
            },

            procedure,
        }
    }

    pub fn title(mut self, title: &str) -> ProgramBuilder<T> {
        self.window_info.window_title = title.to_owned();
        self
    }

    pub fn size(mut self, window_width: u32, window_height: u32) -> ProgramBuilder<T> {
        self.window_info.window_size = Dimension2D {
            width:  window_width,
            height: window_height,
        };

        self
    }

    pub fn build(self) -> ProgramEnv<T> {
        ProgramEnv {
            event_loop  : winit::EventsLoop::new(),
            window_info : self.window_info,
            procedure   : self.procedure,
        }
    }
}

pub struct ProgramEnv<T: ProgramProc> {

    event_loop: winit::EventsLoop,
    window_info: WindowInfo,

    pub(super) procedure: T,
}

impl<T> ProgramEnv<T> where T: ProgramProc {

    pub fn launch(&mut self) -> Result<(), RuntimeError> {

        // TODO: Refactor the following two lines
        use core::physical::PhysicalRequirement;
        use constant::core::DEVICE_EXTENSION;
        let requirement = PhysicalRequirement::init()
            .require_queue_extensions(DEVICE_EXTENSION.to_vec());

        let window = self.window_info.build(&self.event_loop)
            .map_err(|e| RuntimeError::Window(e))?;
        let core = self.initialize_core(&window, requirement)
            .map_err(|e| RuntimeError::Procedure(e))?;
        let resources = self.load_resources(&core)
            .map_err(|e| RuntimeError::Procedure(e))?;

        self.main_loop();

        resources.cleanup(&core);
        core.cleanup();

        Ok(())
    }

    fn main_loop(&mut self) {

        let mut is_running       = true;
        let mut is_first_resized = true;

        'mainloop: loop {
            self.event_loop.poll_events(|event| {
                match event {
                    // handling keyboard event
                    | Event::WindowEvent { event, .. } => match event {
                        | WindowEvent::KeyboardInput { input, .. } => {
                            if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                                is_running = false;
                            }
                        }
                        | WindowEvent::Resized(_) => {

                            if is_first_resized {
                                is_first_resized = false;
                            } else {
                                // TODO: Implement resized handling.
                                unimplemented!("Resized is not implemented yet")
                            }

                        },
                        | WindowEvent::CloseRequested => {
                            is_running = false;
                        },
                        | _ => (),
                    },
                    | _ => (),
                }
            });

            if is_running == false {
                break 'mainloop
            }
        }
    }
}
