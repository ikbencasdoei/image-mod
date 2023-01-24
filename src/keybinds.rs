use eframe::Frame;
use egui::{Context, Key};

pub fn fullscreen(ctx: &Context, frame: &mut Frame) {
    if ctx.input().key_pressed(Key::F11) {
        if frame.info().window_info.fullscreen {
            frame.set_fullscreen(false)
        } else {
            frame.set_fullscreen(true)
        }
    }
}

pub fn exit(ctx: &Context, frame: &mut Frame) {
    if ctx.input().key_pressed(Key::Escape) {
        frame.close();
    }
}
