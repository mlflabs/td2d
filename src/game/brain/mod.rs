use bevy::{prelude::*, log::LogPlugin};
use big_brain::prelude::*;




pub mod thirst;
pub use thirst::*;

pub mod move_to_destination;
pub use move_to_destination::*;

pub mod wander;
pub use wander::*;

pub struct BrainPlugin;

impl Plugin for BrainPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Thinker>()
            .register_type::<Actor>()
            .register_type::<HasThinker>()
            //.register_type::<ActionSpan>()
            //.register_type::<>()
            //.register_type::<ActionSpan>()
            .register_type::<ActionState>()

            .add_plugins(BigBrainPlugin::new(PreUpdate))
            .add_systems(Update, thirst_system)
            .add_systems(
                PreUpdate,
                (
                    (   drink_action_system, 
                        move_to_water_source_action_system,
                        move_to_destination,
                        find_random_location_action_system)
                        .in_set(BigBrainSet::Actions),
                ),
            )
            .add_systems(First, (
                wander_scorer_system,
                thirsty_scorer_system
            ));
    }
}












