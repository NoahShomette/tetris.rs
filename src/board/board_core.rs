use bevy::prelude::*;
use std::collections::HashMap;
use bevy::math::ivec2;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(early_setup)
        .add_startup_system(setup_board);
        
        app.add_system(test_spawn_block); //For testing the board to make sure it fills and is right
    }
}

//sizes
const BOARD_WIDTH_PIXELS: f32 = PIECE_SIZE_PIXEL * BOARD_WIDTH as f32; // total width of board is 360 - 20 for left and right wall bring it to 320
const BOARD_HEIGHT_PIXELS: f32 = PIECE_SIZE_PIXEL * BOARD_HEIGHT as f32;

const WALL_SIZE_PIXEL: f32 = 20.;

const PIECE_SIZE_PIXEL: f32 = 32.;

const BOARD_WIDTH: i32 = 10;
const BOARD_HEIGHT: i32 = 22;

const PIECE_LIGHT_BLUE_COLOR: &str = "LightBlueBlock.png";
const PIECE_BLUE_COLOR: &str = "BlueBlock.png";
const PIECE_ORANGE_COLOR: &str = "OrangeBlock.png";
const PIECE_YELLOW_COLOR: &str = "YellowBlock.png";
const PIECE_GREEN_COLOR: &str = "GreenBlock.png";
const PIECE_PURPLE_COLOR: &str = "PurpleBlock.png";
const PIECE_RED_COLOR: &str = "RedBlock.png";

const WALL_COLOR: Color = Color::rgb(1., 1., 1.);

#[derive(Component)]
pub struct BoardData {
    board_points: HashMap<IVec2, BoardPointBundle>,
    pieces_by_coords: HashMap<IVec2, PieceBundle>,
}

//board point components
#[derive(Component)]
pub struct BoardPointCoordinates {
    coordinates: IVec2,
}

#[derive(Component)]
pub struct BoardPointWorldPosition {
    coordinates: Vec3,
}

#[derive(Component)]
pub enum BoardPointStatus {
    Filled,
    Empty,
}

#[derive(Bundle)]
struct BoardPointBundle {
    board_point_coordinates: BoardPointCoordinates,
    board_point_status: BoardPointStatus,
    board_point_world_coordinates: BoardPointWorldPosition,
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
                y: -((BOARD_HEIGHT_PIXELS / 2.) + (WALL_SIZE_PIXEL / 2.) + 1.),
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
                y: BOARD_HEIGHT_PIXELS + 2.,
                z: 1.,
            },
            BoardWallPosition::Right => Vec3 {
                x: WALL_SIZE_PIXEL,
                y: BOARD_HEIGHT_PIXELS + 2.,
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

//piece components
#[derive(Component)]
enum PieceType {}

#[derive(Component)]
struct PieceColor {
    piece_color: Color,
}

#[derive(Bundle)]
struct PieceBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    piece_coordinates: BoardPointCoordinates,
    piece_color: PieceColor,
}

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
}

pub fn early_setup(mut commands: Commands){
    commands.spawn().insert(BoardData {
        board_points: HashMap::new(),
        pieces_by_coords: HashMap::new(),
    });
}

pub fn setup_board(mut commands: Commands/*, mut board_data_query : Query<&mut BoardData>*/) {

    //let (board_data) = board_data_query.single();

    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            commands.spawn_bundle(BoardPointBundle::new(IVec2 { x, y }));
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
    mut board_data_query : Query<(Entity, &BoardPointWorldPosition, &BoardPointCoordinates, &mut BoardPointStatus)>){
    for (entity, board_world_coords, board_point_coordinates, board_point_status) in board_data_query.iter_mut() {
        
        if(board_point_coordinates.coordinates == ivec2(9,5)){
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
                texture: asset_server.load(PIECE_RED_COLOR),
                ..default()
            });
        }
        

    }
    
}

fn fill_board_test(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    point_query: Query<(&BoardPointCoordinates, &BoardPointWorldPosition)>,
) {
    for (board_points, board_world_coords) in point_query.iter() {
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
            texture: asset_server.load(PIECE_RED_COLOR),
            ..default()
        });
    }
}
