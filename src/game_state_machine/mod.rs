use bevy::prelude::*;

//controls game state from menu or playing

//controls the actual flow of the playing game. when blocks fall, when the rows are checked, etc
#[derive(Debug, PartialEq, Eq)]
pub enum GameFlow {
    Menu,
    PlayerMovingBlock,
    CheckingRows,
    BlocksMovingAfterRowBreak,
}

impl FromWorld for GameFlow {
    fn from_world(world: &mut World) -> Self {
        GameFlow::Menu
    }
}

//controls the state of the game when its playing
#[derive(Debug, PartialEq, Eq)]
pub enum GamePlayState {
    Menu,
    Win,
    Lose,
    Playing,
}

pub struct GameStateInfo {
    pub(crate) game_state: GamePlayState,
    pub(crate) game_flow_state: GameFlow,
}

impl GameStateInfo {
    pub(crate) fn change_game_play_state(
        &mut self,
        play_state: GamePlayState,
        mut event_writer: EventWriter<GamePlayState>,
    ) {
        match self.game_state {
            GamePlayState::Menu => {
                match play_state {
                    GamePlayState::Menu => {} //nothing
                    GamePlayState::Win => {}  //nothing shouldnt be able to go here
                    GamePlayState::Lose => {} //nothing shouldnt be able to go here
                    GamePlayState::Playing => {
                        info!("started game");
                        self.game_state = GamePlayState::Playing;
                        event_writer.send(GamePlayState::Playing)
                    } //starts the game
                }
            }
            GamePlayState::Win => {
                match play_state {
                    GamePlayState::Menu => {
                        self.game_state = GamePlayState::Menu;
                        event_writer.send(GamePlayState::Menu)
                    } //go to main menu
                    GamePlayState::Win => {}  //nothing
                    GamePlayState::Lose => {} //nothing
                    GamePlayState::Playing => {
                        self.game_state = GamePlayState::Playing;
                        event_writer.send(GamePlayState::Playing)
                    } // restart game
                }
            }
            GamePlayState::Lose => {
                match play_state {
                    GamePlayState::Menu => {
                        self.game_state = GamePlayState::Menu;
                        event_writer.send(GamePlayState::Menu)
                    } //go to main menu
                    GamePlayState::Win => {}  //nothing
                    GamePlayState::Lose => {} //nothing
                    GamePlayState::Playing => {
                        self.game_state = GamePlayState::Playing;
                        event_writer.send(GamePlayState::Playing)
                    } //restart game
                }
            }
            GamePlayState::Playing => {
                match play_state {
                    GamePlayState::Menu => {
                        self.game_state = GamePlayState::Menu;
                        event_writer.send(GamePlayState::Menu)
                    } //end game and go to main menu
                    GamePlayState::Win => {
                        self.game_state = GamePlayState::Win;
                        event_writer.send(GamePlayState::Win)
                    } //game done and show win screen
                    GamePlayState::Lose => {
                        self.game_state = GamePlayState::Lose;
                        event_writer.send(GamePlayState::Lose)
                    } //game done and show lose screen
                    GamePlayState::Playing => {
                        self.game_state = GamePlayState::Playing;
                        event_writer.send(GamePlayState::Playing)
                    } // restart game
                }
            }
        }
    }

    fn change_flow_state(&self, flow_state: GameFlow) {
        match self.game_flow_state {
            GameFlow::Menu => match flow_state {
                GameFlow::Menu => {}              //nothing
                GameFlow::PlayerMovingBlock => {} // game is starting handle this until block is done then change state to checking rows
                GameFlow::CheckingRows => {}
                GameFlow::BlocksMovingAfterRowBreak => {} // move blocks until none can move then go back to checking rows
            },
            GameFlow::PlayerMovingBlock => match flow_state {
                GameFlow::Menu => {}
                GameFlow::PlayerMovingBlock => {}
                GameFlow::CheckingRows => {} //check rows to see if any are filled. if so break them then send to blocks moving
                GameFlow::BlocksMovingAfterRowBreak => {}
            },
            GameFlow::CheckingRows => match flow_state {
                GameFlow::Menu => {}
                GameFlow::PlayerMovingBlock => {}
                GameFlow::CheckingRows => {}
                GameFlow::BlocksMovingAfterRowBreak => {}
            },
            GameFlow::BlocksMovingAfterRowBreak => match flow_state {
                GameFlow::Menu => {}
                GameFlow::PlayerMovingBlock => {}
                GameFlow::CheckingRows => {}
                GameFlow::BlocksMovingAfterRowBreak => {}
            },
        }
    }
}

//bevy thing to allow it to be used as a resource
impl FromWorld for GameStateInfo {
    fn from_world(world: &mut World) -> Self {
        GameStateInfo {
            game_state: GamePlayState::Menu,
            game_flow_state: GameFlow::Menu,
        }
    }
}
