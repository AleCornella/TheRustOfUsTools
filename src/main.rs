use rand::Rng;
use robotics_lib;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{destroy, put, robot_map, where_am_i, Direction, Tools};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::utils::LibError::*;
use robotics_lib::utils::{in_bounds, LibError};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use robotics_lib::world::tile::Content::{
    Bank, Bin, Coin, Crate, Fire, Fish, Garbage, Market, Rock, Tree,
};
use robotics_lib::world::tile::TileType::*;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::World;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use strum::IntoEnumIterator;
fn main() {
    let my_banana = MyRobot(Robot::new());

    let mut generator = WorldGenerator::init(2);
    let run = Runner::new(Box::new(my_banana), &mut generator);

    match run {
        Ok(mut x) => {
            let _ = x.game_tick();
        }
        Err(x) => {
            println!("Oh shit");
            println!("{:?}", x);
        }
    }
}

struct Tool {}
pub enum State {
    GET_STONES,
    MAKE_ROAD,
}
#[derive(Debug)]
pub enum RpmError {
    NotEnoughEnergy,
    CannotPlaceHere,
    OutOfBounds,
    NotEnoughMaterial,
    NoRockHere,
    MustDestroyContentFirst,
    UndefinedError,
}
impl Tool {
    fn error_handing(error: Result<usize, LibError>) -> Result<(), RpmError> {
        match error {
            Ok(_) => Ok(()),
            Err(e) => match e {
                NotEnoughEnergy => Err(RpmError::NotEnoughEnergy),
                OutOfBounds => Err(RpmError::OutOfBounds),
                NotEnoughContentInBackPack => Err(RpmError::NotEnoughMaterial),
                ContentNotAllowedOnTile => Err(RpmError::CannotPlaceHere),
                MustDestroyContentFirst => Err(RpmError::MustDestroyContentFirst),
                NoContent => Err(RpmError::NoRockHere),
                NotEnoughContentProvided => Err(RpmError::NotEnoughMaterial),
                _ => Err(RpmError::UndefinedError),
            },
        }
    }
    pub fn road_paving_machine(
        robot: &mut impl Runnable,
        world: &mut World,
        direction: Direction,
        state: State,
    ) -> Result<(), RpmError> {
        where_am_i(robot, world);
        if robot
            .get_backpack()
            .get_contents()
            .get(&Content::Rock(0))
            .is_none()
        {
            return Err(RpmError::NotEnoughMaterial);
        }

        let tile: Tile;
        match in_bounds(robot, world, &direction) {
            Ok(_) => {}
            Err(_) => return Err(RpmError::OutOfBounds),
        }

        let mut col = robot.get_coordinate().get_col();
        let mut row = robot.get_coordinate().get_row();
        match direction {
            Direction::Up => {
                row -= 1;
            }
            Direction::Down => {
                row += 1;
            }
            Direction::Left => {
                col -= 1;
            }
            Direction::Right => {
                col += 1;
            }
        }

        match robot_map(world) {
            None => {
                return Err(RpmError::OutOfBounds);
            }
            Some(map) => {
                tile = map[row][col].clone().unwrap();
            }
        }

        match state {
            State::GET_STONES => {
                let error = destroy(robot, world, direction);
                return Self::error_handing(error);
            }
            State::MAKE_ROAD => {
                match tile.tile_type {
                    DeepWater => {
                        let error = put(robot, world, Content::Rock(0), 3, direction);
                        return Self::error_handing(error);
                    }
                    ShallowWater => {
                        let error = put(robot, world, Content::Rock(0), 2, direction);
                        return Self::error_handing(error);
                    }
                    Sand => {
                        let error = put(robot, world, Content::Rock(0), 1, direction);
                        return Self::error_handing(error);
                    }
                    Grass => {
                        println!("QUA");
                        println!("{:?}", robot.get_backpack_mut());
                        let error = put(robot, world, Content::Rock(0), 1, direction);
                        return Self::error_handing(error);
                    }
                    Street => {
                        return Err(RpmError::CannotPlaceHere);
                    }
                    Hill => {
                        let error = put(robot, world, Content::Rock(0), 1, direction);
                        return Self::error_handing(error);
                    }
                    Mountain => {
                        let error = put(robot, world, Content::None, 0, direction);
                        return Self::error_handing(error);
                    }
                    Snow => {
                        let error = put(robot, world, Content::Rock(0), 1, direction);
                        return Self::error_handing(error);
                    }
                    Lava => {
                        let error = put(robot, world, Content::Rock(0), 3, direction);
                        return Self::error_handing(error);
                    }
                    Teleport(_) => {
                        return Err(RpmError::CannotPlaceHere);
                    }
                    Wall => {
                        return Err(RpmError::CannotPlaceHere);
                    }
                }
                println!("{:?}", robot.get_backpack());
                println!("{:?}", robot.get_backpack());
                Ok(())
            }
        }
    }
}

impl Tools for Tool {}

pub struct MyRobot(Robot);

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        where_am_i(self, world);

        println!(
            "{:?}",
            Tool::road_paving_machine(self, world, Direction::Down, State::GET_STONES)
        );

        println!(
            "{:?}",
            Tool::road_paving_machine(self, world, Direction::Right, State::MAKE_ROAD)
        );
        println!("{:?}", robot_map(world));
    }

    fn handle_event(&mut self, event: Event) {
        println!();
        // println!("{:?}", event);
        println!();
        //println!("test");
    }

    fn get_energy(&self) -> &Energy {
        &self.0.energy
    }

    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.0.energy
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.0.coordinate
    }

    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.0.coordinate
    }

    fn get_backpack(&self) -> &BackPack {
        &self.0.backpack
    }

    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.0.backpack
    }
}

struct WorldGenerator {
    size: usize,
}

impl WorldGenerator {
    fn init(size: usize) -> Self {
        WorldGenerator { size }
    }
}

impl Generator for WorldGenerator {
    fn gen(
        &mut self,
    ) -> (
        Vec<Vec<Tile>>,
        (usize, usize),
        EnvironmentalConditions,
        f32,
        Option<HashMap<Content, f32>>,
    ) {
        let mut rng = rand::thread_rng();
        let mut map: Vec<Vec<Tile>> = Vec::new();
        // Initialize the map with default tiles
        for _ in 0..self.size {
            let mut row: Vec<Tile> = Vec::new();
            for _ in 0..self.size {
                let i_tiletype = rng.gen_range(0..TileType::iter().len());
                let i_content = rng.gen_range(0..Content::iter().len());
                let tile_type = match i_tiletype {
                    0 => DeepWater,
                    1 => ShallowWater,
                    2 => Sand,
                    3 => Grass,
                    4 => Street,
                    5 => Hill,
                    6 => Mountain,
                    7 => Snow,
                    8 => Lava,
                    9 => Teleport(false),
                    _ => Grass,
                };
                let content = match i_content {
                    0 => Rock(0),
                    1 => Tree(2),
                    2 => Garbage(2),
                    3 => Fire,
                    4 => Coin(2),
                    5 => Bin(2..3),
                    6 => Crate(2..3),
                    7 => Bank(3..54),
                    8 => Content::Water(20),
                    9 => Content::None,
                    10 => Fish(3),
                    11 => Market(20),
                    12 => Content::Building,
                    13 => Content::Bush(2),
                    14 => Content::JollyBlock(2),
                    15 => Content::Scarecrow,
                    _ => Content::None,
                };
                row.push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                });
            }
            row[1] = Tile {
                tile_type: TileType::Lava,
                content: Content::None,
                elevation: 0,
            };
            map.push(row);
        }
        let environmental_conditions =
            EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12).unwrap();

        let max_score = rand::random::<f32>();

        (map, (0, 0), environmental_conditions, max_score, None)
    }
}
