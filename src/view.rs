use bevy::{
    input::mouse::MouseWheel,
    math::Vec3Swizzles,
    prelude::{Image as BevyImage, *},
};
use bevy_egui::EguiContext;

use crate::prelude::{Image, *};

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<State>()
            .add_startup_system(setup)
            .add_system(update)
            .add_system(grab)
            .add_system(zoom);
    }
}

#[derive(Component, Default)]
pub struct View {
    pub target_scale: Option<Vec3>,
    pub target_translation: Option<Vec3>,
}

fn setup(mut commands: Commands, mut assets: ResMut<Assets<BevyImage>>) {
    commands.spawn(Camera2dBundle::default());

    let image = Image::default();
    let handle = assets.add(image.into_bevy_image());

    commands
        .spawn(SpriteBundle {
            texture: handle,
            ..default()
        })
        .insert(View::default());
}

fn update(
    handles: Query<&Handle<BevyImage>, With<View>>,
    mut assets: ResMut<Assets<BevyImage>>,
    mut editor: ResMut<Editor>,
) {
    for handle in handles.iter() {
        let image = assets.get_mut(handle).unwrap();
        *image = editor.get_output().into_bevy_image();
    }
}

#[derive(Resource, Default)]
struct State {
    last_mouse_position: Option<Vec2>,
}

fn grab(
    mut query: Query<(&mut Transform, &mut View)>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State>,
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

        if mouse_button_input.pressed(MouseButton::Middle) {
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
    mut query: Query<(&mut Transform, &mut crate::view::View, &Handle<BevyImage>)>,
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
                for (transform, mut sprite, handle) in query.iter_mut() {
                    let to_center_screen = state.last_mouse_position.unwrap_or_default() - {
                        let window = windows.get_primary().unwrap();
                        Vec2::new(window.width(), window.height())
                    } * 0.5;

                    let to_center_image = to_center_screen - transform.translation.xy();

                    let current_size = {
                        if let Some(image) = assets.get(handle) {
                            image.size() * transform.scale.xy()
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

    for (mut transform, sprite, _) in query.iter_mut() {
        if let Some(target_scale) = sprite.target_scale {
            transform.scale = transform.scale.lerp(target_scale, ZOOM_LERP);
        }
        if let Some(target_translation) = sprite.target_translation {
            transform.translation = transform.translation.lerp(target_translation, ZOOM_LERP);
        }
    }
}
