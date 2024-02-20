use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::sprite::Material2d;
use bevy::reflect::TypePath;

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
    pub color: Color,
}

impl Material2d for TaskbarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/taskbar_material.wgsl".into()
    }
}
