use bevy::{
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_web_asset::WebAssetPlugin;
use iyes_perf_ui::prelude::*;
use plugins::scene_viewer_plugin::{SceneHandle, SceneViewerPlugin};
use std::f32::consts::*;
use utils::parse_scene;

mod plugins;
mod utils;

fn main() {
    let mut app = App::new();

    /* #[cfg(debug_assertions)] // debug/dev builds only
    {
        use bevy::diagnostic::LogDiagnosticsPlugin;
        app.add_plugins(LogDiagnosticsPlugin::default());
    } */

    app.add_plugins((
        WebAssetPlugin,
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mower".to_string(),
                ..default()
            }),
            ..default()
        }),
        PanOrbitCameraPlugin,
        SceneViewerPlugin,
    ))
    .add_plugins((
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    ))
    .insert_resource(DirectionalLightShadowMap { size: 4096 })
    .add_plugins(PerfUiPlugin)
    .add_systems(Startup, setup)
    .add_systems(Update, animate_light_direction)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera controlls env map
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.7, 1.0, 1.0)
                .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 250.0,
        },
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 4.0, 2.0),
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    });

    let scene_path = "models/FlightHelmet/FlightHelmet.gltf";
    // let scene_path = "http://183.162.254.169:8074/public/uploads/manual/models/juese_c/model.gltf";
    let (file_path, scene_index) = parse_scene(scene_path.to_string());
    commands.insert_resource(SceneHandle::new(asset_server.load(file_path), scene_index));

    commands.spawn(PerfUiCompleteBundle::default());
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
