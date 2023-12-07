use bevy::prelude::*;
use big_brain::prelude::*;

use crate::game::MAP_MID_Z;





#[derive(Clone, Component, Debug, ActionBuilder, Reflect)]
pub struct MoveAction;



#[derive(Clone, Component, Debug, Reflect)]
pub struct MoveToDestination {
    // The movement speed of the actor.
    pub destination: Vec2,
    pub success_distance: f32,
}


impl MoveToDestination{
    pub fn new(destination: Vec2, success_distance: f32) -> Self {
        Self { destination,  success_distance }
    }
}

#[derive(Clone, Component, Debug, Reflect)]
pub struct Speed {
    // The movement speed of the actor.
    pub value: f32
}


impl Speed {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}



pub fn move_to_destination(
    time: Res<Time>,
    mut action_query: Query<(&Actor, &mut ActionState, &MoveAction, &ActionSpan)>,
    mut movers: Query<(&Speed, &MoveToDestination, &mut Transform)>,
   
) {

    for (actor, mut action_state, move_to, span) in &mut action_query {
        let _guard = span.span().enter();

        match *action_state {
            ActionState::Requested => {
                println!("Moving To Destinaton");
                // We don't really need any initialization code here, since the queries are cheap enough.
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Look up the actor's position.
                println!("11");


                let (speed, dest, mut transform) 
                    = movers.get_mut(actor.0).expect("actor has no transform");
                println!("22");
                // Find how far we are from it.
                let delta = dest.destination - Vec2::from((transform.translation.x, transform.translation.y));

                let distance = delta.length();

                println!("Moving {:?} to {:?}", distance, dest);

                if distance > dest.success_distance {
                    println!("33");
                    let step_size = time.delta_seconds() * speed.value;
                    let step = delta.normalize() * step_size.min(distance);
                    println!("44");
                    transform.translation += Vec3::from((step, MAP_MID_Z));
                    println!("55");
                } else {
                    println!("SUCCESS****************************");
                    *action_state = ActionState::Success;
                }
            }
            ActionState::Cancelled => {
                println!("Move Action Failed");
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}
