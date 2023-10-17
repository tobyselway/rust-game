use bevy::{prelude::*, utils::FloatOrd};
use bevy_mod_picking::prelude::PickSelection;

use crate::target::*;
use crate::bullet::*;
use crate::GameAssets;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Tower>()
            .add_systems(Update, (
                tower_shooting,
                build_tower,
            ));
    }
}

fn spawn_tomato_tower(
    commands: &mut Commands,
    assets: &GameAssets,
    position: Vec3,
) -> Entity {
    commands.spawn(SpatialBundle::from_transform(Transform::from_translation(position)))
        .insert(Name::new("Tomato_Tower"))
        .insert(Tower {
           shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
           bullet_offset: Vec3::new(0.0, 0.6, 0.0), 
        })
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: assets.tomato_tower_scene.clone(),
                transform: Transform::from_xyz(0.0, -0.8, 0.0),
                ..default()
            });
        })
        .id()
}

fn build_tower(
    mut commands: Commands,
    selection: Query<(Entity, &PickSelection, &Transform)>,
    keyboard: Res<Input<KeyCode>>,
    assets: Res<GameAssets>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for (entity, selection, transform) in &selection {
            if selection.is_selected {
                commands.entity(entity).despawn_recursive();
                spawn_tomato_tower(&mut commands, &assets, transform.translation);
            }
        }
    }
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
