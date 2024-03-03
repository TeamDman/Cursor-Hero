use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::sprite::Material2d;

#[derive(Component, Debug, Reflect)]
pub struct AppWindow;

#[derive(Component, Debug, Reflect)]
pub struct Taskbar {
    pub size: Vec2,
}

#[derive(Event, Debug, Reflect)]
pub enum TaskbarEvent {
    Populate { taskbar_id: Entity },
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TaskbarMaterial {
    #[uniform(0)]
    pub taskbar_height: f32,
    // pub taskbar_blur_radius: u32,
    // pub taskbar_blur_total_samples: u32,
    #[uniform(0)]
    pub taskbar_color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub wallpaper_texture: Option<Handle<Image>>,
    #[uniform(0)]
    pub wallpaper_size: Vec2,
    pub alpha_mode: AlphaMode,
}

impl Material2d for TaskbarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/taskbar_material.wgsl".into()
    }
}
