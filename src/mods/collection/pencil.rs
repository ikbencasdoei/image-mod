use bevy::prelude::{Image as BevyImage, *};
use bevy_egui::{
    egui::{Color32, Ui},
    EguiContext,
};

use crate::prelude::{Color, Image, *};

pub struct PencilPlugin;

impl Plugin for PencilPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ModifierCollection>()
            .add_plugin(ModifierPlugin::<Pencil>::default())
            .add_system(update);
    }
}

#[derive(Clone, PartialEq)]
pub struct Pencil {
    color: Color32,
    pixels: Vec<UVec2>,
}

impl Default for Pencil {
    fn default() -> Self {
        Self {
            color: Color32::BLACK,
            pixels: default(),
        }
    }
}

fn update(
    mut editor: ResMut<Editor>,
    mut cursor_events: EventReader<CursorMoved>,
    mut mouse_pos: Local<Vec2>,
    mut last_mouse_pos: Local<Option<Vec2>>,
    mouse_input: Res<Input<MouseButton>>,
    mut egui_context: ResMut<EguiContext>,
    query: Query<(&Transform, &Handle<BevyImage>), With<View>>,
    windows: Res<Windows>,
    assets: Res<Assets<BevyImage>>,
) {
    if let Some(event) = cursor_events.iter().last() {
        *last_mouse_pos = Some(*mouse_pos);
        *mouse_pos = event.position;
    } else {
        *last_mouse_pos = None;
    }

    if let Some(pencil) = editor.get_when_selected_mut::<Pencil>() {
        if (mouse_input.pressed(MouseButton::Left)) && !egui_context.ctx_mut().wants_pointer_input()
        {
            for (transform, handle) in query.iter() {
                let pixel =
                    View::screen_to_pixel(*mouse_pos, transform, &windows, &assets, &handle);

                if let Some(last_mouse_pos) = *last_mouse_pos {
                    let last_pixel = View::screen_to_pixel(
                        last_mouse_pos,
                        transform,
                        &windows,
                        &assets,
                        &handle,
                    );

                    let delta: Vec2 = pixel - last_pixel;

                    if delta.length() > 1.0 {
                        for i in 1..delta.length().ceil() as i32 {
                            let position =
                                last_pixel.lerp(pixel, 1.0 / delta.length().ceil() * (i as f32));

                            pencil.pixels.push(position.as_uvec2());
                        }
                    }
                }

                pencil.pixels.push(pixel.as_uvec2());
            }
        }
    }
}

impl Modifier for Pencil {
    fn apply(&mut self, mut input: Option<Image>) -> Option<Image> {
        if let Some(image) = &mut input {
            for pixel in self.pixels.iter() {
                image.set_pixel(*pixel, Color::from(self.color)).ok();
            }
        }
        input
    }

    fn view(&mut self, ui: &mut Ui) {
        ui.label("color");
        ui.color_edit_button_srgba(&mut self.color);
    }
}
