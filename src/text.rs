use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, Diagnostics}};
use crate::*;


pub struct TextPlugin;

impl Plugin for TextPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(spawn_fps_text)
        .add_system(update_fps_text)
        .add_system(update_score_text)
        .add_system(update_health_text);

    }
}


const HEALTH_ICON_POSITION: f32 = (-WINDOW_HEIGHT/2.) + 100.;

fn spawn_fps_text(mut commands: Commands, asset_server: Res<AssetServer>){
    commands
    .spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        },

        text:Text{
            sections: vec![
                TextSection{
                    value: "Score: ".to_string(),
                    style: TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font_size: 20.0,
                        color: Color::GOLD,
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    },
                },
            ],
            ..default()
        },
        ..default()
    })
    .insert(ScoreText);

    commands.spawn_bundle(TextBundle{
        style: Style{
            align_self: AlignSelf::FlexEnd,
            ..default()
        },

        text: Text{
            sections: vec![
                TextSection{
                    value: "FPS: ".to_string(),
                    style: TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font_size: 20.0,
                        color: Color::GOLD,
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    },
                },
            ],
            ..default()
        },
        ..default()
    })
    .insert(FpsText);

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/player/player_health_icon.png"),
        transform: Transform{
            translation: Vec3::new(-35., HEALTH_ICON_POSITION - 75. , 5.),
            scale: Vec3::new(0.5, 0.5, 0.5),
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(15.0),
                right: Val::Px(250.0),
                ..default()
            },
            ..default()
        },

        text:Text{
            sections: vec![
                TextSection{
                    value: "HEALTH: ".to_string(),
                    style: TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font_size: 20.0,
                        color: Color::GOLD,
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    },
                },
            ],
            ..default()
        },
        ..default()
    })
    .insert(HealthText);

}

fn update_fps_text(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>){
    for mut text in query.iter_mut(){
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS){
            if let Some(average) = fps.average(){
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}

fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>){
    for mut text in query.iter_mut(){
        text.sections[1].value = format!("{:.2}", score.0);
    }
}

fn update_health_text( player_query: Query<&Health, With<Player>>,mut text_query: Query<&mut Text, With<HealthText>>){

    for health in player_query.iter(){
        for mut text in text_query.iter_mut(){

            if health.0 == 0.{
                text.sections[1].value = format!("{:.2}", 0);
            }else{
                text.sections[1].value = format!("{:.2}", health.0 as u32);
            }
        }
    }

    

}
