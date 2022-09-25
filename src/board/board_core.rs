use bevy::math::vec3;
use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_board);
        //app.add_system(fill_board_test); //For testing the board to make sure it fills and is right
    }
}

//sizes
const BOARD_WIDTH_PIXELS :f32 = PIECE_SIZE_PIXEL * BOARD_WIDTH as f32; // total width of board is 360 - 20 for left and right wall bring it to 320
const BOARD_HEIGHT_PIXELS :f32 = PIECE_SIZE_PIXEL * BOARD_HEIGHT as f32;

const WALL_SIZE_PIXEL : f32 = 20.;

const PIECE_SIZE_PIXEL : f32 = 32.;

const BOARD_WIDTH :i32 = 10;
const BOARD_HEIGHT :i32 = 22;

const PIECE_LIGHT_BLUE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PIECE_BLUE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PIECE_ORANGE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PIECE_YELLOW_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PIECE_GREEN_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PIECE_PURPLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PIECE_RED_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

const WALL_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);



//board point components
#[derive(Component)]
struct BoardPointCoordinates {
    coordinates: IVec2,
}

#[derive(Component)]
struct BoardPointWorldPosition {
    coordinates: Vec3,
}

#[derive(Component)]
enum BoardPointStatus {
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
struct WallBundle{
    #[bundle]
    sprite_bundle:SpriteBundle,
    board_wall_position:BoardWallPosition
}

#[derive(Component)]
enum BoardWallPosition{
    Top,
    Bottom,
    Left,
    Right
}

impl BoardWallPosition {
    
    fn position(&self) -> Vec3{
        match self {
            BoardWallPosition::Top => Vec3{x: 0., y: (BOARD_HEIGHT_PIXELS / 2.) + (WALL_SIZE_PIXEL / 2.), z: 0.},
            BoardWallPosition::Bottom => Vec3{x: 0., y: -(BOARD_HEIGHT_PIXELS / 2.) - (WALL_SIZE_PIXEL / 2.), z: 0.},
            BoardWallPosition::Left => Vec3{x: -(BOARD_WIDTH_PIXELS / 2.) - (WALL_SIZE_PIXEL / 2.), y: 0., z: 0.},
            BoardWallPosition::Right => Vec3{x: (BOARD_WIDTH_PIXELS / 2.) + (WALL_SIZE_PIXEL / 2.), y: 0., z: 0.},
        }
    }
    
    fn scale(&self) -> Vec3{
        match self {
            BoardWallPosition::Top => Vec3{x: BOARD_WIDTH_PIXELS + (WALL_SIZE_PIXEL * 2.), y: WALL_SIZE_PIXEL, z: 1.},
            BoardWallPosition::Bottom => Vec3{x: BOARD_WIDTH_PIXELS + (WALL_SIZE_PIXEL * 2.), y: WALL_SIZE_PIXEL, z: 1.},
            BoardWallPosition::Left => Vec3{x: WALL_SIZE_PIXEL, y: BOARD_HEIGHT_PIXELS, z: 1.},
            BoardWallPosition::Right => Vec3{x: WALL_SIZE_PIXEL, y: BOARD_HEIGHT_PIXELS, z: 1.},
        }
    }
}

impl WallBundle{
    
    fn new(wall_position:BoardWallPosition) -> WallBundle{
        WallBundle{
            sprite_bundle: SpriteBundle{
                transform: Transform{
                    translation: wall_position.position(),
                    scale: wall_position.scale(),
                    ..default()
                },
                ..default()
            },
            board_wall_position: wall_position
        }
    }
    
}


//piece components
#[derive(Component)]
enum PieceType{

}

#[derive(Component)]
struct PieceColor{
    piece_color:Color,
}

#[derive(Bundle)]
struct PieceBundle{
    #[bundle]
    sprite_bundle:SpriteBundle,
    piece_coordinates: BoardPointCoordinates,
    piece_color:PieceColor,

}

impl BoardPointBundle {
    fn new(coordinates: IVec2) -> BoardPointBundle {
        BoardPointBundle {
            board_point_coordinates: BoardPointCoordinates {
                coordinates,
            },
            board_point_status: BoardPointStatus::Empty,
            board_point_world_coordinates: BoardPointWorldPosition { 
                coordinates: Vec3{
                    x: (coordinates.x as f32 * PIECE_SIZE_PIXEL as f32) - ((BOARD_WIDTH_PIXELS as f32 - PIECE_SIZE_PIXEL as f32) / 2.), 
                    y : (coordinates.y as f32 * PIECE_SIZE_PIXEL as f32) - ((BOARD_HEIGHT_PIXELS as f32 - PIECE_SIZE_PIXEL as f32) / 2.), 
                    z : 0.} 
            }
        }
    }
}

pub fn setup_board(mut commands : Commands){
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            commands.spawn_bundle(BoardPointBundle::new(IVec2{x, y}));
        }
    }
    
    commands.spawn_bundle(WallBundle::new(BoardWallPosition::Top));
    commands.spawn_bundle(WallBundle::new(BoardWallPosition::Bottom));
    commands.spawn_bundle(WallBundle::new(BoardWallPosition::Left));
    commands.spawn_bundle(WallBundle::new(BoardWallPosition::Right));


}



fn fill_board_test(
    mut commands: Commands,
    point_query : Query<(&BoardPointCoordinates, &BoardPointWorldPosition)>
){
    for (board_points, board_world_coords) in point_query.iter(){
        
        commands.spawn_bundle(
            SpriteBundle { 
                sprite: Sprite{
                    color: PIECE_BLUE_COLOR,
                    ..default()
                }, 
                transform: Transform{
                    translation: board_world_coords.coordinates,
                    scale: Vec3{
                        x: PIECE_SIZE_PIXEL,
                        y: PIECE_SIZE_PIXEL,
                        z: 1.
                    },
                    ..default()
                }, 
                ..default()
            });
    }
}
