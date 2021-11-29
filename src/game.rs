use crate::game_time::GameTime;
use crate::GameContext;

pub trait Game {
    fn load_content(&mut self, context: &mut GameContext);
    fn update(&mut self, context: &mut GameContext, time: GameTime);
    fn draw(&self, context: &mut GameContext, time: GameTime);
}