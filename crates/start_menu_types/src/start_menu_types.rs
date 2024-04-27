use bevy::prelude::*;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use bevy_xpbd_2d::components::Sensor;
use cursor_hero_cursor_types::prelude::Clickable;
use cursor_hero_cursor_types::prelude::Hoverable;

#[derive(Component, Debug, Reflect)]
pub struct StartMenuButton;

#[derive(Component, Debug, Reflect)]
pub struct StartMenuPanel;

#[derive(Event, Debug, Reflect)]
pub enum StartMenuPanelVisibilityChangeRequestEvent {
    Open { start_menu_button_id: Entity },
    Close { start_menu_button_id: Entity },
}

#[derive(Event, Debug, Reflect, Clone, Copy)]
pub struct StartMenuPopulateEvent {
    pub button: Entity,
    pub panel: Entity,
}

pub struct StartMenuPanelAppLauncherIconBuilder<T>
where
    T: Bundle,
{
    pub tag: T,
    pub name: Option<String>,
    pub texture: Handle<Image>,
    pub position: Vec2,
    pub size: Vec2,
}

#[derive(Debug)]
pub enum StartMenuPanelAppLauncherIconBuilderError {
    MissingName,
    MissingTexture,
}

impl<T> StartMenuPanelAppLauncherIconBuilder<T>
where
    T: Bundle,
{
    pub fn new(tag: T) -> Self {
        Self {
            tag,
            name: None,
            texture: Handle::default(),
            size: Vec2::new(100.0, 100.0),
            position: Vec2::ZERO,
        }
    }

    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_texture(mut self, texture: Handle<Image>) -> Self {
        self.texture = texture;
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }
    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }

    pub fn spawn(
        self,
        panel: &Entity,
        commands: &mut Commands,
    ) -> Result<(), StartMenuPanelAppLauncherIconBuilderError> {
        let name = self
            .name
            .ok_or(StartMenuPanelAppLauncherIconBuilderError::MissingName)?;
        let texture = self.texture;
        if texture == Handle::default() {
            return Err(StartMenuPanelAppLauncherIconBuilderError::MissingTexture);
        }

        commands.entity(*panel).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(self.size),
                        ..Default::default()
                    },
                    texture,
                    transform: Transform::from_translation(self.position.extend(1.0)),
                    ..Default::default()
                },
                Name::new(name),
                Hoverable,
                Clickable,
                RigidBody::Static,
                Sensor,
                Collider::cuboid(self.size.x, self.size.y),
                self.tag,
            ));
        });
        Ok(())
    }
}
