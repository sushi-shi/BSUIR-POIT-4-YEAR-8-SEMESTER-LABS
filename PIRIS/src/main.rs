mod forms;
mod state;

use forms::main::Application;
use iced::Sandbox;

pub fn main() -> iced::Result {
    Application::run(iced::Settings::default())
}
