//! Game Sokoban
//!     Player, Box, BoxTarget, Wall, Space
//! 
//! Part 01: ECS - specs
//!     - create a world
//!     - create entities, components
//!     - create systems
//!
//! Part 02: User Input (keyboard) 
//!     - dispatcher -> system.run_now()
//!     - add ggez main window
//!     - handle user inputs
//!         - resource Move => InputKeyQueue
//!         - EventHandler key_down_event()  => save input keys into the queue
//!         - SysMovePlayer => move player from the input keys 
//!         - EventHandler update()  => run SysMovePlayer, SysShowPlayer
//! 
//! Links:
//! https://github.com/ggez/ggez
//! https://github.com/ggez/ggez/blob/master/examples/input_test.rs


// use std::{io::Read, path::PathBuf};

use ggez::{event::{self, EventHandler}, graphics::{self, Color}, input::keyboard::KeyCode, Context, ContextBuilder, GameResult};
use specs::{Builder, Component, Join, NullStorage, ReadStorage, RunNow, System, VecStorage, World, WorldExt, WriteStorage};
use specs::Write;

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
//  Resource vs Component
#[derive(Default)]
struct InputKeyQueue {
    keys_pressed: Vec<KeyCode>
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
        Write<'a, InputKeyQueue>,
        WriteStorage<'a, Position>, 
        ReadStorage<'a, Moverable>
    );

    fn run(&mut self, (mut q , mut positions, moverables): Self::SystemData) {
        for (pos, _mov) in (&mut positions, &moverables).join() { // player
            if let Some(keycode) = q.keys_pressed.pop() {
                match keycode {
                    KeyCode::Up => pos.y += 1,
                    KeyCode::Down => pos.y -= 1,
                    KeyCode::Left => pos.x -= 1,
                    KeyCode::Right => pos.x += 1,
                    _ => ()
                }
            }
        }
    }
}

struct MyGame {
    world: World
}

impl MyGame {
    pub fn new(_ctx: &mut Context, w: World) -> MyGame {
        MyGame { world: w }
    }
}

impl EventHandler for MyGame {
    fn key_down_event(
            &mut self,
            _ctx: &mut Context,
            input: ggez::input::keyboard::KeyInput,
            _repeated: bool,
        ) -> Result<(), ggez::GameError> {
            if let Some(keycode) = input.keycode {
                println!("{:?} pressed", keycode);
                let mut q = self.world.write_resource::<InputKeyQueue>();
                q.keys_pressed.push(keycode);
            }
            Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let mut sys_move_player = SysMovePlayer {};
        sys_move_player.run_now(&self.world);
        let mut sys_show_player = SysShowPlayer {};
        sys_show_player.run_now(&self.world);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        canvas.finish(ctx)
    }
}

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Moverable>();
    world.insert(InputKeyQueue::default());
    world.create_entity().with(Position { x: 0, y: 0 }).with(Moverable {}).build(); // player
    world.create_entity().with(Position { x: 10, y: 10}).build(); // wall

    let my_game = MyGame::new(&mut ctx, world);
    event::run(ctx, event_loop, my_game);
}
