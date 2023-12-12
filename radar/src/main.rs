use rand::Rng;
use robotics_lib;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::one_direction_view;
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
pub enum RadarError {
    NotEnoughEnergy,
    CannotPlaceHere,
    OutOfBounds,
    NotEnoughMaterial,
    NoRockHere,
    MustDestroyContentFirst,
    UndefinedError,
}
/// Given the: robot, world, direction, distance, state and TileType, it will return the coordinates of the closest tile of TileType.
/// The tool will search in a rectangle of 3 x distance, starting from you position and the two tiles in your sides.
///
/// #Usage
///```
/// Todo
///
/// #Arguments
/// - `robot`: The robot
/// - `world`: The world
/// - `direction`: The direction in wkich the robot is looking 
/// - `distance`: How long you want to search
///
/// #Returns
/// -`(Tile, usize, usize)`:returns the tile and it's own coordinates (row and columns)
/// -`LibError`: The error that ocurred
///
/// #Errors
/// - `NotEnoughEnergy`: The robot doesn't have enough energy to see the tile
/// - `NoContent`: The tile you are looking for doesn't exist there
/// - `OutOfBounds`: You tried to look for a tile which is outside of the world
///
/// #Examples
///```rust
/// use robotics_lib::interface::one_direction_view;
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::world::World;
/// use robotics_lib::world::tile::Tile;
/// use robotics_lib::interface::Direction;
/// use //Todo(Where is implemented the tool)
///
/// Todo
impl Tool {
    pub fn radar(
        robot: &mut impl Runnable,
        world: &mut World,
        direction: Direction,
        distance: usize,
        tile_type: TileType,
    ) -> Result<(Tile, usize, usize), LibError> {
        let coordinates = where_am_i(robot, world).1;
        let row = coordinates.0;
        let col = coordinates.1;

        let mut curr_scope = (row, col);
        let mut curr_dist = 2;

        while curr_dist <= distance {
            match direction {
                Direction::Up => curr_scope.0 += curr_dist,
                Direction::Right => curr_scope.1 += curr_dist,
                Direction::Left => curr_scope.1 -= curr_dist,
                Direction::Down => curr_scope.0 -= curr_dist,
            }

            match one_direction_view(robot, world, direction.clone(), curr_dist) {
                Ok(tiles) => match direction {
                    Direction::Up => {
                        for i in (tiles.len() - 1..=0).rev() {
                            for j in 0..tiles.len() {
                                if tiles[i][j].tile_type == tile_type {
                                    return Ok((tiles[i][j].clone(), row + curr_dist, col - 1 + j));
                                }
                            }
                        }
                    }
                    Direction::Down => {
                        for i in (tiles.len() - 1..=0).rev() {
                            for j in 0..tiles.len() {
                                if tiles[i][j].tile_type == tile_type {
                                    return Ok((tiles[i][j].clone(), row + curr_dist, col - 1 + j));
                                }
                            }
                        }
                    }
                    Direction::Left => {
                        for i in (tiles.len() - 1..=0).rev() {
                            for j in 0..tiles.len() {
                                if tiles[i][j].tile_type == tile_type {
                                    return Ok((tiles[i][j].clone(), row + curr_dist, col - 1 + j));
                                }
                            }
                        }
                    }
                    Direction::Right => {
                        for i in (tiles.len() - 1..=0).rev() {
                            for j in 0..tiles.len() {
                                if tiles[i][j].tile_type == tile_type {
                                    return Ok((tiles[i][j].clone(), row + curr_dist, col - 1 + j));
                                }
                            }
                        }
                    }
                },
                Err(err) => {
                    return Err(err);
                }
            }

            curr_dist += 3;
        }
        return Err(LibError::NoContent);
    }
}

impl Tools for Tool {}

pub struct MyRobot(Robot);

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        where_am_i(self, world);
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
