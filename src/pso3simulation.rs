


use action::Action;
use player::{Player, PlayerId};
use deck::Deck;
use field::Field;
use boardstate::BoardState;
//use phase;
//use phase::phase::Phase;
use phase::phase::Phase;
use phase::gamestart::GameStart;
use error::SimulationError;
use statechange::StateChange;



pub struct PSO3State {
    boardstate: BoardState,
    pub active_player: Option<PlayerId>,
    player1: Player,
    player2: Player,
}


pub struct PSO3Simulation {
    phase: Box<Phase>,
    state: PSO3State,
}





impl PSO3Simulation {
    pub fn new(field: Field, p1deck: Deck, p2deck: Deck) -> PSO3Simulation {
        PSO3Simulation {
            phase: Box::new(GameStart::new()),
            state: PSO3State {
                boardstate: BoardState::new(),
                active_player: None,
                player1: Player::new(p1deck),
                player2: Player::new(p2deck),
            }
        }
    }

    pub fn apply_action(&mut self, action: Action) -> Result<Vec<StateChange>, SimulationError> {
        //self.phase.action
        /*let (phase, statechange) = match self.phase {
            Phase::GameStart => phase::gamestart::game_start(self, action)?,


            _ => panic!("unknown phase!")
        };*/
        //let (phase, statechange) =
        let (mut statechange, newphase) = self.phase.handle_action(&mut self.state, action)?;
        if let Some(phase) = newphase {
            self.phase = phase;
            statechange.push(StateChange::PhaseChange(self.phase.phase_type()))
        }
        //Ok(statechange)
        Ok(statechange)
    }
    
}