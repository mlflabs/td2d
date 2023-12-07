use bevy::{prelude::*, utils::HashMap};




pub mod actions;
pub use actions::*;

pub mod scorers;
use bevy_xpbd_2d::parry::utils::hashmap;
pub use scorers::*;

pub struct LittleBrainPlugin;

impl Plugin for LittleBrainPlugin {
    fn build(&self, app: &mut App) {



        let mut services: HashMap<String, Box<dyn MyTrait>> = HashMap::new();
        services.insert("abhi".to_string(), Box::new(Bar));
        services.insert("rust".to_string(), Box::new(Foo));
        //  = HashMap::from([
        //     ("abhi".to_string(), Box::new(Bar)),
        //     ("rust".to_string(), Box::new(Foo))
        // ]);

        let mut s: HashMap<String, dyn Component> = HashMap::new();
        s.insert("rust".to_string(), Test2);
        s.insert("rust2".to_string(), Test);
    }
}

///// tag, 




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

