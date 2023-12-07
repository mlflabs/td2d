use bevy::prelude::*;

use super::Player;


pub struct MyCameraPlugin;

impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (
                camera_move, 
            
            ));
        
    } 
}




#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(1000.0, 1800.0, 0.0),
            projection: OrthographicProjection {
                // don't forget to set `near` and `far`
                near: -1000.0,
                far: 1000.0,
                scale: 10.1,
                ..default()
            },
            ..default()
        },
        MainCamera,
    ));
}


pub fn camera_move(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,    
){
    if let Ok (player_transform) = player_query.get_single() {
        if let Ok((mut camera, mut p)) =  camera_query.get_single_mut() {
            camera.translation = player_transform.compute_transform().translation + Vec3::new(0.0, 0.0, 100.);
            p.scale = 6.0;
        }
    
    }
}
