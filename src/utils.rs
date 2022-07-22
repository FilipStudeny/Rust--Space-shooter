
use crate::*;

pub fn animate_object(time: Res<Time>, texture_atlases: Res<Assets<TextureAtlas>>,mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>,)>){

    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

pub fn animate_explosion(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>){

        for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1; // move to next sprite cell
            if sprite.index >= 16 {
                commands.entity(entity).despawn()
            }
        }
    }
}

pub fn explosion_spawn(mut commands: Commands, query: Query<(Entity, &ExplosionToSpawn)>, texture: Res<GameTextures>){

    for (explosion_entity, explosion_to_spawn) in query.iter(){

        commands.spawn_bundle(SpriteSheetBundle{
            texture_atlas: texture.explosion.clone(),
            transform: Transform{
                translation: explosion_to_spawn.0,
                scale: Vec3::new(explosion_to_spawn.1, explosion_to_spawn.1, 1.),
                ..default()
            },
            ..default()
        })
        .insert(Explosion)
        .insert(ExplosionTimer::default())
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

        commands.entity(explosion_entity).despawn();
    }
}

