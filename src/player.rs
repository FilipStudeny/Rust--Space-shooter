use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::utils::HashSet;
use crate::*;


const PLAYER_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const PLAYER_SCALE: f32 = 1.;

const PLAYER_BULLET_SIZE: (f32, f32) = (10., 10.);
const PLAYER_SIZE: (f32, f32) = (32., 32.);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
        .insert_resource(PlayerState::default())
        .insert_resource(Score(0))
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.5))
            .with_system(spawn_player)
        )
        .add_system(animate_object)
        .add_system(player_input_event)
        .add_system(player_movement)
        .add_system(player_shooting)
        .add_system(player_bullet_movement)
        .add_system(player_bullet_collision)
        .add_system(player_to_enemy_collision)
        .add_system(player_bullet_to_enemy_bullet_collision);

    }
}

const PLAYER_BOTTOM_POSITION: f32 = (-WINDOW_HEIGHT/2.) + 100.;

fn spawn_player(mut commands: Commands, texture: Res<GameTextures>, mut player_state: ResMut<PlayerState>, time: Res<Time>){


    let time_now = time.seconds_since_startup();
    let last_shot = player_state.last_shot;

    if !player_state.is_alive && (last_shot == -1. || time_now > last_shot + PLAYER_RESPAWN_DELAY){
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture.player.clone(),
            /* 
            sprite: Sprite{
                color: PLAYER_COLOR,
                custom_size: Some(Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1)),
                ..default()
                
            },*/
            transform: Transform{
                translation: Vec3::new(0.0, PLAYER_BOTTOM_POSITION, 1.),
                scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(SpriteSize::from(PLAYER_SIZE))
        .insert(Velocity {x: 0. , y: 0.})
        .insert(Health(5f32)) //COMMON COMPONENT
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

        player_state.spawned();
    }

    
}



fn player_input_event(input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>){
    if let Ok(mut velocity) = query.get_single_mut(){
        velocity.x = if input.pressed(KeyCode::Left){
            -1.
        } else if input.pressed(KeyCode::Right) {
            1.
        }else{
            0.
        }

    }
}

fn player_movement(mut query: Query<(&Velocity, &mut Transform), With<Player>>){

    for (velocity, mut transform) in query.iter_mut(){

        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * GAME_SPEED;
        translation.x = translation.x.min((WINDOW_WIDTH/2.) - PLAYER_SIZE.0).max((-WINDOW_WIDTH/2.) + PLAYER_SIZE.0);
    }
}

fn player_shooting(mut commands: Commands, input: Res<Input<KeyCode>>, query: Query<&Transform, With<Player>>){

    if let Ok(transform) = query.get_single(){  //SIGNLE QUERY, NOT MUTABLE ONLY READ
        let player_position_x = transform.translation.x;
        let player_position_y = transform.translation.y;

        if input.just_pressed(KeyCode::Space){
            commands.spawn_bundle(SpriteBundle{
                sprite: Sprite{
                    color: PLAYER_COLOR,
                    custom_size: Some(Vec2::new(PLAYER_BULLET_SIZE.0, PLAYER_BULLET_SIZE.1)),
                    ..default()
                },
                transform: Transform{
                    translation: Vec3::new(player_position_x, player_position_y + 50., 1.0),
                    scale: Vec3::new(1., 1., 1.),
                    ..default()
                },
                ..default()
            })
            .insert(ComingFromPlayer)
            .insert(SpriteSize::from(PLAYER_BULLET_SIZE))
            .insert(Bullet)
            .insert(MovableObject {auto_despawn: true})
            .insert(Velocity {x: 0. , y: 1.}); //COMMON COMPONENT

        }
    }

}

fn player_bullet_movement(mut commands: Commands, mut query: Query<(Entity,&Velocity, &mut Transform), With<Bullet>>){
    let mut despawned_entitites: HashSet<Entity> = HashSet::new();

    for(bullet_entity, velocity ,mut bullet_transform) in query.iter_mut(){
        if despawned_entitites.contains(&bullet_entity){
            continue;
        }
        
        let bullet_position = &mut bullet_transform.translation;
        bullet_position.y += velocity.y * TIME_STEP * GAME_SPEED;

        if bullet_position.y > (WINDOW_HEIGHT - 400.) {
            commands.entity(bullet_entity).despawn();
            despawned_entitites.insert(bullet_entity);
        }
    }
}

fn player_to_enemy_collision(mut commands: Commands, mut score: ResMut<Score>, mut player_state: ResMut<PlayerState>, time: Res<Time>,mut player_query: Query<(Entity, &Transform, &SpriteSize, &mut Health),With<Player>>, enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>){

    for(player_entity, player_transform, player_size,mut player_health) in player_query.iter_mut(){

        let player_scales: (f32, f32) = (player_transform.scale.x, player_transform.scale.y);
        let player_scale = Vec2::new(player_scales.0, player_scales.1);

        for (enemy_entity, enemy_transform, enemy_size) in enemy_query.iter(){
            
            let enemy_scales: (f32, f32) = (enemy_transform.scale.x, enemy_transform.scale.y);
            let enemy_scale = Vec2::new(enemy_scales.0, enemy_scales.1);

            let collision = collide(
                player_transform.translation, player_size.0 * player_scale, 
                enemy_transform.translation, enemy_size.0 * enemy_scale
            );

            //colision logic
            if let Some(_) = collision{

                commands.entity(enemy_entity).despawn();
                commands.spawn().insert(ExplosionToSpawn(enemy_transform.translation.clone(), 1.5f32));

                player_health.0 -= 1.;
                if player_health.0 == 0.{

                    player_health.0 = 0.;
                    commands.spawn().insert(ExplosionToSpawn(player_transform.translation.clone(), 1.5f32));
                    commands.entity(player_entity).despawn(); 

                    score.0 = 0;
                    player_state.player_is_shot(time.seconds_since_startup());
                }  
            }
        }

    }
}

fn player_bullet_to_enemy_bullet_collision(mut commands: Commands, 
    player_bullet_query: Query<(Entity, &Transform, &SpriteSize),(With<Bullet>, With<ComingFromPlayer>)>, 
    enemy_bullet_query: Query<(Entity, &Transform, &SpriteSize), (With<Bullet>, With<ComingFromEnemy>)>){

    let mut despawned_entitites: HashSet<Entity> = HashSet::new();

    for (player_bullet_entity, player_bullet_transform, player_bullet_size) in player_bullet_query.iter(){
        
        if despawned_entitites.contains(&player_bullet_entity){
            continue;
        }

        let player_scales: (f32, f32) = (player_bullet_transform.scale.x, player_bullet_transform.scale.y);
        let player_scale = Vec2::new(player_scales.0, player_scales.1);

        for (enemy_bullet_entity, enemy_bullet_transform, enemy_bullet_size) in enemy_bullet_query.iter(){
            
            if despawned_entitites.contains(&enemy_bullet_entity) || despawned_entitites.contains(&enemy_bullet_entity){
                continue;
            }

            let enemy_scales: (f32, f32) = (enemy_bullet_transform.scale.x, enemy_bullet_transform.scale.y);
            let enemy_scale = Vec2::new(enemy_scales.0, enemy_scales.1);

            let collision = collide(
                player_bullet_transform.translation, player_bullet_size.0 * player_scale, 
                enemy_bullet_transform.translation, enemy_bullet_size.0 * enemy_scale
            );

            //colision logic
            if let Some(_) = collision{
                commands.spawn().insert(ExplosionToSpawn(enemy_bullet_transform.translation.clone(), 0.5f32));

                commands.entity(player_bullet_entity).despawn();
                commands.entity(enemy_bullet_entity).despawn();

                despawned_entitites.insert(player_bullet_entity);
                despawned_entitites.insert(enemy_bullet_entity);

            }
        }

    }    

  }

fn player_bullet_collision(mut commands: Commands, mut score: ResMut<Score>, mut enemy_count: ResMut<EnemyCount>, player_query: Query<(Entity, &Transform, &SpriteSize), (With<Bullet>, With<ComingFromPlayer>)>,mut enemy_query: Query<(Entity, &Transform, &SpriteSize,&mut Health), With<Enemy>>){

    let mut despawned_entitites: HashSet<Entity> = HashSet::new();
    //iterate through bullets
    for (bullet_entity, bullet_transform, bullet_size) in player_query.iter(){

        if despawned_entitites.contains(&bullet_entity){
            continue;
        }
        let bullet_scales: (f32, f32) = (bullet_transform.scale.x, bullet_transform.scale.y);
        let bullet_scale = Vec2::new(bullet_scales.0, bullet_scales.1);
        //iterate thgourh enemies
        for(enemy_entity, enemy_transform, enemy_size,mut enemy_health) in enemy_query.iter_mut(){

            if despawned_entitites.contains(&enemy_entity) ||despawned_entitites.contains(&bullet_entity){
                continue;
            }

            let enemy_scales: (f32, f32) = (enemy_transform.scale.x, enemy_transform.scale.y);
            let enemy_scale = Vec2::new(enemy_scales.0, enemy_scales.1);

            //determine colison

            let collision = collide(
                bullet_transform.translation, bullet_size.0 * bullet_scale, 
                enemy_transform.translation, enemy_size.0 * enemy_scale);

            //colision logic
            if let Some(_) = collision{

                enemy_health.0 -= 1.;
                if enemy_health.0 == 0.{

                    commands.spawn().insert(ExplosionToSpawn(enemy_transform.translation.clone(), 1.5f32));

                    commands.entity(enemy_entity).despawn();
                    despawned_entitites.insert(enemy_entity);
                    enemy_count.0 -= 1;
                    score.0 += 1;

                    break;

                }

                commands.spawn().insert(ExplosionToSpawn(bullet_transform.translation.clone(), 0.5f32));

                commands.entity(bullet_entity).despawn();
                despawned_entitites.insert(bullet_entity);

                break;
            }
        }
    }
}
