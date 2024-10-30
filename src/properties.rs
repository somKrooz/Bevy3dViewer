use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct PROPERTIES {
    pub is_screen: bool,
}