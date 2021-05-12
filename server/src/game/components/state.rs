use crate::game::components::movement::{Location, Transformation};
use crate::game::location::pos::Position;
use crate::net::data::PlayerInputAction;
use crate::net::packet::state_delta::{ObjectStateBatch, ObjectStateChange};
use std::sync::atomic::AtomicPtr;
use std::time::Duration;

type BoxedState<T> = Box<dyn State<T> + Sync + Send>;

pub struct StateData {}

pub struct MovableStateData {
    pub transformation: AtomicPtr<Transformation>,
    pub location: AtomicPtr<Location>,
    pub delta: Duration,
    pub action: Option<PlayerInputAction>,
    pub state_delta: Option<ObjectStateBatch>,
}

impl<'a> MovableStateData {
    pub fn new(
        transformation: Option<&'a mut Transformation>,
        location: Option<&'a mut Location>,
        delta: Duration,
        action: Option<PlayerInputAction>,
    ) -> Self {
        MovableStateData {
            transformation: transformation.map_or(AtomicPtr::default(), |t| AtomicPtr::new(t)),
            location: location.map_or(AtomicPtr::default(), |l| AtomicPtr::new(l)),
            delta,
            action,
            state_delta: Some(ObjectStateBatch::new()),
        }
    }
}

pub trait State<T> {
    fn update(&mut self, data: &mut T) -> Option<Box<dyn State<T> + Sync + Send>>;
    fn on_start(&mut self);
    fn on_stop(&mut self);
}

struct IdleState;

impl State<MovableStateData> for IdleState {
    fn update(&mut self, data: &mut MovableStateData) -> Option<BoxedState<MovableStateData>> {
        match data.action {
            Some(PlayerInputAction::MoveForward) => {
                unsafe {
                    let mut transformation = &mut (*(*data.transformation.get_mut()));
                    data.state_delta.as_mut().unwrap().add(ObjectStateChange::Speed(transformation.speed));
                }
                Some(Box::new(MoveState {})) },
            _ => None,
        }
    }

    fn on_start(&mut self) {
        debug!("STARTED IDLE");
    }

    fn on_stop(&mut self) {
        debug!("STOPPED IDLE");
    }
}

struct MoveState;

impl State<MovableStateData> for MoveState {
    fn update(&mut self, data: &mut MovableStateData) -> Option<BoxedState<MovableStateData>> {
        info!("Move state update");
        match data.action {
            Some(PlayerInputAction::StopMove) => {
                data.state_delta.as_mut().unwrap().add(ObjectStateChange::Speed(0.0));
                return Some(Box::new(IdleState));
            }
            _ => (),
        };
        unsafe {
            let mut location = &mut (*(*data.location.get_mut()));
            let mut transformation = &mut (*(*data.transformation.get_mut()));
            let speed = transformation.speed;
            let angle = transformation.facing.get_facing();
            let delta: Duration = data.delta;
            let calculated_speed = speed * delta.as_secs() as f32;

            let vx = calculated_speed * angle.cos();
            let vy = calculated_speed * angle.sin();

            let new_position = Position::from_coord(
                location.position.x() + vx as f64,
                location.position.y() + vy as f64,
            );

            debug!("Position update to {:#?}", &new_position);
            location.position = new_position;
            data.state_delta.as_mut().unwrap().add(ObjectStateChange::Position(new_position));
        }

        None
    }

    fn on_start(&mut self) {
        info!("STARTED MOVE");
    }

    fn on_stop(&mut self) {
        info!("STOPPED MOVE");
    }
}

pub struct StateMachineComponent<T>
where
    T: 'static,
{
    state: Box<dyn State<T> + Send + Sync>,
}

impl StateMachineComponent<MovableStateData> {
    pub fn new() -> Self {
        StateMachineComponent {
            state: Box::new(IdleState {}),
        }
    }

    pub fn update(&mut self, data: &mut MovableStateData) {
        let new_state = self.state.update(data);
        if let Some(s) = new_state {
            self.state.on_stop();
            self.state = s;
            self.state.on_start();
        }
    }
}
