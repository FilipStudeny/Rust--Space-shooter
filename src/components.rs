use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Velocity{
    pub x: f32,
    pub y: f32,
}


#[derive(Component)]
pub struct MovableObject{
    pub auto_despawn: bool,
}

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize{
    fn from(value: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(value.0, value.1))
    }
}

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct ScoreText;
pub struct Score(pub u32);

#[derive(Component)]
pub struct HealthText;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

//PLAYER
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct ComingFromPlayer;

pub struct PlayerState{
    pub is_alive: bool,
    pub last_shot: f64,
}

impl Default for PlayerState{
    fn default() -> Self {
        Self { is_alive: false, last_shot: -1. }
    }
}

impl PlayerState{
    pub fn player_is_shot(&mut self, time: f64){
        self.is_alive = false;
        self.last_shot = time;
    }

    pub fn spawned(&mut self){
        self.is_alive = true;
        self.last_shot = -1.;
    }

    
}




//ENEMY
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct ComingFromEnemy;

#[derive(Component)]
pub struct EnemySpawnPosition(pub (f32, f32));

pub struct EnemyCount(pub u32);


pub struct GameTextures{
   pub player: Handle<TextureAtlas>,
   pub enemy: Handle<TextureAtlas>,
   pub explosion: Handle<TextureAtlas>,
}


//EXPLOSION
#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3, pub f32);

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
	fn default() -> Self {
		Self(Timer::from_seconds(0.05, true))
	}
}