use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use bevy_editor_pls::prelude::*;


pub mod plugin_player;
pub use plugin_player::*;

pub mod plugin_camera;
pub use plugin_camera::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        // .add_systems(Startup, setup_camera)
            // .add_systems(
            //     Update,
            //     ((
            //         camera_move
            //     ))
            // )
            .add_plugins((
                PlayerPlugin,
                MyCameraPlugin,


                EditorPlugin::default(),
                PhysicsPlugins::default(),
                //PhysicsDebugPlugin::default()
            ));
            
    }
}
 







