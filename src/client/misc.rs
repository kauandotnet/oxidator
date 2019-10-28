use super::client::*;
use crate::*;

impl App {
    pub fn clear_from_play(&mut self) {
        self.game_state.players.clear();
        self.game_state.my_player_id = None;
        self.game_state.kbots.clear();
        self.game_state.selected.clear();
        self.game_state.kinematic_projectiles.clear();
        self.game_state.in_screen.clear();
        self.kbot_gpu.update_instance_dirty(&[], &self.gpu.device);
        self.health_bar.update_instance(&[], &self.gpu.device);
        self.kinematic_projectile_gpu
            .update_instance_dirty(&[], &self.gpu.device);
    }
}