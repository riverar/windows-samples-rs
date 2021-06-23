## Rust for Windows Samples > Project Reunion

This workspace contains an experimental `project_reunion` crate that, similarly to the `windows` crate, lets you call any Project Reunion APIs past, present, and future using code generated on the fly directly from the metadata describing the API. It also includes a number of standalone examples that demo the use of this crate.

## Getting started

Start by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
windows = "0.12.0"
project_reunion = { path = "https://github.com/riverar/windows-samples-rs/", branch="rafael/add-project-reunion-samples" }

[build-dependencies]
windows = "0.12.0"
project_reunion = { path = "https://github.com/riverar/windows-samples-rs/", branch="rafael/add-project-reunion-samples" }
```

This will instruct Cargo to download, build, and cache the Windows and Project Reunion crates.

Next, open a PowerShell instance, navigate to the root of your crate, and issue the following commands:

```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force

Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/riverar/windows-samples-rs/rafael/add-project-reunion-samples/project_reunion/stage.ps1'))
```

This script will download a few Project Reunion NuGet packages and extract a few dependencies into the `.windows` folder.

Next, specify which types you need inside a `build.rs` build script and the `windows` crate will generate the necessary bindings:

```rust
fn main() {
    windows::build! {
        Microsoft::UI::*,
        Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK},
    };
}
```

Finally, make use of the APIs you specified earlier.

```rust
mod bindings {
    windows::include_bindings!();
}

use bindings::{
    Microsoft::UI::*,
    Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK},
};
use project_reunion::bootstrap::PackageVersion;

fn main() -> windows::Result<()> {
    project_reunion::bootstrap::initialize(PackageVersion::new(0, 8, 0, 0))?;

    let color = Colors::MidnightBlue()?;
    let message = format!("Midnight Blue can also be represented as: \n\n{:?}", color);
    unsafe { MessageBoxA(None, message, "Tip", MB_OK); }
    Ok(())
}
```

To reduce build time, use a `bindings` crate rather than simply a module. This will allow Cargo to cache the results and build your project far more quickly.