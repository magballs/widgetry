use iced::Task;
use iced::{Application, Settings};

fn main() -> iced::Result {
    WidgetryApp::run(Settings::default())
}

use nvml_wrapper::Nvml;
use nvml_wrapper::device::Device;

struct WidgetryApp<'a> {
    nvml: Nvml,
    device: Device<'a>,
    used_mib: u64,
    total_mib: u64,
}

enum Message {
    Tick,
    VramPolled(Result<(u64, u64), String>),
}

use iced::widget::{column, text};
use iced::{Element, Subscription, Theme, executor};
use std::time::Duration;

impl<'a> Application for WidgetryApp<'a> {
    type Executor = executor::Default;
    type Message = message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Task<Self::Message>) {
        // Initialize nvml and device
        let nvml = Nvml::init().expect("Failed to init Nvml");
        let device = nvml.device_by_index(0).expect("No GPU at index 0");

        (
            WidgetryApp {
                nvml,
                device,
                used_mib: 0,
                total_mib: 0,
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        String::from("WidgetryApp VRAM Sampler")
    }

    fn Theme(&self) -> Theme {
        Theme::Dark
    }

    fn Subscription(&self) -> Subscription<self::Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::Tick => {
                // Re-initialize NVML each time instead of trying to clone Device
                let task = async {
                    let nvml = Nvml::init().map_err(|e| e.to_string())?;
                    let device = nvml.device_by_index(0).map_err(|e| e.to_string())?;

                    device
                        .memory_info()
                        .map(|mem| (mem.used / 1024 / 1024, mem.total / 1024 / 1024))
                        .map_err(|e| e.to_string())
                };

                Task::perform(task, Message::VramPolled)
            }

            Message::VramPolled(Ok((used, total))) => {
                self.used_mib = used;
                self.total_mib = total;
                Task::none()
            }

            Message::VramPolled(Err(err)) => {
                eprintln!("VRAM polling error: {err}");
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let label = format!("VRAM usage: {} / {} MiB", self.used_mib, self.total_mib);
        column![text(label)].into()
    }
}
