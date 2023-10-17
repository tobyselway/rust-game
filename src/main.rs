use bevy::utils::FloatOrd;
use bevy::{
    prelude::*,
    window::WindowResolution
};
use bevy::ecs::component::Component;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

#[derive(Resource)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>,
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet.glb#Scene0"),
    });
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    shooting_timer: Timer,
    bullet_offset: Vec3,
}

fn tower_shooting(
    mut commands: Commands,
    bullet_assets: Res<GameAssets>,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    time: Res<Time>,
) {
    for (tower_ent, mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            commands.entity(tower_ent).with_children(|commands| {
                let bullet_spawn = transform.translation() + tower.bullet_offset;
    
                let direction = targets.iter()
                    .min_by_key(|target_transform| {
                        FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                    })
                    .map(|closest_target| closest_target.translation() - bullet_spawn);

                if let Some(direction) = direction {
                    commands
                        .spawn(SceneBundle {
                            scene: bullet_assets.bullet_scene.clone(),
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..default()
                        })
                        .insert(Lifetime {
                            timer: Timer::from_seconds(10.0, TimerMode::Once),
                        })
                        .insert(Bullet {
                            direction,
                            speed: 2.0,
                        })
                        .insert(Name::new("Bullet"));
                }
            });
        }
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target {
    speed: f32,
}

fn move_targets(
    mut targets: Query<(&Target, &mut Transform)>,
    time: Res<Time>,
) {
    for (target, mut transform) in &mut targets {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    value: i32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet {
    direction: Vec3,
    speed: f32,
}

fn move_bullets(
    mut bullets: Query<(&Bullet, &mut Transform)>,
    time: Res<Time>
) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
    }
}

fn bullet_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
    mut targets: Query<(&mut Health, &Transform), With<Target>>,
) {
    for (bullet, bullet_transform) in &bullets {
        for (mut health, target_transform) in &mut targets {
            if Vec3::distance(bullet_transform.translation(), target_transform.translation) < 0.2 {
                commands.entity(bullet).despawn_recursive();
                health.value -= 1;
                break;
            }
        }
    }
}

fn target_death(mut commands: Commands, targets: Query<(Entity, &Health)>) {
    for (entity, health) in &targets {
        if health.value <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0, subdivisions: 0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
    .insert(Name::new("Ground"));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.67, 0.84, 0.3).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert(Tower {
        shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        bullet_offset: Vec3::new(0.0, 0.2, 0.5),
    })
    .insert(Name::new("Tower"));

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
        ))
        .register_type::<Tower>()
        .register_type::<Lifetime>()
        .register_type::<Target>()
        .register_type::<Bullet>()
        .register_type::<Health>()
        // Systems
        .add_systems(Startup, (
            asset_loading,
            spawn_basic_scene,
            spawn_camera,
        ))
        .add_systems(Update, (
            tower_shooting,
            bullet_despawn,
            move_targets,
            move_bullets,
            target_death,
            bullet_collision,
        ))
        .run();
}
