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
//! 
//! Part 03: Rendering 
//!     - add images into Context
//!     - GameImages as ggez Resource
//!     - SysRender, EventHandler::draw()
//! Challenges:
//!     - coordinates: x >= 0, y >= 0, boundary
//!     - draw wall, crate ...
//!     - different character images for different input keys
//! 


use std::{collections::HashMap, path::PathBuf};

use ggez::{conf::WindowMode, event::{self, EventHandler}, graphics::{self, Canvas, Color, DrawParam, Image}, input::keyboard::KeyCode, Context, ContextBuilder, GameError, GameResult};
use specs::{Builder, Component, Join, NullStorage, ReadStorage, DenseVecStorage, RunNow, System, VecStorage, World, WorldExt, WriteStorage};
use specs::{Read, Write};

const GAME_WIDTH: u32 = 20;
const GAME_HEIGHT: u32 = 20;
const GAME_UNIT_SIZE: f32 = 64.0;

// Components
struct Position {
    x: u32,
    y: u32
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
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

#[derive(PartialEq, Eq, Hash)]
#[derive(Component)]
enum EntityType {
    PlayerUp,
    PlayerDown,
    PlayerLeft,
    PlayerRight,
    Wall,
    Crate
}

#[derive(Default)]
struct GameImages {
    entity_images: HashMap<EntityType, Image>
}

impl GameImages {
    fn new(ctx: &Context) -> Result<Self, GameError> {
        let mut entity_images = HashMap::new();
        let _ = entity_images.insert(EntityType::PlayerUp, Image::from_path(ctx, "/Character7.png")?);
        let _ = entity_images.insert(EntityType::PlayerDown, Image::from_path(ctx, "/Character4.png")?);
        let _ = entity_images.insert(EntityType::PlayerLeft, Image::from_path(ctx, "/Character1.png")?);
        let _ = entity_images.insert(EntityType::PlayerRight, Image::from_path(ctx, "/Character2.png")?);
        let _ = entity_images.insert(EntityType::Crate, Image::from_path(ctx, "/CrateDark_Beige.png")?);
        let _ = entity_images.insert(EntityType::Wall, Image::from_path(ctx, "/Wall_Beige.png")?);

        Ok(Self { entity_images })
    }
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
        ReadStorage<'a, Moverable>,
        WriteStorage<'a, EntityType>,
    );

    fn run(&mut self, (mut q , mut positions, moverables, mut etypes): Self::SystemData) {
        for (pos, _mov, etype) in (&mut positions, &moverables, &mut etypes).join() { // player
            if let Some(keycode) = q.keys_pressed.pop() {
                match keycode {
                    KeyCode::Up => {
                        pos.y = pos.y.saturating_sub(1);
                        *etype = EntityType::PlayerUp;
                    }
                    KeyCode::Down => if pos.y < GAME_HEIGHT - 1 {
                        pos.y += 1;
                        *etype = EntityType::PlayerDown;
                    },
                    KeyCode::Left => {
                        pos.x = pos.x.saturating_sub(1);
                        *etype = EntityType::PlayerLeft;
                    }
                    KeyCode::Right => if pos.x < GAME_WIDTH -1 {
                        pos.x += 1;
                        *etype = EntityType::PlayerRight;
                    }
                    _ => ()
                }
            }
        }
    }
}

// system: rendering

struct SysRender {
    canvas: Canvas
}

impl<'a> System<'a> for SysRender {
    type SystemData = (
        Read<'a, GameImages>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, EntityType>,
    );

    fn run(&mut self, (images, positions, etypes): Self::SystemData) {
        for (position, etype) in (&positions, &etypes).join() {
            let image = images.entity_images.get(etype).unwrap();
            let x = position.x as f32 * 64.0;
            let y = position.y as f32 * 64.0;
            let dest = ggez::glam::Vec2::new(x, y);
            self.canvas.draw(image, DrawParam::new().dest(dest))

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
        let mut sys_render = SysRender { canvas };
        sys_render.run_now(&self.world);
        sys_render.canvas.finish(ctx)
    }
}

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .add_resource_path(PathBuf::from("..").join("images").join("PNG"))
        .window_mode(WindowMode::default().dimensions(GAME_WIDTH as f32 * GAME_UNIT_SIZE , GAME_HEIGHT as f32 * GAME_UNIT_SIZE))
        .build()
        .expect("aieee, could not create ggez context!");

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Moverable>();
    world.register::<EntityType>();
    world.insert(InputKeyQueue::default());
    world.insert(GameImages::new(&ctx).unwrap());
    world.create_entity().with(Position { x: 0, y: 0 }).with(Moverable {}).with(EntityType::PlayerRight).build(); // player
    world.create_entity().with(Position { x: 10, y: 10}).with(EntityType::Wall).build(); // wall
    world.create_entity().with(Position { x: 11, y: 11}).with(EntityType::Crate).build(); // crate

    let my_game = MyGame::new(&mut ctx, world);
    event::run(ctx, event_loop, my_game);
}
