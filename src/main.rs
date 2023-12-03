use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;



mod game;
use game::*;




fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // resolution: (800., 460.).into(), //2850, 0
                        position: WindowPosition::At((3850,1040).into()),
                        resolution: (1200., 600.).into(), //2850, 0
                        // position: WindowPosition::At((2550,0).into()),

                        //resolution: (1280., 720.).into(),
                        //position: (0, 0).into(),
                        // fill the entire browser window
                        fit_canvas_to_parent: true,
                        // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    
                    ..default()
            }),
            LdtkPlugin,

            //Our plugins
            GamePlugin
        ))
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map1.ldtk"),
        ..Default::default()
    });
}

#[derive(Default, Component)]
struct ComponentA;

#[derive(Default, Component)]
struct ComponentB;

#[derive(Default, Bundle, LdtkEntity)]
pub struct MyBundle {
    a: ComponentA,
    b: ComponentB,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}