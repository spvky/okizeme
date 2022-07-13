pub use bevy::prelude::*;
pub use okizeme_types::Hitstop;

pub fn oki_animation_player(
    mut animation_players: Query<&mut AnimationPlayer,Without<Hitstop>>,
) {
    for  mut player in animation_players.iter_mut() {
        let elapsed = player.elapsed();
        player.set_elapsed( elapsed + (1./60.));
    }
}
