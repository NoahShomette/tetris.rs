use bevy::math::vec3;
use bevy::prelude::*;
use bevy::reflect::List;
use std::collections::HashMap;
use std::option::Option;
use std::process::id;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoardData>()
            .insert_resource(BlockId {highest_block_id: 1 })
            .add_startup_system(setup_board)
            //.add_startup_system(fill_board_test) //For testing the board to make sure it fills and is right
            ;
    }
}

//sizes
const BOARD_WIDTH_PIXELS: f32 = PIECE_SIZE_PIXEL * BOARD_WIDTH as f32; // total width of board is 360 - 20 for left and right wall bring it to 320
const BOARD_HEIGHT_PIXELS: f32 = PIECE_SIZE_PIXEL * BOARD_HEIGHT as f32;

const WALL_SIZE_PIXEL: f32 = 20.;

const PIECE_SIZE_PIXEL: f32 = 32.;

const BOARD_WIDTH: i32 = 10;
const BOARD_HEIGHT: i32 = 30;
const BOARD_GAMEPLAY_HEIGHT: i32 = 20;

const BLOCK_LIGHT_BLUE_SPRITE: &str = "LightBlueBlock.png";
const BLOCK_BLUE_SPRITE: &str = "BlueBlock.png";
const BLOCK_ORANGE_SPRITE: &str = "OrangeBlock.png";
const BLOCK_YELLOW_SPRITE: &str = "YellowBlock.png";
const BLOCK_GREEN_SPRITE: &str = "GreenBlock.png";
const BLOCK_PURPLE_SPRITE: &str = "PurpleBlock.png";
const BLOCK_RED_SPRITE: &str = "RedBlock.png";

const WALL_COLOR: Color = Color::rgb(1., 1., 1.);

//data structure stuff

pub struct BlockId {
    highest_block_id: u64,
}

impl FromWorld for BlockId {
    fn from_world(world: &mut World) -> Self {
        BlockId {
            highest_block_id: 1,
        }
    }
}

//represents a set of blocks in a piece
struct Piece {
    rotation: PieceRotation,
}

enum PieceRotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

//a point on the grid
struct BoardPoint {
    is_full: bool,
    coordinates: IVec2,
    entity_in_point: Option<Entity>,
    id: u64,
}

pub struct BoardData {
    board_points: HashMap<IVec2, BoardPoint>,
    pieces_in_game: Vec<Piece>,
}

impl FromWorld for BoardData {
    fn from_world(world: &mut World) -> Self {
        BoardData {
            board_points: HashMap::new(),
            pieces_in_game: vec![],
        }
    }
}

impl Piece {
    fn new(
        piece_type: PieceType,
        mut commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        mut board_data: &mut ResMut<BoardData>,
        highest_block_id: &mut ResMut<BlockId>,
    ) -> Piece {
        let piece_coords: Vec<IVec2> = piece_type.get_block_coords_delta();
        let new_piece_id: u64 = highest_block_id.highest_block_id + 1;
        highest_block_id.highest_block_id = new_piece_id;

        let new_piece = Piece {
            rotation: PieceRotation::Zero,
        };

        for piece_coord in piece_coords {
            let spawn_coord_base = piece_type.return_spawn_coord_base();
            let piece_true_coord = IVec2 {
                x: piece_coord.x + spawn_coord_base.x,
                y: piece_coord.y + spawn_coord_base.y,
            };
            //info!(piece_true_coord.x, piece_true_coord.y);
            let entity_commands = commands.spawn_bundle(BlockBundle::new(
                BoardPointCoordinates {
                    coordinates: piece_true_coord,
                },
                piece_type.get_block_color(),
                new_piece_id,
                &asset_server,
            ));

            let entity = entity_commands.id();

            if let Some(mut board_point) = board_data.board_points.get_mut(&piece_true_coord) {
                board_point.entity_in_point = Option::from(entity);
                board_point.is_full = true;
                board_point.id = new_piece_id;
            }
        }

        new_piece
    }
}

#[derive(Component)]
pub struct CurrentPlayerControlled {}

#[derive(Component)]
pub struct BlockID {
    id: u64,
}

//board point components
#[derive(Component)]
pub struct BoardPointCoordinates {
    coordinates: IVec2,
}

impl BoardPointCoordinates {
    fn world_position(&self) -> Vec3 {
        vec3(
            (self.coordinates.x as f32 * PIECE_SIZE_PIXEL as f32)
                - ((BOARD_WIDTH_PIXELS as f32 - PIECE_SIZE_PIXEL as f32) / 2.),
            (self.coordinates.y as f32 * PIECE_SIZE_PIXEL as f32)
                - ((BOARD_HEIGHT_PIXELS as f32
                    - (PIECE_SIZE_PIXEL as f32 * 10.)
                    - PIECE_SIZE_PIXEL)
                    / 2.),
            0.,
        )
    }
}

#[derive(Component)]
pub struct BoardPointWorldPosition {
    coordinates: Vec3,
}

//piece components
#[derive(Component, Copy, Clone)]
pub enum PieceType {
    I, // 4x1
    J, // 3 tall with a two base, facing left like a j
    L, //
    O,
    S,
    T,
    Z,
}

impl PieceType {
    fn get_block_color(&self) -> PieceColor {
        let mut piece_color: PieceColor;
        match self {
            PieceType::I => {
                piece_color = PieceColor::LightBlue;
            }
            PieceType::J => {
                piece_color = PieceColor::Blue;
            }
            PieceType::L => {
                piece_color = PieceColor::Orange;
            }
            PieceType::O => {
                piece_color = PieceColor::Yellow;
            }
            PieceType::S => {
                piece_color = PieceColor::Green;
            }
            PieceType::T => {
                piece_color = PieceColor::Purple;
            }
            PieceType::Z => {
                piece_color = PieceColor::Red;
            }
        }

        piece_color
    }

    fn get_block_coords_delta(&self) -> Vec<IVec2> {
        let mut block_coords_delta: Vec<IVec2> = vec![];
        match self {
            PieceType::I => {
                block_coords_delta.push(IVec2 { x: 0, y: 0 });
                block_coords_delta.push(IVec2 { x: 1, y: 0 });
                block_coords_delta.push(IVec2 { x: 2, y: 0 });
                block_coords_delta.push(IVec2 { x: 3, y: 0 });
            }
            PieceType::J => {
                block_coords_delta.push(IVec2 { x: 0, y: 0 });
                block_coords_delta.push(IVec2 { x: 0, y: -1 });
                block_coords_delta.push(IVec2 { x: 1, y: -1 });
                block_coords_delta.push(IVec2 { x: 2, y: -1 });
            }
            PieceType::L => {
                block_coords_delta.push(IVec2 { x: 0, y: 0 });
                block_coords_delta.push(IVec2 { x: 1, y: 0 });
                block_coords_delta.push(IVec2 { x: 2, y: 0 });
                block_coords_delta.push(IVec2 { x: 2, y: 1 });
            }
            PieceType::O => {
                block_coords_delta.push(IVec2 { x: 0, y: 0 });
                block_coords_delta.push(IVec2 { x: 1, y: 0 });
                block_coords_delta.push(IVec2 { x: 1, y: 1 });
                block_coords_delta.push(IVec2 { x: 0, y: 1 });
            }
            PieceType::S => {
                block_coords_delta.push(IVec2 { x: 0, y: 0 });
                block_coords_delta.push(IVec2 { x: 1, y: 0 });
                block_coords_delta.push(IVec2 { x: 1, y: 1 });
                block_coords_delta.push(IVec2 { x: 2, y: 1 });
            }
            PieceType::T => {
                block_coords_delta.push(IVec2 { x: 0, y: 0 });
                block_coords_delta.push(IVec2 { x: 1, y: 0 });
                block_coords_delta.push(IVec2 { x: 2, y: 0 });
                block_coords_delta.push(IVec2 { x: 1, y: 1 });
            }
            PieceType::Z => {
                block_coords_delta.push(IVec2 { x: 0, y: 0 });
                block_coords_delta.push(IVec2 { x: 1, y: 0 });
                block_coords_delta.push(IVec2 { x: 1, y: -1 });
                block_coords_delta.push(IVec2 { x: 2, y: -1 });
            }
        }

        block_coords_delta
    }

    fn return_spawn_coord_base(&self) -> IVec2 {
        let block_base_coord: IVec2;
        match self {
            PieceType::I => {
                block_base_coord = (IVec2 { x: 3, y: 21 });
            }
            PieceType::J => {
                block_base_coord = (IVec2 { x: 3, y: 22 });
            }
            PieceType::L => {
                block_base_coord = (IVec2 { x: 3, y: 21 });
            }
            PieceType::O => {
                block_base_coord = (IVec2 { x: 4, y: 21 });
            }
            PieceType::S => {
                block_base_coord = (IVec2 { x: 3, y: 21 });
            }
            PieceType::T => {
                block_base_coord = (IVec2 { x: 3, y: 21 });
            }
            PieceType::Z => {
                block_base_coord = (IVec2 { x: 3, y: 22 });
            }
        }

        block_base_coord
    }
}

#[derive(Component)]
enum PieceColor {
    LightBlue,
    Blue,
    Orange,
    Yellow,
    Green,
    Purple,
    Red,
}

impl PieceColor {
    fn return_texture_path(&self) -> &str {
        match self {
            PieceColor::LightBlue => BLOCK_LIGHT_BLUE_SPRITE,
            PieceColor::Blue => BLOCK_BLUE_SPRITE,
            PieceColor::Orange => BLOCK_ORANGE_SPRITE,
            PieceColor::Yellow => BLOCK_YELLOW_SPRITE,
            PieceColor::Green => BLOCK_GREEN_SPRITE,
            PieceColor::Purple => BLOCK_PURPLE_SPRITE,
            PieceColor::Red => BLOCK_RED_SPRITE,
        }
    }
}

#[derive(Bundle)]
struct BlockBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    piece_coordinates: BoardPointCoordinates,
    id: BlockID,
    piece_color: PieceColor,
    player_controlled: CurrentPlayerControlled,
}

impl BlockBundle {
    fn new(
        location: BoardPointCoordinates,
        color: PieceColor,
        id: u64,
        asset_server: &Res<AssetServer>,
    ) -> BlockBundle {
        BlockBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Default::default(),
                    flip_x: false,
                    flip_y: false,
                    custom_size: None,
                    anchor: Default::default(),
                },
                transform: Transform {
                    translation: location.world_position(),
                    ..default()
                },
                texture: asset_server.load(color.return_texture_path()),
                ..default()
            },
            piece_coordinates: location,
            id: BlockID { id },
            piece_color: color,
            player_controlled: CurrentPlayerControlled {},
        }
    }
}

//wall stuff
#[derive(Bundle)]
struct WallBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    board_wall_position: BoardWallPosition,
}

#[derive(Component)]
enum BoardWallPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl BoardWallPosition {
    fn position(&self) -> Vec3 {
        match self {
            BoardWallPosition::Top => Vec3 {
                x: 0.,
                y: (BOARD_HEIGHT_PIXELS / 2.) + (WALL_SIZE_PIXEL / 2.) + 1.,
                z: 0.,
            },
            BoardWallPosition::Bottom => Vec3 {
                x: 0.,
                y: -(((BOARD_HEIGHT_PIXELS - (PIECE_SIZE_PIXEL as f32 * 10.)) / 2.)
                    + (WALL_SIZE_PIXEL / 2.)
                    + 1.),
                z: 0.,
            },
            BoardWallPosition::Left => Vec3 {
                x: -((BOARD_WIDTH_PIXELS / 2.) + (WALL_SIZE_PIXEL / 2.) + 1.),
                y: 0.,
                z: 0.,
            },
            BoardWallPosition::Right => Vec3 {
                x: ((BOARD_WIDTH_PIXELS / 2.) + (WALL_SIZE_PIXEL / 2.) + 1.),
                y: 0.,
                z: 0.,
            },
        }
    }

    fn scale(&self) -> Vec3 {
        match self {
            BoardWallPosition::Top => Vec3 {
                x: BOARD_WIDTH_PIXELS + (WALL_SIZE_PIXEL * 2.) + 2.,
                y: WALL_SIZE_PIXEL,
                z: 1.,
            },
            BoardWallPosition::Bottom => Vec3 {
                x: BOARD_WIDTH_PIXELS + (WALL_SIZE_PIXEL * 2.) + 2.,
                y: WALL_SIZE_PIXEL,
                z: 1.,
            },
            BoardWallPosition::Left => Vec3 {
                x: WALL_SIZE_PIXEL,
                y: BOARD_HEIGHT_PIXELS + 2. - (PIECE_SIZE_PIXEL as f32 * 10.),
                z: 1.,
            },
            BoardWallPosition::Right => Vec3 {
                x: WALL_SIZE_PIXEL,
                y: BOARD_HEIGHT_PIXELS + 2. - (PIECE_SIZE_PIXEL as f32 * 10.),
                z: 1.,
            },
        }
    }
}

impl WallBundle {
    fn new(wall_position: BoardWallPosition) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                transform: Transform {
                    translation: wall_position.position(),
                    scale: wall_position.scale(),
                    ..default()
                },
                ..default()
            },
            board_wall_position: wall_position,
        }
    }
}

pub fn setup_board(mut commands: Commands, mut board_data: ResMut<BoardData>) {
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            board_data.board_points.insert(
                IVec2 { x, y },
                BoardPoint {
                    is_full: false,
                    coordinates: IVec2 { x, y },
                    id: 0,
                    entity_in_point: None,
                },
            );
        }
    }

    //commands.spawn_bundle(WallBundle::new(BoardWallPosition::Top));
    commands.spawn_bundle(WallBundle::new(BoardWallPosition::Bottom));
    commands.spawn_bundle(WallBundle::new(BoardWallPosition::Left));
    commands.spawn_bundle(WallBundle::new(BoardWallPosition::Right));
}

pub fn spawn_new_block(
    commands: &mut Commands,
    board_data: &mut ResMut<BoardData>,
    piece_type: PieceType,
    asset_server: &Res<AssetServer>,
    mut highest_block_id: &mut ResMut<BlockId>,
) {
    Piece::new(
        piece_type,
        commands,
        asset_server,
        board_data,
        highest_block_id,
    );
}

pub fn move_all_pieces(
    mut board_data: &mut ResMut<BoardData>,
    mut commands: &mut Commands,
    mut blocks_query: &mut Query<(
        Entity,
        &BlockID,
        &mut BoardPointCoordinates,
        &mut Transform,
        Option<&CurrentPlayerControlled>,
    )>,
) -> bool {
    let hashmap_of_moves = check_all_pieces_move_validity(&board_data, &blocks_query);

    let mut something_moved: bool = false;

    for (entity, id, mut coords, mut transform, player_controlled_block) in blocks_query.iter_mut()
    {
        let &id_can_move = hashmap_of_moves.get(&id.id).unwrap();
        //info!(id_can_move);
        info!("{}: block can move = {}", id.id, id_can_move);

        if id_can_move == true {
            let point = board_data
                .board_points
                .get_mut(&coords.coordinates)
                .unwrap();

            point.is_full = false;
            point.id = 0;
            point.entity_in_point = None;

            let new_point = board_data
                .board_points
                .get_mut(&IVec2 {
                    x: coords.coordinates.x,
                    y: coords.coordinates.y - 1,
                })
                .unwrap();

            new_point.is_full = true;
            new_point.id = id.id;
            new_point.entity_in_point = Option::from(entity);

            coords.coordinates = IVec2 {
                x: coords.coordinates.x,
                y: coords.coordinates.y - 1,
            };

            transform.translation = coords.world_position();

            something_moved = true;
        } else if let Some(player_controlled_block) = player_controlled_block {
            commands.entity(entity).remove::<CurrentPlayerControlled>();
        }
    }
    //info!("something moved: {}", something_moved);
    something_moved
}

fn check_all_pieces_move_validity(
    board_data: &ResMut<BoardData>,
    blocks_query: &Query<(
        Entity,
        &BlockID,
        &mut BoardPointCoordinates,
        &mut Transform,
        Option<&CurrentPlayerControlled>,
    )>,
) -> HashMap<u64, bool> {
    let mut hashmap = HashMap::new();

    for (_entity, id, coords, _transform, _player_controlled_block) in blocks_query.iter() {
        if let Some(point) = board_data.board_points.get(&IVec2 {
            x: coords.coordinates.x,
            y: coords.coordinates.y - 1,
        }) {
            if point.is_full && point.id > 0 && point.id != id.id {
                hashmap.insert(id.id, false);
            } else if hashmap.contains_key(&id.id) == false {
                hashmap.insert(id.id, true);
            }
        } else {
            hashmap.insert(id.id, false);
        }
    }
    hashmap
}

pub fn move_piece_horizontal(
    mut board_data: &mut ResMut<'_, BoardData>,
    mut commands: &Commands,
    mut blocks_query: &mut Query<
        (Entity, &BlockID, &mut BoardPointCoordinates, &mut Transform),
        With<CurrentPlayerControlled>,
    >,
    direction: IVec2,
) {
    let move_valid = check_individual_piece_move(board_data, blocks_query, direction);
    for (entity, id, mut coords, mut transform) in blocks_query.iter_mut() {
        if move_valid == true {
            let point = board_data
                .board_points
                .get_mut(&IVec2 {
                    x: coords.coordinates.x,
                    y: coords.coordinates.y,
                })
                .unwrap();

            point.is_full = false;
            point.id = 0;
            point.entity_in_point = None;

            let new_point = board_data
                .board_points
                .get_mut(&IVec2 {
                    x: coords.coordinates.x + direction.x,
                    y: coords.coordinates.y + direction.y,
                })
                .unwrap();

            new_point.is_full = true;
            new_point.id = id.id;
            new_point.entity_in_point = Option::from(entity);

            *coords = BoardPointCoordinates {
                coordinates: IVec2 {
                    x: coords.coordinates.x + direction.x,
                    y: coords.coordinates.y + direction.y,
                },
            };
            transform.translation = coords.world_position();
        }
    }
}

fn check_individual_piece_move(
    board_data: &ResMut<BoardData>,
    blocks_query: &mut Query<
        (Entity, &BlockID, &mut BoardPointCoordinates, &mut Transform),
        With<CurrentPlayerControlled>,
    >,
    direction: IVec2,
) -> bool {
    let mut move_valid: bool = true;

    for (_entity, id, coords, _transform) in blocks_query.iter() {
        if let Some(point) = board_data.board_points.get(&IVec2 {
            x: coords.coordinates.x + direction.x,
            y: coords.coordinates.y + direction.y,
        }) {
            if point.is_full && point.id > 0 && point.id != id.id {
                move_valid = false;
            } else if move_valid == false {
            } else {
                move_valid = true;
            }
        } else {
            move_valid = false;
        }
    }
    move_valid
}
