use eframe::egui::{Context, Key, ViewportCommand};

pub fn fullscreen(ctx: &Context) {
    if ctx.input(|input| input.key_pressed(Key::F11)) {
        if ctx.input(|input| {
            input
                .viewport()
                .fullscreen
                .is_some_and(|fullscreen| fullscreen)
        }) {
            ctx.send_viewport_cmd(ViewportCommand::Fullscreen(false))
        } else {
            ctx.send_viewport_cmd(ViewportCommand::Fullscreen(true))
        }
    }
}

pub fn exit(ctx: &Context) {
    if ctx.input(|input| input.key_pressed(Key::Escape)) {
        ctx.send_viewport_cmd(ViewportCommand::Close);
    }
}
