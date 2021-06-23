fn main() {
    windows::build! {
        Microsoft::Foundation::*,
        Microsoft::UI::Xaml::Controls::Button,
        Microsoft::UI::Xaml::*,
        Windows::Win32::Foundation::{BOOL, RECT},
        Windows::Win32::UI::HiDpi::GetDpiForWindow,
        Windows::Win32::UI::WindowsAndMessaging::*,
    };
}
