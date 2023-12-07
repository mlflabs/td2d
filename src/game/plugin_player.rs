use bevy::prelude::*;
use bevy_xpbd_2d::components::{RigidBody, Collider, LinearVelocity, GravityScale};
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::{orientation::Direction};
use bevy_spine::{
    SkeletonController, SkeletonData, Spine, SpineBundle, SpinePlugin, SpineReadyEvent, SpineSet,
};

use super::MAP_MID_Z;

pub const PLAYER_SPEED: f32 = 500.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                SpinePlugin, 
                InputManagerPlugin::<PlayerAction>::default()))
            .add_systems(Update, on_spawn.in_set(SpineSet::OnReady))
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (
                player_walks, 
            
            ));
        
    } 
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
/// Demo marker component
pub struct Player;


#[derive(Component)]
pub struct GridPosition {
    pub pos: Vec2,
}





#[derive(Debug, Clone, Eq, PartialEq, Reflect, Hash)]
pub enum PlayerStates {
    Idle,
    Move,
    Dash,
}

impl Default for PlayerStates {
    fn default() -> Self {
        Self::Idle
    }
}


#[derive(Component, Clone, Default, Reflect, Debug)]
pub struct CurrentState {
    state: PlayerStates,
}

impl CurrentState {
    pub fn new(state: PlayerStates) -> Self { Self { state } }
}


#[derive(Component, Clone, Debug)]
struct PlayerIdle;

#[derive(Component, Clone, Debug)]
struct PlayerDashing;

#[derive(Component, Clone, Debug)]
struct PlayerDoubleJumping;


#[derive(Component, Clone, Debug)]
struct PlayerWalking;

#[derive(Component, Clone, Debug)]
struct PlayerRunning;

#[derive(Component)]
struct InteractionRayCaster;



#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
    Dash,
    Interact,
    Move,

}


impl PlayerAction {
    // Lists like this can be very useful for quickly matching subsets of actions
    pub const DIRECTIONS: [Self; 4] = [
        PlayerAction::Up,
        PlayerAction::Down,
        PlayerAction::Left,
        PlayerAction::Right,
    ];

    pub fn direction(self) -> Option<Direction> {
        match self {
            PlayerAction::Up => Some(Direction::NORTH),
            PlayerAction::Down => Some(Direction::SOUTH),
            PlayerAction::Left => Some(Direction::WEST),
            PlayerAction::Right => Some(Direction::EAST),
            _ => None,
        }
    }
}



fn spawn_player(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut skeletons: ResMut<Assets<SkeletonData>>,
    //player: Query<Entity, Added<Player>>,
) {





    //Controller
    let input_map = InputMap::new([
        // WASD
        (KeyCode::W, PlayerAction::Up),
        (KeyCode::S, PlayerAction::Down),
        (KeyCode::A, PlayerAction::Left),
        (KeyCode::D, PlayerAction::Right),
        (KeyCode::W, PlayerAction::Move),
        (KeyCode::S, PlayerAction::Move),
        (KeyCode::A, PlayerAction::Move),
        (KeyCode::D, PlayerAction::Move),
        // Cursor keys
        (KeyCode::Up, PlayerAction::Up),
        (KeyCode::Down, PlayerAction::Down),
        (KeyCode::Left, PlayerAction::Left),
        (KeyCode::Right, PlayerAction::Right),
        (KeyCode::Up, PlayerAction::Move),
        (KeyCode::Down, PlayerAction::Move),
        (KeyCode::Left, PlayerAction::Move),
        (KeyCode::Right, PlayerAction::Move),
        // Space
        (KeyCode::Space, PlayerAction::Dash),
        // E
        (KeyCode::E, PlayerAction::Interact),
    ]);
    

    let skeleton = SkeletonData::new_from_json(
        asset_server.load("spineboy/export/spineboy-pro.json"),
        asset_server.load("spineboy/export/spineboy-pma.atlas"),
    );
    let skeleton_handle = skeletons.add(skeleton);
    
    

    let player_id = commands.spawn((
            Player,
            CurrentState::new(PlayerStates::Idle),
            RigidBody::Dynamic,
            GravityScale(0.0),
            Collider::ball(0.5),
            //TransformBundle::from_transform(Transform::from_xyz(0.0, 3.0, 0.0)),

            SpineBundle {
                skeleton: skeleton_handle.clone(),
                transform: Transform::from_xyz(1000., 1000., MAP_MID_Z).with_scale(Vec3::ONE * 0.4),
                ..Default::default()
            },
            // SpriteBundle {
            //     transform: Transform::from_xyz(0.0, 0.0, 10.),
            //     sprite: Sprite {
            //         color: Color::rgb(0., 0.47, 1.),
            //         custom_size: Some(Vec2::new(10., 10.)),
            //         ..default()
            //     },
            //     ..default()
            // },

            InputManagerBundle::<PlayerAction> {
                input_map,
                ..default() }

        ))
    //.insert(player_state_machine(entity))
    .insert(Name::new("Player"));

}



pub fn player_walks(
    input_query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut player_query: Query<&mut LinearVelocity, With<Player>>,
    //time: Res<Time>
){
    
    let action_state = input_query.single();
    let mut direction_vector = Vec2::ZERO;
    for input_direction in PlayerAction::DIRECTIONS {
        
        if action_state.pressed(input_direction) {
            if let Some(direction) = input_direction.direction() {
                // Sum the directions as 2D vectors
                direction_vector += Vec2::from(direction);
                // println!("direction: {}", direction);
            }
        }
    }
    direction_vector = direction_vector.normalize_or_zero();
    if direction_vector == Vec2::ZERO {
        if let Ok(mut linear_velocity) = player_query.get_single_mut() {

            linear_velocity.x *= 0.1;
            linear_velocity.y *= 0.1;
        }
    }

    if let Ok(mut linear_velocity) = player_query.get_single_mut() {
        linear_velocity.x = direction_vector.x  * PLAYER_SPEED;
        linear_velocity.y = direction_vector.y * PLAYER_SPEED;
    }


}







fn on_spawn(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut spine_query: Query<&mut Spine>,
) {
    for event in spine_ready_event.iter() {
        if let Ok(mut spine) = spine_query.get_mut(event.entity) {
            let Spine(SkeletonController {
                animation_state, ..
            }) = spine.as_mut();
            let _ = animation_state.set_animation_by_name(0, "walk", true);
        }
    }
}








