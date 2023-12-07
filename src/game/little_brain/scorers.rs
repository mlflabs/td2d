use bevy::prelude::*;

use super::{LittleBrainExecutingActionTag, RestAction, WanderAction};




#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScorerList {
    None, Wander, Rest
}

impl Default for ScorerList {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ScorerStep {
    Init, Evaluating, CleaningUp, Finished
}

impl Default for ScorerStep {
    fn default() -> Self {
        Self::Evaluating
    }
}




#[derive(Component, Debug, Default)]
pub struct ScorerAction {
    

}


#[derive(Component, Debug, Default)]
pub struct Score {
    pub value: f32,
    pub step: ScorerStep,
    pub scorer: ScorerList,
    pub previous_winner: ScorerList,
    pub previous_winner2: ScorerList,
    pub prevent_evaluation_finish: ScorerList
}




pub fn score_management_system(
    cmd: &mut Commands,
    mut scorers: Query<(Entity, &mut Score), Without<LittleBrainExecutingActionTag>>,
){

    for (e, mut score) in scorers.iter_mut() {
        score.step = match score.step {
            ScorerStep::Init => {

                score.previous_winner2 = score.previous_winner.clone();
                score.previous_winner = score.scorer.clone();
                score.value = 0.;
                score.scorer = ScorerList::None;

                ScorerStep::Evaluating
            },
            ScorerStep::Evaluating => ScorerStep::CleaningUp,
            ScorerStep::CleaningUp => {
                cmd.entity(e).insert(LittleBrainExecutingActionTag);
                ScorerStep::Finished
            },
            ScorerStep::Finished => ScorerStep::Finished




        }
    }
}








#[derive(Component, Reflect, Default, Debug)]
pub struct WanderScorerTag;


pub fn wander_scorer_system(
    cmd: &mut Commands,
    mut scorers: Query<(Entity, &mut Score), (With<WanderScorerTag>, Without<LittleBrainExecutingActionTag>)>
){
    for (e, mut score) in scorers.iter_mut() {

        match score.step {
            ScorerStep::Evaluating=> {
                if score.previous_winner == ScorerList::Wander {
                    if score.value < 0.5 {
                        score.value = 0.5;
                        score.scorer = ScorerList::Wander;
                    }
                    
                }
                else {
                    if score.value < 1. {
                        score.value = 1.;
                        score.scorer = ScorerList::Wander;
                    }
                }
            },
            ScorerStep::CleaningUp=> {
                if score.scorer == ScorerList::Wander {
                    cmd.entity(e).insert(WanderAction);
                }
            }
            _ => println!("Not supported step")
        };
    }
}





#[derive(Component, Reflect, Default, Debug)]
pub struct RestScorerTag;





pub fn rest_scorer_system(
    cmd: &mut Commands,
    mut scorers: Query<(Entity, &mut Score), (With<RestScorerTag>, Without<LittleBrainExecutingActionTag>)>
){
    for (e, mut score) in scorers.iter_mut() {
        if score.step == ScorerStep::Evaluating {
                if score.value < 0.8 {
                    score.value = 0.8;
                    score.scorer = ScorerList::Rest;
                }
        }
        else if score.step == ScorerStep::CleaningUp {
            if score.scorer == ScorerList::Rest {
                cmd.entity(e).insert(RestAction);
            }
        }
    }
}


