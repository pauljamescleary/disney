# Disney Home Screen

This application renders a demo Disney home screen and allows the user to scroll for programs / content.

## Pre-requisites

1. [Install Rust][install-rust]
2. [Install Clippy][install-clippy]
3. [Install SDL2][install-sdl2] - on a mac `brew install sdl2` is sufficient
4. Run `cargo install cargo-vcpkg` to install vcpkg needed to build and run this app

## Developing

- `cargo vcpkg build` to build the sdl2 dependencies (this will take some time)
- `cargo clippy` - runs the linter to ensure the code is clean
- `cargo fmt` - formats all code
- `cargo run` - runs the application

## Running

- `cargo run` will start the application.  All log entries will appear in stdout in the terminal.

### Navigating

- `key down` - scrolls down to the next row
- `key up` - scrolls up to the previous row
- `key left` - scrolls to the next program on a row
- `key right` - scrolls to the prior program on a row
- `ESC` - to quit
 
## Design

This application uses [SDL2][sdl2] for the main UI engine, which is a `rust` library that is actively maintained.

### Code and libraries

- [Reqwest][reqwest] - for http calls, can support blocking and non-blocking HTTP.  Using a client enables pooling of connections as SSL is the long leg of downloading content
- [Anyhow][anyhow] - for all error handling, has convenient syntax for propagating and managing errors accross the application.  All `Result`s are actually `anyhow::Result`
- [Futures][futures] - for future and stream extensions.  Streaming enables memory safe, controlled gathering of data
- [Tokio][tokio] - for the async runtime

### Concurrency

The application runs under the [Tokio runtime][tokio], but most of the application itself runs on a single main thread.
Tokio is used to enable async processing, specifically for the loading of HTTP data, content sets, and images.
The overall design can be modified to support different threading models (including using rust native spawn), but 
Tokio was chosen for familiarity and features.

Concurrency might need to be constrained (or impossible?) on devices with more limited resources.  The `concurrency` is 
hard-coded to "20" concurrent futures being run.  Since that is all non-blocking, that can easily be changed based 
on the profile of the device, or passed in as a config argument.

### Screen Size

For the screen size, I chose 1920x1080 as the default, and hard coded the image size to be "1.78", which is 
the ration of width to height for the images that are chosen in the application.

### Image loading

Images are loaded in a background thread that pipes them into the application's main event loop.  This is done via 
the `EventSender` from SDL2.  The application batches images "aribtrarily" into groups of 15, as that seemed 
to be a sweet spot for updating the UI.

### Rendering

When the application is updated via a keystroke or an image loading, the `viewport` of the window is analyzed 
to see if the current component is "in view".  If not, rendering will be skipped.  This cuts down on 
blocking the entire application until all images are loaded.

### JSON

The application uses [Serde JSON][serde] for json deserialization.  This makes deserialization simple via derivation, 
but is likely not the most optimal way to load data.  Given more time, creating a streaming Deserializer would be preferable, 
as we can immediately fetch missing content sets (ref) and start fetching images immediately.  The current 
application requires the entire home screen json to be parsed before proceeding with loading the UI.

[anyhow]: https://docs.rs/anyhow/latest/anyhow/
[futures]: https://docs.rs/futures/latest/futures/
[install-clippy]: https://github.com/rust-lang/rust-clippy#as-a-cargo-subcommand-cargo-clippy
[install-rust]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install-sdl2]: https://github.com/Rust-SDL2/rust-sdl2
[reqwest]: https://docs.rs/reqwest/latest/reqwest/
[sdl2]: https://docs.rs/sdl2/latest/sdl2/
[serde]: https://serde.rs/
[tokio]: https://tokio.rs/
