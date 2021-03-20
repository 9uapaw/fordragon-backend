use crate::game::components::connection::NetworkConnectionComponent;
use crate::game::components::movement::{Location, Transformation};
use crate::game::location::pos::Position;
use crate::game::resource::new_user::NewUsers;
use specs::{Entities, System, Write, WriteStorage};
use std::borrow::{BorrowMut, Borrow};
use crate::game::location::facing::Facing;
use crate::game::components::state::{StateMachineComponent, MovableStateData};

pub struct NewUserHandlerSystem;

impl<'a> System<'a> for NewUserHandlerSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, NewUsers>,
        WriteStorage<'a, Location>,
        WriteStorage<'a, Transformation>,
        WriteStorage<'a, NetworkConnectionComponent>,
        WriteStorage<'a, StateMachineComponent<MovableStateData>>,
    );

    fn run(
        &mut self,
        (entities, mut new_users, mut loc, mut transform, mut conn): Self::SystemData,
    ) {
        for user in new_users.0.drain(..) {
            info!("Adding new user {}", &user.name);
            let player = entities.create();
            loc.borrow_mut().insert(
                player,
                Location {
                    position: Position::new(),
                },
            );
            transform.borrow_mut().insert(player, Transformation{speed: 1.0, facing: Facing::new()});
            conn.borrow_mut().insert(player, NetworkConnectionComponent::new(user));
        }
    }
}
