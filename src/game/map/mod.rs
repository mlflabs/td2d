use bevy::prelude::*;
use big_brain::{thinker::Thinker, pickers::FirstToScore};

use super::{Thirst, Thirsty, Drink, MAP_LOW_Z, brain::Position};



#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
/// Demo marker component
pub struct WaterSourceComponent;



pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<WaterSourceComponent>()

            .add_systems(Startup,(
                spawn_water,
            ));
            // .add_plugins(DefaultPlugins.set(LogPlugin {
            //     // Use `RUST_LOG=big_brain=trace,thirst=trace cargo run --example
            //     // thirst --features=trace` to see extra tracing output.
            //     filter: "big_brain=debug,thirst=debug".to_string(),
            //     ..default()
            // }))
            //.add_plugins()
            // .add_systems(
            //     PreUpdate,
            //     (
            //         drink_action_system.in_set(BigBrainSet::Actions),
            //         thirsty_scorer_system.in_set(BigBrainSet::Scorers),
            //     ),
            // );
    }
}


pub fn spawn_water(
    mut commands: Commands,
) {

    commands.spawn((
        WaterSourceComponent,
        Name::new("WaterSource"),
        SpriteBundle {
            transform: Transform::from_xyz(4000.0, 2000.0, MAP_LOW_Z),
            sprite: Sprite {
                color: Color::rgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            ..default()
        },
        Position {
            position: Vec2::new(4000.0, 2000.0),
        },
    ));

    commands.spawn((
        WaterSourceComponent,
        Name::new("WaterSource"),
        SpriteBundle {
            transform: Transform::from_xyz(100.0, 100.0, MAP_LOW_Z),
            sprite: Sprite {
                color: Color::rgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            ..default()
        },
        Position {
            position: Vec2::new(100.0, 100.0),
        },
    ));
 


    commands.spawn((
        WaterSourceComponent,
        Name::new("WaterSource"),
        SpriteBundle {
            transform: Transform::from_xyz(1000.0, 2500.0, MAP_LOW_Z),
            sprite: Sprite {
                color: Color::rgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            ..default()
        },
        Position {
            position: Vec2::new(1000.0, 2500.0),
        },
    ));
 
 
}



