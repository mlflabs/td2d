use bevy::{prelude::*, utils::HashMap};




pub mod actions;
pub use actions::*;

pub mod scorers;
use bevy_xpbd_2d::parry::utils::hashmap;
pub use scorers::*;

use super::{Player, MainCamera};

pub struct LittleBrainPlugin;

impl Plugin for LittleBrainPlugin {
    fn build(&self, app: &mut App) {

        let p = Point::new(2.,2.);

        app
            .add_systems(Update, (
                camera_move,
                test_system,
                //p.test_system
            ));


        let mut services: HashMap<String, Box<dyn MyTrait>> = HashMap::new();
        services.insert("abhi".to_string(), Box::new(Bar));
        services.insert("rust".to_string(), Box::new(Foo));
        //  = HashMap::from([
        //     ("abhi".to_string(), Box::new(Bar)),
        //     ("rust".to_string(), Box::new(Foo))
        // ]);

        // let mut s: HashMap<String, dyn Component> = HashMap::new();
        // s.insert("rust".to_string(), Test2);
        // s.insert("rust2".to_string(), Test);
    }
}


const X:f32 = 10.0;

pub fn test_system(
    player_query: Query<&GlobalTransform, With<Player>>,
){
    print!("T1: {}", X);
}


pub fn camera_move(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,    
){
    print!("T1: {}", X);
}


///// tag, 
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }

    fn print(&self) {
        println!("Testing Point {}", self.x);
    }


    pub fn test_system(
        player_query: Query<&GlobalTransform, With<Player>>,
    ){
        print!("T1: {}", X);
    }

    pub fn test_system2(
        &self,
        cmd: &mut Commands,
    ){
        print!("T1: {}", self.x.clone());
    }
}



// pub struct TestTest5 {
//     pub t1: f32,
// }

// impl TestTest5 {
//     pub fn wander_scorer_system(
//         cmd: &mut Commands,
//     ){
//         print!("T1: {}", t1);
//     }
// }




#[derive(Component, Reflect, Default, Debug)]
pub struct Test2;



#[derive(Component, Reflect, Default, Debug)]
pub struct Test;


pub struct LittleBrain {
    pub scorers: HashMap<String, Box<dyn MyTrait>>,
    pub actions: HashMap<String, String>,
}

//Implement a funtion and pass that function


enum Actions {
    Tst, tset, test, testset,
}

fn cleanup_scorers<Func, T>(){

}



#[derive(Component, Reflect, Default, Debug)]
pub struct LittleBrainTag;


#[derive(Component, Reflect, Default, Debug)]
pub struct LittleBrainExecutingActionTag;



pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}


#[derive(Component, Default, Debug)]
struct Foo;

#[derive(Component, Default, Debug)]
struct Bar;

pub trait MyTrait {
    fn myfunc(&self);
  }

impl MyTrait for Foo{
    fn myfunc(&self){
    }
}

impl MyTrait for Bar{
    fn myfunc(&self){
    }
}

