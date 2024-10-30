use bevy::prelude::*;
use std::process::exit;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_blendy_cameras::*;
use bevy_atmosphere::plugin::*;

mod loader;
use loader::LoaderPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, LoaderPlugin,BlendyCamerasPlugin,AtmospherePlugin))
        .add_systems(
            Startup,
            (spawn_camera, spawn_floor, spawn_light),
        )
        .add_systems(Update, cursor_grab)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
    Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        ..default()
    },
    OrbitCameraController{
        force_update: true,
        zoom_to_mouse_position: true,
        ..Default::default()
    },
    FlyCameraController {
        is_enabled: false,
        ..default()
    },
    AtmosphereCamera::default()
));
}

fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    input: Res<ButtonInput<KeyCode>>, // Changed to Res<Input<KeyCode>> for handling keyboard input
) {
    let mut primary_window = q_windows.single_mut();
     primary_window.title = "Ruty-GLTF".to_string();
    if input.just_pressed(KeyCode::KeyG) {
        primary_window.cursor.grab_mode = match primary_window.cursor.grab_mode {
            CursorGrabMode::Locked => CursorGrabMode::None, // Unlock if currently locked
            _ => CursorGrabMode::Locked, // Lock if currently unlocked
        };
        primary_window.cursor.visible = !primary_window.cursor.visible;
    }

    
    if input.just_pressed(KeyCode::Escape) {
        exit(0);
    }
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut material = StandardMaterial::default();
    material.base_color = Color::srgb(0.1, 0.1, 0.1);
    material.perceptual_roughness = 0.2;
    material.double_sided = true;

    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d::default().mesh().size(10000.0, 10000.0))),
        material: materials.add(material) ,
        ..default()
    };

    commands.spawn(floor);
}

fn spawn_light(mut commands: Commands) {
    // Main Point Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 100000.0,
            range: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 100.0, 0.0),
        ..default()
    });

    // Additional Directional Light for general ambient lighting
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 5000.0, // Change this value based on preferred brightness
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-0.5)),
        ..default()
    });
}
