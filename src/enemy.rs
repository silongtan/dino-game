use bevy::prelude::*;
use crate::{GameTextures, SPRITE_SCALE, WinSize, ENEMY_SIZE, BASE_SPEED, TIME_STEP, EnemyCount, ENEMY_MAX, ENEMY_LASER_SIZE};
use crate::components::{Velocity, SpriteSize, Player, Movable, Laser, Enemy, FromPlayer, FromEnemy};
use rand::prelude::*;
use bevy::core::FixedTimestep;
use bevy::ecs::schedule::ShouldRun;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system);
            // .add_system(enemy_spawn_system);
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(2.0))
                    .with_system(enemy_spawn_system),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(enemy_fire_criteria)
                    .with_system(enemy_fire_system),
            );
    }
}

fn enemy_spawn_system(
    mut commands: Commands, 
    game_textures: Res<GameTextures>, 
    mut enemy_count: ResMut<EnemyCount>,
    win_size: Res<WinSize>,
) {
    if enemy_count.0 < ENEMY_MAX {
        // compute x, y
        let mut rng = thread_rng();
        let w_span = win_size.w / 2.0 - 100.0;
        let h_span = win_size.h / 2.0 - 100.0;
        let x = rng.gen_range(-w_span..w_span);
        let y = rng.gen_range(-h_span..h_span);
        
        commands.spawn_bundle(SpriteBundle {
            texture: game_textures.enemy.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 10.0),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy)
        .insert(SpriteSize::from((ENEMY_SIZE)));

        enemy_count.0 += 1;
    }
}

fn enemy_fire_criteria() -> ShouldRun {
    if thread_rng().gen_bool(1.0 / 60.0) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for &transform in enemy_query.iter() {
        // spawn enemy laser
        let (x, y) = (transform.translation.x, transform.translation.y);
        commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.enemy_laser.clone(),
            transform: Transform {
                translation: Vec3::new(x, y-15.0, 0.0),
                rotation: Quat::from_rotation_x(std::f32::consts::PI),
                scale: Vec3::new(SPRITE_SCALE * 0.1, SPRITE_SCALE * 0.1, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Laser)
        .insert(SpriteSize::from((ENEMY_LASER_SIZE)))
        .insert(FromEnemy)
        .insert(Movable {auto_despawn: true})
        .insert(Velocity {x: 0.0, y: -0.4});
    }
}