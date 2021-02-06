pub trait State<T> {
    fn update(&mut self, data: &mut T) -> Option<Box<dyn State<T> + Send>>;
    fn on_start(&mut self);
    fn on_stop(&mut self);
}

struct IdleState {}

impl<T> State<T> for IdleState {
    fn update(&mut self, data: &mut T) -> Option<Box<dyn State<T> + Send>> {
        Some(Box::new(MoveState {}))
    }

    fn on_start(&mut self) {
        println!("STARTED IDLE");
    }

    fn on_stop(&mut self) {
        println!("STOPPED IDLE");
    }
}

struct MoveState {}

impl<T> State<T> for MoveState {
    fn update(&mut self, data: &mut T) -> Option<Box<dyn State<T> + Send>> {
        None
    }

    fn on_start(&mut self) {
        println!("STARTED MOVE");
    }

    fn on_stop(&mut self) {
        println!("STOPPED MOVE");
    }
}

pub struct StateMachineComponent<T> {
    state: Box<dyn State<T> + Send>,
}

impl<T> StateMachineComponent<T> {
    pub fn new() -> Self {
        StateMachineComponent { state: Box::new(IdleState{}) }
    }

    pub fn update(&mut self, data: &mut T) {
        let new_state = self.state.update(data);
        if let Some(s) = new_state {
            self.state.on_stop();
            self.state = s;
            self.state.on_start();
        }
    }
}
