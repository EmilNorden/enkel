use std::time::Duration;

#[derive(Copy, Clone)]
pub struct GameTime {
    game_duration: Duration,
    frame_duration: Duration,
    frame_millis: f32,
}

impl GameTime {
    pub fn new(game_duration: Duration, frame_duration: Duration) -> Self {
        let frame_millis: f32 = frame_duration.as_micros() as f32;
        GameTime {
            game_duration,
            frame_duration,
            frame_millis,
        }
    }
}

impl GameTime {
    pub fn game_duration(&self) -> Duration {
        self.game_duration
    }

    pub fn frame_duration(&self) -> Duration {
        self.frame_duration
    }

    pub fn frame_millis(&self) -> f32 {
        self.frame_millis
    }
}
