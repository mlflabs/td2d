use bevy::prelude::*;

use super::LittleBrainExecutingActionTag;


#[derive(Component, Reflect, Default, Debug)]
pub struct RestAction;



#[derive(Component, Default, Debug)]
pub enum ActionState {
    #[default] Init, Running, Cleanup, Finished
}

#[derive(Component, Default, Debug)]
pub struct Action {
    pub state: ActionState

}



#[derive(Component, Reflect, Default, Debug)]
pub struct WanderAction;


pub fn wander_action_system(
    cmd: &mut Commands,
    mut actions: Query<(Entity, &mut Action), (With<WanderAction>, With<LittleBrainExecutingActionTag>)>
){
    for (e, mut action) in actions.iter_mut() {
        action.state = match action.state {
            ActionState::Init => {
                println!("Wander, Initial");
                ActionState::Running
            },
            ActionState::Running => {
                ActionState::Cleanup
            },
            ActionState::Cleanup => {
                cmd.entity(e).remove::<WanderAction>();


                ActionState::Finished
            },
            ActionState::Finished => ActionState::Finished
        };
    }
}


pub fn rest_action_system(
    cmd: &mut Commands,
    mut actions: Query<(Entity, &mut Action), (With<RestAction>, With<LittleBrainExecutingActionTag>)>
){
    for (e, mut action) in actions.iter_mut() {
        action.state = match action.state {
            ActionState::Init => {
                println!("Rest, Initial");
                ActionState::Running
            },
            ActionState::Running => {
                ActionState::Cleanup
            },
            ActionState::Cleanup => {
                cmd.entity(e).remove::<RestAction>();
                ActionState::Finished
            },
            _ => {
                println!("Found not support option");
                ActionState::Finished
            }
        };
    }
}