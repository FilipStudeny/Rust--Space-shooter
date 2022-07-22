
use bevy::utils::HashSet;
use bevy::{prelude::*, core::FixedTimestep, ecs::schedule::ShouldRun, sprite::collide_aabb::collide};
use rand::{thread_rng, Rng};
use crate::*;


const ENEMY_SIZE: (f32, f32) = (64., 64.);
const ENEMY_SCALE: f32 = 1.;

const ENEMY_BULLET_SIZE: (f32, f32) = (10., 10.);
const ENEMY_BULLET_COLOR: Color = Color::rgb(1.0, 0.5, 0.58);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.5))
            .with_system(spawn_enemy)
        )
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(enemy_fire_criteria)
            .with_system(enemy_fire)
        )
        .add_system(enemy_movement)
        .add_system(enemy_bullet_despawn)
        .add_system(enemy_bullet_colision);
    }
}

pub fn spawn_enemy(commands: Commands, texture: Res<GameTextures>, enemy_count: ResMut<EnemyCount>){

    if enemy_count.0 < MAXIMUM_NUM_OF_ENEMIES_IN_ARENA{

        let mut random_generator = thread_rng();
        let position_x: f32 = random_generator.gen_range(((-WINDOW_WIDTH/2f32) + 20f32)..((WINDOW_WIDTH/2f32) - 20f32));
        let position_y: f32 = WINDOW_HEIGHT;

        spawn_enemy_entity(commands, texture, enemy_count, position_x, position_y);
    }

}

fn spawn_enemy_entity(mut commands: Commands, texture: Res<GameTextures>, mut enemy_count: ResMut<EnemyCount>, position_x: f32, position_y: f32){


    commands.spawn_bundle(SpriteSheetBundle{
        texture_atlas: texture.enemy.clone(),
        transform: Transform{
            translation: Vec3::new(position_x, position_y, 10.),
            scale: Vec3::new(ENEMY_SCALE, ENEMY_SCALE, 1.),
            ..default()
        },
        ..default()
    })
    .insert(Enemy)
    .insert(SpriteSize::from(ENEMY_SIZE))
    .insert(Health(2.))
    .insert(Velocity {x: 0., y: -0.3})
    .insert(EnemySpawnPosition((position_x, position_y)))
    .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

    enemy_count.0 += 1;
}

fn enemy_movement(mut commands: Commands, mut enemy_count: ResMut<EnemyCount>, mut query: Query<(Entity, &Velocity, &mut Transform), With<Enemy>>){
    
    for(_enemy_entity, velocity ,mut enemy_transform) in query.iter_mut(){
        let enemy_position = &mut enemy_transform.translation;

        enemy_position.y += velocity.y * TIME_STEP * GAME_SPEED;

        if  enemy_position.y < -WINDOW_HEIGHT {
            commands.entity(_enemy_entity).despawn();
            enemy_count.0 -= 1;
        }

    }
}

fn enemy_fire_criteria() -> ShouldRun{
    if thread_rng().gen_bool(1. / 100.){
        ShouldRun::Yes
    }else{
        ShouldRun::No
    }
}

fn enemy_fire(mut commands: Commands, query: Query<&Transform, With<Enemy>>){

    for &transform in query.iter(){

        let enemy_postion: (f32, f32) = (transform.translation.x, transform.translation.y);

        //spawn bullet
        commands.spawn_bundle(SpriteBundle{
            sprite: Sprite{
                color: ENEMY_BULLET_COLOR,
                custom_size: Some(Vec2::new(ENEMY_BULLET_SIZE.0, ENEMY_BULLET_SIZE.1)),
                ..default()
            },
            transform: Transform{
                translation: Vec3::new(enemy_postion.0, enemy_postion.1 - 25., 1.0),
                scale: Vec3::new(1., 1., 1.),
                ..default()
            },
            ..default()
        })
        .insert(ComingFromEnemy)
        .insert(SpriteSize::from(ENEMY_BULLET_SIZE))
        .insert(Bullet)
        .insert(MovableObject {auto_despawn: true})
        .insert(Velocity {x: 0. , y: -1.}); //COMMON COMPONENT

    }
}

fn enemy_bullet_despawn(mut commands: Commands, query: Query<(Entity, &Transform), (With<Bullet>, With<ComingFromEnemy>)>)
{
    let mut despawned_entitites: HashSet<Entity> = HashSet::new();

    for(bullet_entity, bullet_transform) in query.iter(){
        if despawned_entitites.contains(&bullet_entity){
            continue;
        }
        
        let bullet_position = &bullet_transform.translation;

        if bullet_position.y < (-WINDOW_HEIGHT/2.) {
           commands.entity(bullet_entity).despawn();
           despawned_entitites.insert(bullet_entity);
        }
    }
}
fn enemy_bullet_colision(mut commands: Commands, 
    mut player_state: ResMut<PlayerState>,
    mut score: ResMut<Score>,
    time: Res<Time>,
    mut player_query: Query<(Entity, &Transform, &SpriteSize, &mut Health), With<Player>>, 
    bullet_query: Query<(Entity, &Transform, &SpriteSize), (With<Bullet>, With<ComingFromEnemy>)>){

    for(player_entity, player_transform, player_size, mut player_health) in player_query.iter_mut(){

        let player_scales: (f32, f32) = (player_transform.scale.x, player_transform.scale.y);
        let player_scale = Vec2::new(player_scales.0, player_scales.1);    

        for (bullet_entity, bullet_transform, bullet_size) in bullet_query.iter(){

            let bullet_scales: (f32, f32) = (bullet_transform.scale.x, bullet_transform.scale.y);
            let bullet_scale = Vec2::new(bullet_scales.0, bullet_scales.1);

            //determine colisions
            let collision = collide(
                bullet_transform.translation, bullet_size.0 * bullet_scale, 
                player_transform.translation, player_size.0 * player_scale);

            //colision logic

            if let Some(_) = collision{

                player_health.0 -= 1.;
                if player_health.0 == 0.{


                    commands.spawn().insert(ExplosionToSpawn(player_transform.translation.clone(), 1.5f32));
                    commands.entity(player_entity).despawn(); 

                    score.0 = 0;
                    player_state.player_is_shot(time.seconds_since_startup());
                }               
            
                commands.spawn().insert(ExplosionToSpawn(bullet_transform.translation.clone(), 0.5f32));
                commands.entity(bullet_entity).despawn();
            }
        }
    }

}
