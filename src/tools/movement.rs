use crate::view::View;
use bevy::{
    input::mouse::MouseWheel,
    math::Vec3Swizzles,
    prelude::{Image as BevyImage, *},
};
use bevy_egui::{egui, egui::util::id_type_map::TypeId, EguiContext};

use super::{
    plugin::{Tool, ToolDescription, ToolPlugin},
    CurrentTool,
};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(State::default())
            .add_plugin(ToolPlugin::<GrabTool>::default())
            .add_system(grab)
            .add_system(zoom)
            .add_system(help);
    }
}

#[derive(Resource, Default)]
struct State {
    last_mouse_position: Option<Vec2>,
}

#[derive(Component, Default, Reflect)]
pub struct GrabTool;

impl Tool<GrabTool> for GrabTool {
    fn get_description() -> ToolDescription {
        ToolDescription {
            name: "Grab".to_string(),
        }
    }
}

fn grab(
    mut query: Query<(&mut Transform, &mut View)>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State>,
    current_tool: Res<CurrentTool>,
) {
    if !egui_context.ctx_mut().wants_pointer_input() {
        let delta = if let Some(cursor) = cursor_moved_events.iter().last() {
            let delta = if let Some(position) = state.last_mouse_position {
                position - cursor.position
            } else {
                Vec2::ZERO
            };

            state.last_mouse_position = Some(cursor.position);

            -delta
        } else {
            Vec2::ZERO
        };

        let tool_active = if let Some(current_tool) = current_tool.to_owned() {
            current_tool.type_id == TypeId::of::<GrabTool>()
        } else {
            false
        };

        if tool_active
            && (mouse_button_input.pressed(MouseButton::Right)
                || mouse_button_input.pressed(MouseButton::Left))
            || mouse_button_input.pressed(MouseButton::Middle)
        {
            for (mut transform, mut sprite) in query.iter_mut() {
                transform.translation += Vec3::from((delta, 0.0));
                sprite.target_translation = Some(transform.translation);
            }
        }
    } else {
        state.last_mouse_position = None;
    }

    cursor_moved_events.clear();
}

const ZOOM_FACTOR: f32 = 1.3;
const ZOOM_LERP: f32 = 0.3;

fn zoom(
    mut query: Query<(&mut Transform, &mut crate::view::View)>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
    assets: Res<Assets<BevyImage>>,
    state: Res<State>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if !(egui_context.ctx_mut().wants_pointer_input()
        || mouse_button_input.pressed(MouseButton::Left)
        || mouse_button_input.pressed(MouseButton::Right)
        || mouse_button_input.pressed(MouseButton::Middle))
    {
        for event in mouse_wheel_events.iter() {
            if event.y.is_normal() {
                for (transform, mut sprite) in query.iter_mut() {
                    let to_center_screen = state.last_mouse_position.unwrap_or_default() - {
                        let window = windows.get_primary().unwrap();
                        Vec2::new(window.width(), window.height())
                    } * 0.5;

                    let to_center_image = to_center_screen - transform.translation.xy();

                    let current_size = {
                        if let Some(image) = sprite.image.as_ref() {
                            if let Some(image) = assets.get(image) {
                                image.size() * transform.scale.xy()
                            } else {
                                Vec2::ZERO
                            }
                        } else {
                            Vec2::ZERO
                        }
                    };

                    let image_center_to_corner = current_size / 2.0;
                    let mouse_corner_factor = to_center_image / image_center_to_corner;

                    if event.y.is_sign_positive() {
                        let size_after = current_size * ZOOM_FACTOR;
                        let corner_diff = (size_after - current_size) / 2.0;

                        let new_scale = transform.scale * ZOOM_FACTOR;

                        if new_scale.max_element() > 250. {
                            return;
                        }

                        sprite.target_scale = Some(transform.scale * ZOOM_FACTOR);
                        sprite.target_translation = Some(
                            transform.translation
                                - Vec3::from((corner_diff * mouse_corner_factor, 0.0)),
                        );
                    } else {
                        let size_after = current_size / ZOOM_FACTOR;
                        let corner_diff = (current_size - size_after) / 2.0;

                        sprite.target_scale = Some(transform.scale / ZOOM_FACTOR);
                        sprite.target_translation = Some(
                            transform.translation
                                + Vec3::from((corner_diff * mouse_corner_factor, 0.0)),
                        );
                    }
                }
            }
        }
    } else {
        mouse_wheel_events.clear();
    }

    for (mut transform, sprite) in query.iter_mut() {
        if let Some(target_scale) = sprite.target_scale {
            transform.scale = transform.scale.lerp(target_scale, ZOOM_LERP);
        }
        if let Some(target_translation) = sprite.target_translation {
            transform.translation = transform.translation.lerp(target_translation, ZOOM_LERP);
        }
    }
}

fn help(mut egui_context: ResMut<EguiContext>, query: Query<&GrabTool>) {
    for _ in query.iter() {
        egui::Window::new(format!("{} Help", GrabTool::get_description().name)).show(
            egui_context.ctx_mut(),
            |ui| {
                ui.label("Use right and left mouse button grab the image.");
            },
        );
    }
}
