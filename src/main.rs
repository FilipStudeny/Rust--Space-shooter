use bevy::prelude::*;
use player::*;
use enemy::*;
use components::*;
use text::*;
use utils::*;

mod player;
mod enemy;
mod components;
mod text;
mod utils;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const WINDOW_HEIGHT: f32 = 800.;
const WINDOW_WIDTH: f32 = 600.;

const TIME_STEP: f32 = 1. / 60.;
const GAME_SPEED: f32 = 500.;

const MAXIMUM_NUM_OF_ENEMIES_IN_ARENA: u32 = 15u32;
const PLAYER_RESPAWN_DELAY: f64 = 2.;

fn main() {

    App::new()
    .insert_resource(ClearColor(BACKGROUND_COLOR))
    .insert_resource(WindowDescriptor{
        title: "Spacey invader".to_string(),
        width: WINDOW_WIDTH, height: WINDOW_HEIGHT,
        ..default()
    })
    .insert_resource(EnemyCount(0))

    .add_startup_system(create_2d_camera)
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(TextPlugin)
    .add_system(explosion_spawn)
    .add_system(animate_explosion)
    .add_system(bevy::input::system::exit_on_esc_system)
    .run();
}

fn create_2d_camera(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let player_texture_handler: Handle<Image> = asset_server.load("images/player/player.png");
    let texture_atlas = TextureAtlas::from_grid(player_texture_handler,Vec2::new(64., 64.), 1, 4);
    let player_sprites = texture_atlases.add(texture_atlas);

    let enemy_texture_handler: Handle<Image> = asset_server.load("images/enemy/enemy.png");
    let texture_atlas = TextureAtlas::from_grid(enemy_texture_handler,Vec2::new(64., 64.), 1, 4);
    let enemy_sprites = texture_atlases.add(texture_atlas);

    let explosion_texture_handler: Handle<Image> = asset_server.load("images/explosion.png");
    let texture_atlas = TextureAtlas::from_grid(explosion_texture_handler,Vec2::new(64., 64.), 1, 16);
    let explosion_sprites = texture_atlases.add(texture_atlas);


    let textures = GameTextures{
        player: player_sprites,
        enemy: enemy_sprites,
        explosion: explosion_sprites
    };

    commands.insert_resource(textures);
}


