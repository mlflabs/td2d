use bevy::prelude::*;
use big_brain::{thinker::Thinker, pickers::FirstToScore, actions::Steps};

use super::{
        Thirst, 
        Thirsty, 
        Drink, 
        MAP_MID_Z,
        MoveToWaterSource, 
        brain::Position, 
        Wander, 
        Speed, 
        MoveToDestination, 
        FindRandomLocationAction, 
        MoveAction};



#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
/// Demo marker component
pub struct NpcComponent;



pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        
        app
            //.register_type::<()
            .register_type::<Position>()
            .add_systems(Startup,(
                spawn_npc,
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


pub fn spawn_npc(
    mut commands: Commands,
) {

    let move_and_drink = Steps::build()
        .label("MoveAndDrink")
        // ...move to the water source...
        .step(MoveToWaterSource { speed: 50.0 })
        // ...and then drink.
        .step(Drink { per_second: 10.0 });

    let wander = Steps::build()
        .label("Wander")
        .step(FindRandomLocationAction)
        .step(MoveAction);


    let thinker = Thinker::build()
        .label("ThirstyThinker")
        // We don't do anything unless we're thirsty enough.
        .picker(FirstToScore { threshold: 0.5 })
        //.when(Wander, )
        .when(Thirsty, move_and_drink)
        .when(Wander, wander);
        

    commands.spawn((
        NpcComponent,
        Name::new("Npc"),
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, MAP_MID_Z),
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            ..default()
        },
        Thirst::new(75.0, 2.0),
        Speed::new(50.),
        MoveToDestination::new(Vec2::ZERO, 10.),
        Position {
            position: Vec2::new(0.0, 0.0),
        },
        thinker,
        
        //RigidBody::KinematicPositionBased,
        //Collider::ball(8.)
    ));
 
}



