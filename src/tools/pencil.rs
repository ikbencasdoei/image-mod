use crate::{image::ImageHelper, viewer::Sprite};
use bevy::{math::Vec3Swizzles, prelude::*, reflect::FromReflect};
use bevy_egui::{
    egui::{self, Color32},
    EguiContext,
};

use super::plugin::{Tool, ToolDescription, ToolPlugin};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ToolPlugin::<PencilTool>::default())
            .insert_resource(State::default())
            .add_system(do_stuff)
            .add_system(ui)
            .add_system(help);
    }
}

#[derive(Component, Reflect, FromReflect)]
pub struct PencilTool {
    #[reflect(ignore)]
    primary_color: Color32,
    #[reflect(ignore)]
    secondary_color: Color32,
    counter: i32,
}

impl Default for PencilTool {
    fn default() -> Self {
        PencilTool {
            primary_color: Color32::WHITE,
            secondary_color: Color32::BLACK,
            counter: 0,
        }
    }
}

impl Tool<PencilTool> for PencilTool {
    fn get_description() -> ToolDescription {
        ToolDescription {
            name: "3: pencil".to_string(),
        }
    }
}

#[derive(Default)]
struct State {
    last_mouse_position: Option<Vec2>,
}

fn do_stuff(
    query: Query<(&Sprite, &PencilTool, &Transform)>,
    mut egui_context: ResMut<EguiContext>,
    mut assets: ResMut<Assets<Image>>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut state: ResMut<State>,
) {
    let window_size = {
        let window = windows.get_primary().unwrap();
        Vec2::new(window.width(), window.height())
    };

    for (sprite, pencil, transform) in query.iter() {
        if let Some(image) = assets.get_mut(sprite.image.as_ref().unwrap()) {
            let size = image.size();
            let mut helper = ImageHelper::new(image);

            if (mouse_button_input.pressed(MouseButton::Left)
                || mouse_button_input.pressed(MouseButton::Right))
                && !egui_context.ctx_mut().wants_pointer_input()
            {
                let color = if mouse_button_input.pressed(MouseButton::Left) {
                    pencil.primary_color
                } else {
                    pencil.secondary_color
                };

                paint(
                    transform,
                    &mut cursor_moved_events,
                    &mut state,
                    window_size,
                    size,
                    &mut helper,
                    color,
                );
            } else {
                state.last_mouse_position = None;
            }
        }
    }
}

fn paint(
    transform: &Transform,
    cursor_moved_events: &mut EventReader<CursorMoved>,
    state: &mut ResMut<State>,
    window_size: Vec2,
    size: Vec2,
    helper: &mut ImageHelper,
    color: Color32,
) {
    let bottom_left_corner_on_screen =
        (window_size - size * transform.scale.xy()) * 0.5 + transform.translation.xy();

    for event in cursor_moved_events.iter() {
        let mut mouse_on_image =
            (event.position - bottom_left_corner_on_screen) / transform.scale.xy();

        mouse_on_image.y = size.y - mouse_on_image.y;

        if let Some(last_mouse_position) = state.last_mouse_position {
            let delta: Vec2 = mouse_on_image - last_mouse_position;

            if delta.length() > 1.0 {
                for i in 1..delta.length().ceil() as i32 {
                    helper.set_pixel(
                        last_mouse_position
                            .lerp(mouse_on_image, 1.0 / delta.length().ceil() * (i as f32)),
                        color.to_array(),
                    );
                }
            }
            helper.set_pixel(mouse_on_image, color.to_array());
        }
        state.last_mouse_position = Some(mouse_on_image);
    }
}

fn ui(mut query: Query<&mut PencilTool>, mut egui_context: ResMut<EguiContext>) {
    for mut pencil in query.iter_mut() {
        egui::Window::new("Pencil").show(egui_context.ctx_mut(), |ui| {
            ui.label("primary color:");
            ui.color_edit_button_srgba(&mut pencil.primary_color);
            ui.label("secondary color:");
            ui.color_edit_button_srgba(&mut pencil.secondary_color);
        });
    }
}

fn help(mut egui_context: ResMut<EguiContext>, query: Query<&PencilTool>) {
    for _ in query.iter() {
        egui::Window::new("Pencil Help").show(egui_context.ctx_mut(), |ui| {
            ui.label("Use the right mouse button to paint the primary color and use the left to paint the secondary color.");
        });
    }
}
