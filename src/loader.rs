use bevy::prelude::*;
use rfd::FileDialog;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use crate::properties::PROPERTIES;

pub struct LoaderPlugin;

#[derive(Resource)]
struct KROOZ {
    scale: f32,
    x: f32,
    y: f32,
    z: f32,
}

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EguiPlugin)
            .insert_resource(KROOZ { scale: 1.0, x: 0.0, y: 0.0, z: 0.0 }) // Start scale at 1.0, position at (0,0,0)
            .insert_resource(CurrentSceneHandle(None))
            .add_systems(Update, ui_system)
            .add_systems(Update, open_file_picker)
            .add_systems(Update, transform_system); // Update the system name to transform_system
    }
}

#[derive(Resource)]
struct CurrentSceneHandle(Option<Entity>);

#[derive(Component)]
struct Scalable; 

fn ui_system(mut contexts: EguiContexts, mut krooz: ResMut<KROOZ> , pro: Res<PROPERTIES>) {

    if !pro.is_screen
    {
        egui::Window::new("Properties").id(egui::Id::new("rnd")).show(contexts.ctx_mut(), |ui| {
        // Scale slider
        ui.add(egui::Slider::new(&mut krooz.scale, 0.01..=10.0).text("Scale"));
        // Position sliders
        ui.add(egui::Slider::new(&mut krooz.x, -10.0..=10.0).text("Position X"));
        ui.add(egui::Slider::new(&mut krooz.y, -10.0..=10.0).text("Position Y"));
        ui.add(egui::Slider::new(&mut krooz.z, -10.0..=10.0).text("Position Z"));
    });
    }

}

fn open_file_picker(
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    krooz: Res<KROOZ>,
    mut current_scene_handle: ResMut<CurrentSceneHandle>,
) {
    if input.just_pressed(KeyCode::Enter) {
        if let Some(path) = FileDialog::new()
            .add_filter("glTF Files", &["gltf", "glb"])
            .pick_file()
        {
            let setter = "#Scene0";
            let path_str = format!("{}{}", path.to_str().unwrap().to_string(), setter);
            let handle = asset_server.load(&path_str);

            if let Some(entity) = current_scene_handle.0 {
                commands.entity(entity).despawn_recursive();
            }

            let entity = commands.spawn(SceneBundle {
                scene: handle,
                transform: Transform {
                    translation: Vec3::new(krooz.x, krooz.y, krooz.z), // Initial position
                    scale: Vec3::new(krooz.scale, krooz.scale, krooz.scale), // Initial scale
                    ..default()
                },
                ..Default::default()
            }).insert(Scalable) // Mark the entity as scalable
            .id();

            current_scene_handle.0 = Some(entity);
        }
    }
}

// System to apply scale and position dynamically
fn transform_system(
    mut query: Query<(&Scalable, &mut Transform)>, // Query scalable entities
    krooz: Res<KROOZ>,
) {
    for (_, mut transform) in query.iter_mut() {
        // Update the scale and position based on the KROOZ resource values
        transform.scale = Vec3::new(krooz.scale, krooz.scale, krooz.scale);
        transform.translation = Vec3::new(krooz.x, krooz.y, krooz.z);
    }
}
