//! ECS: Entity Component System
//! 
//! Object-oriented
//! struct Person { name, age }, struct Car { plate, size, color }
//! 
//! Data-oriented, ECS
//! Entity (person, car, ...), Component (name, age, plate, size, color, ...), System

// Game: Hero, Equipment

// struct Name(&'static str);
// struct Health(i32);

// #[derive(Default)]
// struct World {
//     name_component_vec: Vec<Option<Name>>,
//     health_component_vec: Vec<Option<Health>>,
// }

// impl World {
//     fn new_entity(&mut self, name: Option<Name>, health: Option<Health>) {
//         self.name_component_vec.push(name);
//         self.health_component_vec.push(health);
//     }
// }


// fn main() {
//     let mut world = World::default();
//     world.new_entity(Some(Name("ironman")), Some(Health(80)));
//     world.new_entity(Some(Name("spiderman")), Some(Health(100)));
//     world.new_entity(Some(Name("Stone")), None);

//     // for i in 0..world.name_component_vec.len() {
//     //     if let Some(name) = world.name_component_vec.get(i) {
//     //         if let Some(health) = world.health_component_vec.get(i) {
//     //             if let Some(health) = health {
//     //                 println!("Hero {} has health {}", name.as_ref().unwrap().0, health.0);
//     //             }
//     //         }
//     //     }
//     // }

//     let heroes = world.name_component_vec.iter()
//         .zip(world.health_component_vec.iter())
//         .filter_map(|(name, health)| Some((name.as_ref()?, health.as_ref()?)));

//     for (name, health) in heroes {
//         println!("Hero {name} has health {health}", name = name.0, health = health.0);
//     }
// }

use std::cell::{Ref, RefCell, RefMut};


// Dynamic Component
struct Name(&'static str);
struct Health(i32);
struct Color(&'static str);

trait ComponentVec {
    fn push_none(&mut self);
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<ComponentType: 'static> ComponentVec for RefCell<Vec<Option<ComponentType>>> {
    fn push_none(&mut self) {
        self.get_mut().push(None);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

#[derive(Default)]
struct World {
    //  [
    //   [Some(name1), Some(name2), ...]
    //   [Some(health1), Some(health2), ...]
    //   ...
    //]
    component_vecs: Vec<Box<dyn ComponentVec>>,
    entities_count: usize
}

impl World {
    fn new_entity(&mut self) -> usize {
        let entity = self.entities_count;
        for cv in self.component_vecs.iter_mut() {
            cv.push_none();
        }
        self.entities_count += 1;
        entity
    }

    fn add_component_to_entity<ComponentType: 'static>(&mut self, entity: usize, component: ComponentType) {
        for cv in self.component_vecs.iter_mut() {
            // std::any::Any
            if let Some(cv) = cv.as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>() {
                    cv.get_mut()[entity] = Some(component);
                    return;
                }
        }

        let mut new_cv: Vec<Option<ComponentType>> = Vec::with_capacity(self.entities_count);
        for _ in 0..self.entities_count {
            new_cv.push(None);
        }
        new_cv[entity] = Some(component);
        self.component_vecs.push(Box::new(RefCell::new(new_cv)));

    }

    fn get_component_vec<ComponentType: 'static>(&self) -> Option<Ref<Vec<Option<ComponentType>>>>{
        for cv in self.component_vecs.iter() {
            if let Some(cv) = cv.as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>() {
                    return Some(cv.borrow())
                }
        }

        None
    }

    fn get_component_vec_mut<ComponentType: 'static>(&self) -> Option<RefMut<Vec<Option<ComponentType>>>>{
        // RefCell::borrow_mut() => RefMut
        for cv in self.component_vecs.iter() {
            if let Some(cv) = cv.as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>() {
                    return Some(cv.borrow_mut())
                }
        }

        None
    }
}

fn show_heros(world: &World) {
    let name_cv: Ref<Vec<Option<Name>>> = world.get_component_vec().unwrap();
    let health_cv: Ref<Vec<Option<Health>>> = world.get_component_vec().unwrap();
    let heroes = name_cv.iter()
        .zip(health_cv.iter())
        .filter_map(|(name, health)| Some((name.as_ref()?, health.as_ref()?)));
    for (name, health) in heroes {
        println!("Hero {name} has health {health}", name = name.0, health = health.0);
    }
}

fn main() {
    let mut world = World::default();
    let ironman = world.new_entity();
    let spiderman = world.new_entity();
    assert!(ironman == 0);
    assert!(spiderman == 1);
    world.add_component_to_entity(ironman, Name("Ironman"));
    world.add_component_to_entity(ironman, Health(80)); 
    world.add_component_to_entity(spiderman, Name("Spiderman"));
    world.add_component_to_entity(spiderman, Health(100));
    let stone_space = world.new_entity();
    world.add_component_to_entity(stone_space, Name("Space Stone"));
    world.add_component_to_entity(stone_space, Color("Blue"));

    show_heros(&world);

    let mut name_cv: RefMut<Vec<Option<Name>>> = world.get_component_vec_mut().unwrap();
    let mut health_cv: RefMut<Vec<Option<Health>>> = world.get_component_vec_mut().unwrap();
    let heroes = name_cv.iter_mut()
        .zip(health_cv.iter_mut())
        .filter_map(|(name, health)| Some((name.as_mut()?, health.as_mut()?)));
    for (name, health) in heroes {
        if name.0 == "Ironman" {
            health.0 += 10;
        } else if name.0 == "Spiderman" {
            health.0 -= 10;
        }
    }
    drop(name_cv);
    drop(health_cv);

    show_heros(&world);

    // show stones
    let name_cv: Ref<Vec<Option<Name>>> = world.get_component_vec().unwrap();
    let color_cv: Ref<Vec<Option<Color>>> = world.get_component_vec().unwrap();
    let stones = name_cv.iter()
        .zip(color_cv.iter())
        .filter_map(|(name, color)| Some((name.as_ref()?, color.as_ref()?)));
    for (name, color) in stones {
        println!("{name} has color {color}", name = name.0, color = color.0);
    }

}