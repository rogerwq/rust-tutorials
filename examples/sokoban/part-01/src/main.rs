//! Game Sokoban
//!     Player, Box, BoxTarget, Wall, Space
//! 
//! Part 01: ECS - specs
//!     - create a world
//!     - create entities, components
//!     - create systems
//! 

// use std::{io::Read, path::PathBuf};

use specs::{Builder, Component, DispatcherBuilder, Join, NullStorage, Read, ReadStorage, System, VecStorage, World, WorldExt, WriteStorage};

// Components
struct Position {
    x: i32,
    y: i32
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

// struct Image {
//     path: PathBuf
// }

// impl Component for Image {
//     type Storage = VecStorage<Self>;
// }

struct Moverable;  // mark component

impl Component for Moverable {
    type Storage = NullStorage<Self>;
}


// Resource
#[derive(Default)]
struct Move {
    x: i32, 
    y: i32
}

// System: show player

struct SysShowPlayer;

impl<'a> System<'a> for SysShowPlayer {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Moverable>);

    fn run(&mut self, (positions, moverables): Self::SystemData) {
        for (pos, _mov) in (&positions, &moverables).join() {
            println!("Player currently at x: {}, y: {}", pos.x, pos.y);
        }
    }
}

struct SysMovePlayer;

impl<'a> System<'a> for SysMovePlayer {
    type SystemData = (
        Read<'a, Move>,
        WriteStorage<'a, Position>, 
        ReadStorage<'a, Moverable>
    );

    fn run(&mut self, (mov_pos, mut positions, moverables): Self::SystemData) {
        for (pos, _mov) in (&mut positions, &moverables).join() {
            pos.x += mov_pos.x;
            pos.y += mov_pos.y;
        }
    }
}

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Moverable>();
    world.insert(Move { x: 1, y: 0 });

    world.create_entity().with(Position { x: 0, y: 0 }).with(Moverable {}).build(); // player
    world.create_entity().with(Position { x: 10, y: 10}).build(); // wall

    // first dispatcher
    let mut dispatcher = DispatcherBuilder::new()
        .with(SysShowPlayer, "show_player", &[])
        .with(SysMovePlayer, "move_player", &["show_player"])
        .build();
    dispatcher.dispatch(&world);
    world.maintain();

    //  2nd dispatcher, (0, 0) => (1, 0)
    let mut dispatcher = DispatcherBuilder::new()
        .with(SysShowPlayer, "show_player_2", &[])
        .build();
    dispatcher.dispatch(&world);
    world.maintain();

    // 3rd dispatcher, (1, 0) => (2, -1) 
    {
        let mut move_pos = world.write_resource::<Move>();
        move_pos.y = -1;
    }
    let mut dispatcher = DispatcherBuilder::new()
        .with(SysMovePlayer, "move_player", &[])
        .build();
    dispatcher.dispatch(&world);
    world.maintain();
    let mut dispatcher = DispatcherBuilder::new()
        .with(SysShowPlayer, "show_player_2", &[])
        .build();
    dispatcher.dispatch(&world);
    world.maintain();

}
