use bevy::prelude::*;
use std::process::exit;
// use std::process::Command;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_blendy_cameras::*;
use bevy_atmosphere::plugin::*;
use bevy::render::view::screenshot::ScreenshotManager;
use bevy_egui::{egui, EguiContexts};
mod loader;
pub mod properties;
use loader::LoaderPlugin;
use crate::properties::PROPERTIES;


// #[derive(Resource)]
// struct ScreenshotTimer(Timer);

#[derive(Resource)]
struct Frames{
    max_frames: f32,
    ctr:f32
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, LoaderPlugin,BlendyCamerasPlugin,AtmospherePlugin))
        .add_systems(
            Startup,
            (spawn_camera, spawn_floor, spawn_light),
        )
        .insert_resource(Frames {max_frames:  1.0 ,ctr: 0.0})
        // .insert_resource(ScreenshotTimer(Timer::from_seconds(0.1, TimerMode::Once))) // Correct Timer initialization 
        .add_systems(Update, cursor_grab)
        .add_systems(Update , render_system)
        .insert_resource(PROPERTIES { is_screen :false})
        .add_systems(Update,ui_system)
        .run();
}


fn ui_system(mut contexts: EguiContexts, mut krooz: ResMut<PROPERTIES>) {

    if !krooz.is_screen {
        egui::Window::new("Render Properties").show(contexts.ctx_mut(), |ui| {
        let render_btn = ui.button("render");

        if render_btn.clicked() 
        {
            krooz.is_screen = true;  
        } 
        });
    }
}

// rendering
fn render_system(
    // time: Res<Time>,
    // mut timer: ResMut<ScreenshotTimer>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut krooz: ResMut<PROPERTIES>,
    mut data: ResMut<Frames>, 
) {
    if krooz.is_screen {
        // timer.0.tick(time.delta());
        //  timer.0.finished() && 

        if data.ctr < data.max_frames {
            let filename = format!("./renders/Render_{}.png", data.ctr);
            
            screenshot_manager
                .save_screenshot_to_disk(main_window.single(), filename)
                .unwrap();
            data.ctr += 1.0;
        }

        if data.ctr >= data.max_frames{
        krooz.is_screen = false; 
        data.ctr = 0.0; 

        // let output = Command::new("python") // Use "python" directly
        //     .arg("./src/h34.py") // Pass the script as a single argument
        //     .output()
        //     .expect("failed to execute process");

        // if !output.status.success() {
        //     eprintln!("Error executing script: {}", String::from_utf8_lossy(&output.stderr));
        // }
    }
    }
}

// Camera
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
    AtmosphereCamera{
        ..Default::default()
    }
    ));
}

// Setting Window Parameters 
fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    input: Res<ButtonInput<KeyCode>>, 
) {
    let mut primary_window = q_windows.single_mut();
     primary_window.title = "Rusty-GLTF".to_string();
    if input.just_pressed(KeyCode::KeyG) {
        primary_window.cursor.grab_mode = match primary_window.cursor.grab_mode {
            CursorGrabMode::Locked => CursorGrabMode::None, 
            _ => CursorGrabMode::Locked, 
        };
        primary_window.cursor.visible = !primary_window.cursor.visible;
    }

    
    if input.just_pressed(KeyCode::Escape) {
        exit(0);
    }
}

// Additional Geometry Spawing
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

// Lights
fn spawn_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 2000.0, 
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-0.2)),
        ..default()
    });
}


