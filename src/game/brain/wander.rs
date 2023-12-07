use bevy::prelude::*;
use big_brain::prelude::*;
use rand::Rng;

use super::MoveToDestination;


#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Wander;



/// An action where the actor moves to the closest water source
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct FindRandomLocationAction;





pub fn wander_scorer_system(
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<Wander>>,
) {
    for (Actor(_actor), mut score, _span) in &mut query {  
        println!("Wander: {:?}", 1); 
        score.set(1.);
    }
}


pub fn find_random_location_action_system(
    mut action_query: Query<(&Actor, &mut ActionState, &ActionSpan), With<FindRandomLocationAction>>,
    mut movers: Query<(&Transform,&mut MoveToDestination)>,
)
{
    for (actor, mut action_state, span) in &mut action_query {
        let _guard = span.span().enter();

        // Different behavior depending on action state.
        match *action_state {
            // Action was just requested; it hasn't been seen before.
            ActionState::Requested => {
                println!("Calculating random location");
                // We don't really need any initialization code here, since the queries are cheap enough.
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Look up the actor's position.
                println!("1");
                let (transform, mut dest)  
                    = movers.get_mut(actor.0).expect("actor has no moveToDestination");
                let mut rng = rand::thread_rng();
                let x = rng.gen_range(-1000.0..1000.0);
                let y = rng.gen_range(-1000.0..1000.0);
                println!("Random location offset: {:?}, {:?}", x, y);
                dest.destination = Vec2::new(
                    transform.translation.x + x,
                    transform.translation.y + y);

                *action_state = ActionState::Success;
            }
            ActionState::Cancelled => {
                println!("Random location cancelled");
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}
