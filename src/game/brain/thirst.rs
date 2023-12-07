use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::utils::tracing::{debug, trace};
use big_brain::prelude::*;

use crate::game::{WaterSourceComponent, MAP_MID_Z};


/// First, we make a simple Position component.
#[derive(Component, Debug, Copy, Clone, Reflect)]
pub struct Position {
    pub position: Vec2,
}


#[derive(Component, Reflect, Debug)]
pub struct Thirst {
    pub per_second: f32,
    pub thirst: f32,
}

impl Thirst {
    pub fn new(thirst: f32, per_second: f32) -> Self {
        Self { thirst, per_second }
    }
}


pub fn thirst_system(time: Res<Time>, mut thirsts: Query<&mut Thirst>) {
    for mut thirst in &mut thirsts {
        thirst.thirst += thirst.per_second * (time.delta().as_micros() as f32 / 1_000_000.0);
        if thirst.thirst >= 100.0 {
            thirst.thirst = 100.0;
        }
        //println!("Thirst: {}", thirst.thirst);
    }
}


#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Drink {
    pub per_second: f32,
}


pub fn drink_action_system(
    time: Res<Time>,
    mut thirsts: Query<(&Position, &mut Thirst), Without<WaterSourceComponent>>,
    waters: Query<&Position, With<WaterSourceComponent>>,
    mut query: Query<(&Actor, &mut ActionState, &Drink, &ActionSpan)>,
) {
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (Actor(actor), mut state, drink, span) in &mut query {
        let _guard = span.span().enter();

        // Look up the actor's position and thirst from the Actor component in the action entity.
        let (actor_position, mut thirst) = thirsts.get_mut(*actor).expect("actor has no thirst");

        match *state {
            ActionState::Requested => {
                // We'll start drinking as soon as we're requested to do so.
                debug!("Drinking the water.");
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Look up the closest water source.
                // Note that there is no explicit passing of a selected water source from the GoToWaterSource action,
                // so we look it up again. Note that this decouples the actions from each other,
                // so if the actor is already close to a water source, the GoToWaterSource action
                // will not be necessary (though it will not harm either).
                //
                // Essentially, being close to a water source would be a precondition for the Drink action.
                // How this precondition was fulfilled is not this code's concern.
                let closest_water_source = find_closest_water_source(&waters, actor_position);

                // Find how far we are from it.
                let distance = (closest_water_source.position - actor_position.position).length();

                // Are we close enough?
                if distance < MAX_DISTANCE {
                    //println!("Drinking!");

                    // Start reducing the thirst. Alternatively, you could send out some kind of
                    // DrinkFromSource event that indirectly decreases thirst.
                    thirst.thirst -= drink.per_second * time.delta_seconds();

                    // Once we hit 0 thirst, we stop drinking and report success.
                    if thirst.thirst <= 0.0 {
                        thirst.thirst = 0.0;
                        *state = ActionState::Success;
                    }
                } else {
                    // The actor was told to drink, but they can't drink when they're so far away!
                    // The action doesn't know how to deal with this case, it's the overarching system's
                    // to fulfill the precondition.
                    //debug!("We're too far away!");
                    *state = ActionState::Failure;
                }
            }
            // All Actions should make sure to handle cancellations!
            // Drinking is not a complicated action, so we can just interrupt it immediately.
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Thirsty;


pub fn thirsty_scorer_system(
    thirsts: Query<&Thirst>,
    // Same dance with the Actor here, but now we use look up Score instead of ActionState.
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<Thirsty>>,
) {
    for (Actor(actor), mut score, span) in &mut query {
        if let Ok(thirst) = thirsts.get(*actor) {
            // This is really what the job of a Scorer is. To calculate a
            // generic "Utility" score that the Big Brain engine will compare
            // against others, over time, and use to make decisions. This is
            // generally "the higher the better", and "first across the finish
            // line", but that's all configurable using Pickers!
            //
            // The score here must be between 0.0 and 1.0.
            score.set(thirst.thirst / 100.0 - 0.01);
            println!("Thirst Score: {:?}", score);
            if thirst.thirst >= 80.0 {
                span.span().in_scope(|| {
                    debug!("Thirst above threshold! Score: {}", thirst.thirst / 100.0)
                });
            }
        }
    }
}








/// An action where the actor moves to the closest water source
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MoveToWaterSource {
    // The movement speed of the actor.
    pub speed: f32,
}

/// Closest distance to a water source to be able to drink from it.
const MAX_DISTANCE: f32 = 0.1;

pub fn move_to_water_source_action_system(
    time: Res<Time>,
    // Find all water sources
    waters: Query<&Position, With<WaterSourceComponent>>,
    // We use Without to make disjoint queries.
    mut positions: Query<(&mut Position, &mut Transform), Without<WaterSourceComponent>>,
    // A query on all current MoveToWaterSource actions.
    mut action_query: Query<(&Actor, &mut ActionState, &MoveToWaterSource, &ActionSpan)>,
) {
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (actor, mut action_state, move_to, span) in &mut action_query {
        let _guard = span.span().enter();

        // Different behavior depending on action state.
        match *action_state {
            // Action was just requested; it hasn't been seen before.
            ActionState::Requested => {
                println!("Let's go find some water!");
                // We don't really need any initialization code here, since the queries are cheap enough.
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Look up the actor's position.
                let (mut actor_position, mut transform)
                    = positions.get_mut(actor.0).expect("actor has no position");

                //println!("Actor position: {:?}", actor_position.position);

                // Look up the water source closest to them.
                let closest_water_source = find_closest_water_source(&waters, &actor_position);

                // Find how far we are from it.
                let delta = closest_water_source.position - actor_position.position;

                let distance = delta.length();

                //println!("Distance: {}", distance);

                if distance > MAX_DISTANCE {
                    // We're still too far, take a step toward it!

                    //println!("Stepping closer.");

                    // How far can we travel during this frame?
                    let step_size = time.delta_seconds() * move_to.speed;
                    // Travel towards the water-source position, but make sure to not overstep it.
                    let step = delta.normalize() * step_size.min(distance);

                    // Move the actor.
                    actor_position.position += step;
                    transform.translation = Vec3::from((actor_position.position, MAP_MID_Z));
                } else {
                    // We're within the required distance! We can declare success.

                    //println!("We got there!");

                    // The action will be cleaned up automatically.
                    *action_state = ActionState::Success;
                }
            }
            ActionState::Cancelled => {
                // Always treat cancellations, or we might keep doing this forever!
                // You don't need to terminate immediately, by the way, this is only a flag that
                // the cancellation has been requested. If the actor is balancing on a tightrope,
                // for instance, you may let them walk off before ending the action.
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

fn find_closest_water_source(
    waters: &Query<&Position, With<WaterSourceComponent>>,
    actor_position: &Position,
) -> Position {
    *(waters
        .iter()
        .min_by(|a, b| {
            let da = (a.position - actor_position.position).length_squared();
            let db = (b.position - actor_position.position).length_squared();
            da.partial_cmp(&db).unwrap()
        })
        .expect("no water sources"))
}




























