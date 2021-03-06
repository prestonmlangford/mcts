use super::MCTS;
use std::time::Duration;


impl MCTS {
    ///Call this method to instantiate a new search with default parameters.
    pub fn new() -> Self {
        Self {
            time: Duration::new(1, 0),
            exploration: (2.0 as f32).sqrt(),
            expansion: 10,
            use_custom_evaluation: false,
        }
    }
    
    ///Sets the time parameter.
    pub fn with_time(mut self, time: Duration) -> Self {
        self.time = time;
        self
    }
    
    ///Sets the exploration parameter.
    pub fn with_exploration(mut self, exploration: f32) -> Self {
        assert!(exploration > 0.0,"A positive value is required for the exploration constant.");
        self.exploration = exploration;
        self
    }

    ///Sets the expansion parameter
    pub fn with_expansion_minimum(mut self, expansion: u32) -> Self {
        assert!(expansion > 0,"The value for expansion minimum must be greater than zero.");
        self.expansion = expansion;
        self
    }

    ///Enables the custom evaluation method.
    pub fn with_custom_evaluation(mut self) -> Self {
        self.use_custom_evaluation = true;
        self
    }
}