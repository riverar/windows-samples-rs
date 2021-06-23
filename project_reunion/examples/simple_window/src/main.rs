#![windows_subsystem = "windows"]

use std::convert::TryFrom;

use project_reunion::bootstrap::{PackageVersion, workarounds};

use bindings::{
    Microsoft,
    Microsoft::UI::Xaml::{
        Application, ApplicationInitializationCallback, Controls::Button, HorizontalAlignment,
        LaunchActivatedEventArgs, RoutedEventHandler, Window,
    },
};

use windows::{IInspectable, Interface, implement};

#[implement(extend Microsoft::UI::Xaml::Application, override OnLaunched)]
struct App {
    _window: Option<Window>,
}

#[allow(non_snake_case)]
impl App {
    fn OnLaunched(&mut self, _: &Option<LaunchActivatedEventArgs>) -> windows::Result<()> {
        let window = Window::new().unwrap();
        window.SetTitle("WinUI Desktop, Unpackaged (Rust)")?;

        let button = Button::new()?;
        button.SetContent(IInspectable::try_from("Click Me")?)?;
        button.SetHorizontalAlignment(HorizontalAlignment::Center)?;
        button.Click(RoutedEventHandler::new(|sender, _args| {
            if let Some(button) = sender {
                button
                    .cast::<Button>()?
                    .SetContent(IInspectable::try_from("Clicked! 🦀")?)?;
            }
            Ok(())
        }))?;

        window.SetContent(&button)?;

        let inspectable = &windows::IInspectable::from(&window);
        workarounds::resize_window(inspectable, 800, 600);
        workarounds::center_window(inspectable);

        let result = window.Activate();
        self._window = Some(window);
        result
    }
}

fn main() -> windows::Result<()> {
    project_reunion::bootstrap::initialize(PackageVersion::new(0, 8, 0, 0)).and_then(|_| {
        Application::Start(ApplicationInitializationCallback::new(|_| {
            App { _window: None }.new()?;
            Ok(())
        }))
    })
}
