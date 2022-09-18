use std::marker::PhantomData;

use bevy::{
    math::Vec3Swizzles,
    prelude::{Image as BevyImage, *},
    reflect::GetTypeRegistration,
};
use bevy_egui::EguiContext;

use crate::{
    image::ImageHelper,
    tools::plugin::{Tool, ToolPlugin},
    viewer::Sprite,
};

pub trait PencilTool {
    fn get_draw_color(&mut self, mouse_position: Vec2) -> [u8; 4];
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
    mut query: Query<(&Sprite, &mut T, &Transform)>,
    mut egui_context: ResMut<EguiContext>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut assets: ResMut<Assets<BevyImage>>,
    windows: Res<Windows>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut local: Local<PencilLocal>,
) where
    T: Component + PencilTool,
{
    if (mouse_button_input.pressed(MouseButton::Left)
        || mouse_button_input.pressed(MouseButton::Right))
        && !egui_context.ctx_mut().wants_pointer_input()
    {
        for (sprite, mut pencil, transform) in query.iter_mut() {
            if let Some(image) = assets.get_mut(sprite.image.as_ref().unwrap()) {
                let image_size = image.size();

                let window_size = {
                    let window = windows.get_primary().unwrap();
                    Vec2::new(window.width(), window.height())
                };

                let mut helper = ImageHelper::new(image);

                let bottom_left_corner_on_screen =
                    (window_size - image_size * transform.scale.xy()) * 0.5
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

                                helper.set_pixel(position, pencil.get_draw_color(position));
                            }
                        }
                        helper.set_pixel(mouse_on_image, pencil.get_draw_color(mouse_on_image));
                    }
                    local.last_mouse_position = Some(mouse_on_image);
                }
            }
        }
    } else {
        local.last_mouse_position = None;
    }
}
