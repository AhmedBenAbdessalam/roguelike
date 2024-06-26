mod collision;
mod end_turn;
mod entity_renderer;
mod map_renderer;
mod movement;
mod player_input;
mod random_move;
use crate::prelude::*;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_renderer::map_render_system())
        .add_system(entity_renderer::entity_renderer_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(collision::collisions_system())
        .flush()
        .add_system(map_renderer::map_render_system())
        .add_system(entity_renderer::entity_renderer_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_enemy_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .flush()
        .add_system(collision::collisions_system())
        .flush()
        .add_system(map_renderer::map_render_system())
        .add_system(entity_renderer::entity_renderer_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
