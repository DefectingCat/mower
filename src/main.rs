use bevy::{asset::AssetMetaCheck, pbr::DirectionalLightShadowMap, prelude::*};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use std::f32::consts::*;

mod plugins;
mod utils;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics on web build on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Mower".to_string(),
                    canvas: Some("#bevy".to_string()),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
        PanOrbitCameraPlugin,
    ))
    .insert_resource(DirectionalLightShadowMap { size: 4096 })
    .add_systems(Startup, setup)
    .add_systems(Update, animate_light_direction)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 4.0, 2.0),
        ..default()
    });

    let scene_path = "models/FlightHelmet/FlightHelmet.gltf#Scene0";
    let model = asset_server.load(GltfAssetLabel::Scene(0).from_asset(scene_path));
    commands.spawn(SceneBundle {
        scene: model,
        ..default()
    });

    // Camera controlls env map
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 1.5),
            ..default()
        },
        PanOrbitCamera {
            focus: Vec3::new(0.0, 0.2, 0.0),
            ..default()
        },
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 250.0,
        },
    ));
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}
