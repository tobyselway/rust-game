use bevy::{
    prelude::*,
    window::WindowResolution,
    pbr::NotShadowCaster,
    math::vec4,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{
    *,
    prelude::{ Highlight, HighlightKind, RaycastPickTarget, DebugPickingPlugin },
};

use camera::*;
use target::*;
use bullet::*;
use tower::*;

mod camera;
mod target;
mod bullet;
mod tower;

#[derive(Resource)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>,
    tower_base_scene: Handle<Scene>,
    tomato_tower_scene: Handle<Scene>,
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet.glb#Scene0"),
        tower_base_scene: assets.load("TowerBase.glb#Scene0"),
        tomato_tower_scene: assets.load("TomatoTower.glb#Scene0"),
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<GameAssets>,
) {
    let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());
    const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
        hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
            base_color: matl.base_color + vec4(-0.2, -0.2, 0.4, 0.0),
            ..matl.to_owned()
        })),
        pressed: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
            base_color: matl.base_color + vec4(-0.3, -0.3, 0.5, 0.0),
            ..matl.to_owned()
        })),
        selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
            base_color: matl.base_color + vec4(-0.3, 0.2, -0.3, 0.0),
            ..matl.to_owned()
        })),
    };

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 25.0, subdivisions: 0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
    .insert(Name::new("Ground"));

    commands.spawn(SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.8, 0.0)))
        .insert(Name::new("Tower_Base"))
        .insert(meshes.add(shape::Capsule::default().into()))
        .insert(default_collider_color)
        .insert(HIGHLIGHT_TINT)
        .insert(NotShadowCaster)
        .insert(PickableBundle::default())
        .insert(RaycastPickTarget::default())
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: assets.tower_base_scene.clone(),
                transform: Transform::from_xyz(0.0, -0.8, 0.0),
                ..default()
            });
        });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    })
    .insert(Name::new("Light"));

    for i in 0..5 {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-6.0 + (i as f32), 0.2, 1.5),
            ..default()
        })
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Porridge".to_string(),                        
                        resolution: WindowResolution::new(600.0, 400.0)
                            .with_scale_factor_override(1.0),
                        ..default()
                    }),
                    ..default()
                }),
            WorldInspectorPlugin::new(),
            DefaultPickingPlugins
                .build()
                .disable::<DebugPickingPlugin>(),
            CameraPlugin,
            BulletPlugin,
            TargetPlugin,
            TowerPlugin,
        ))
        // Systems
        .add_systems(PreStartup, (
            asset_loading,
        ))
        .add_systems(Startup, (
            spawn_basic_scene,
        ))
        .run();
}
