use bevy::math::{ivec2, vec3};
use bevy::prelude::*;
use std::collections::HashMap;
use std::option::Option;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoardData>()
            .add_startup_system(setup_board)
            //.add_startup_system(fill_board_test)
            ;

        app.add_system(test_spawn_block); //For testing the board to make sure it fills and is right
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

//represents a set of blocks in a piece
struct Piece {
    blocks_in_piece: HashMap<IVec2, Block>,
}
//represents an individual block
struct Block {
    coordinates: IVec2,
    owning_piece: Piece,
    color: PieceColor,
}
//a point on the grid
struct BoardPoint {
    is_full: bool,
    coordinates: IVec2,
    block_in_point: Option<Block>,
    entity_in_point: Option<Entity>,
}

pub struct BoardData {
    board_points: HashMap<IVec2, BoardPoint>,
    pieces_by_coords: HashMap<IVec2, Block>,
    pieces_in_game: Vec<Piece>,
}

impl FromWorld for BoardData {
    fn from_world(world: &mut World) -> Self {
        BoardData {
            board_points: HashMap::new(),
            pieces_by_coords: HashMap::new(),
            pieces_in_game: vec![],
        }
    }
}

impl Piece{
    fn new(piece_type: PieceType, ) -> Piece{
        
        
        
        
        
        Piece{
            blocks_in_piece: HashMap::new(),
        }
    }
    
}

impl Block{
    fn new(owner:Piece, color: PieceColor, coords: IVec2) -> Block{
        Block{
            coordinates: coords,
            owning_piece: owner,
            color
        }
    }
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
#[derive(Component)]
enum PieceType {}

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
struct PieceBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    piece_coordinates: BoardPointCoordinates,
    piece_color: PieceColor,
}

impl PieceBundle {
    fn new(
        location: BoardPointCoordinates,
        color: PieceColor,
        asset_server: &Res<AssetServer>,
    ) -> PieceBundle {
        PieceBundle {
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
            piece_color: color,
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

/*
impl BoardPointBundle {
    fn new(coordinates: IVec2) -> BoardPointBundle {
        BoardPointBundle {
            board_point_coordinates: BoardPointCoordinates { coordinates },
            board_point_status: BoardPointStatus::Empty,
            board_point_world_coordinates: BoardPointWorldPosition {
                coordinates: Vec3 {
                    x: (coordinates.x as f32 * PIECE_SIZE_PIXEL as f32)
                        - ((BOARD_WIDTH_PIXELS as f32 - PIECE_SIZE_PIXEL as f32) / 2.),
                    y: (coordinates.y as f32 * PIECE_SIZE_PIXEL as f32)
                        - ((BOARD_HEIGHT_PIXELS as f32 - PIECE_SIZE_PIXEL as f32) / 2.),
                    z: 0.,
                },
            },
        }
    }
}*/

pub fn setup_board(mut commands: Commands, mut board_data: ResMut<BoardData>) {
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            board_data.board_points.insert(
                IVec2 { x, y },
                BoardPoint {
                    is_full: false,
                    coordinates: IVec2 { x, y },
                    block_in_point: None,
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

pub fn test_spawn_block(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut board_data_query: Query<(Entity, &BoardPointWorldPosition, &BoardPointCoordinates)>,
) {
    for (entity, board_world_coords, board_point_coordinates) in board_data_query.iter_mut() {
        if board_point_coordinates.coordinates == ivec2(9, 5) {
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite::default(),
                transform: Transform {
                    translation: board_world_coords.coordinates,
                    scale: Vec3 {
                        x: 1.,
                        y: 1.,
                        z: 1.,
                    },
                    ..default()
                },
                texture: asset_server.load(BLOCK_RED_SPRITE),
                ..default()
            });
        }
    }
}

fn fill_board_test(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    board_data: Res<BoardData>,
) {
    for y in 0..BOARD_GAMEPLAY_HEIGHT {
        for x in 0..BOARD_WIDTH {
            commands.spawn_bundle(PieceBundle::new(
                BoardPointCoordinates {
                    coordinates: IVec2 { x: x, y: y },
                },
                PieceColor::Red,
                &asset_server,
            ));
        }
    }
}

fn spawn_new_block(board_data: ResMut<BoardData>, location: IVec2) {}

/*
fn get_world_coords_of_board_coords() -> Vec3{

}*/
