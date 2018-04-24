extern crate adapter;

use adapter::MediaPlayer;
use adapter::AudioPlayer;

fn main() {
    let mut audio_player = AudioPlayer::new();

    audio_player.play("mp3", "beyond_the_horizon.mp3");
    audio_player.play("mp4", "alone.mp4");
    audio_player.play("vlc", "far_far_away.vlc");
    audio_player.play("avi", "mind_me.avi");
}
