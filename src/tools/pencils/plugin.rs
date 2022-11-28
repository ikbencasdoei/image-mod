use std::marker::PhantomData;

use bevy::{math::Vec3Swizzles, prelude::*, reflect::GetTypeRegistration};
use bevy_egui::EguiContext;

use crate::{
    color::Color,
    image::Image,
    project::Project,
    tools::plugin::{Tool, ToolPlugin},
    view::View,
};

pub trait PencilTool {
    fn get_draw_color(&mut self, mouse_position: UVec2, image: &mut Image) -> Option<Color>;
}

#[derive(Default)]
pub struct PencilPlugin<T>(PhantomData<T>);

impl<T> Plugin for PencilPlugin<T>
where
    T: Tool<T> + Component + Default + GetTypeRegistration + PencilTool,
{
    fn build(&self, app: &mut App) {
        app.add_plugin(ToolPlugin::<T>::default())
            .add_system(process::<T>);
    }
}

#[derive(Default)]
struct PencilLocal {
    last_mouse_position: Option<Vec2>,
}

fn process<T>(
    mut query: Query<(&mut T, &Transform), With<View>>,
    mut egui_context: ResMut<EguiContext>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut local: Local<PencilLocal>,
    mut project: ResMut<Project>,
) where
    T: Component + PencilTool,
{
    if (mouse_button_input.pressed(MouseButton::Left)
        || mouse_button_input.pressed(MouseButton::Right))
        && !egui_context.ctx_mut().wants_pointer_input()
    {
        for (mut pencil, transform) in query.iter_mut() {
            let mut image = &mut project.input;
            let image_size = image.size().as_vec2();

            let window_size = {
                let window = windows.get_primary().unwrap();
                Vec2::new(window.width(), window.height())
            };

            let bottom_left_corner_on_screen = (window_size - image_size * transform.scale.xy())
                * 0.5
                + transform.translation.xy();

            for event in cursor_moved_events.iter() {
                let mut mouse_on_image =
                    (event.position - bottom_left_corner_on_screen) / transform.scale.xy();

                mouse_on_image.y = image_size.y - mouse_on_image.y;

                if let Some(last_mouse_position) = local.last_mouse_position {
                    let delta: Vec2 = mouse_on_image - last_mouse_position;

                    if delta.length() > 1.0 {
                        for i in 1..delta.length().ceil() as i32 {
                            let position = last_mouse_position
                                .lerp(mouse_on_image, 1.0 / delta.length().ceil() * (i as f32));

                            if let Some(color) =
                                pencil.get_draw_color(position.as_uvec2(), &mut image)
                            {
                                image.set_pixel(position.as_uvec2(), color).ok();
                            }
                        }
                    }
                    if let Some(color) =
                        pencil.get_draw_color(mouse_on_image.as_uvec2(), &mut image)
                    {
                        image.set_pixel(mouse_on_image.as_uvec2(), color).ok();
                    }
                }
                local.last_mouse_position = Some(mouse_on_image);
            }
        }
    } else {
        local.last_mouse_position = None;
    }

    cursor_moved_events.clear();
}
