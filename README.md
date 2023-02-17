![preview image](./preview.png)
# Image-mod
A experimental, remembering, work in progress image editor written in Rust.

## Concept
Image-mod works using a dynamic combination of so-called modifiers: each representing a step in the editing process. This means that changes are never fixed and can always be removed, inserted, reordered and edited.

## Technologies used
* [Rust](https://www.rust-lang.org/)
* [egui](https://www.egui.rs/)
* [image](https://github.com/image-rs/image)
* [Rfd](https://github.com/PolyMeilex/rfd)


## Running
Like most other Rust crates, simply `cargo run`. The libraries used work cross-platform.
### Linux
For running on linux egui/eframe requires some [dependencies](https://github.com/emilk/egui/tree/master/crates/eframe).  
