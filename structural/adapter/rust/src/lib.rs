pub trait MediaPlayer {
    fn play(&mut self, audio_type: &str, file_name: &str);
}

trait AdvancedMediaPlayer {
    fn play_vlc(&self, file_name: &str);
    fn play_mp4(&self, file_name: &str);
}

struct VlcPlayer;

impl AdvancedMediaPlayer for VlcPlayer {
    fn play_vlc(&self, file_name: &str) {
        println!("Playing vlc file. Name: {}", file_name);
    }
    fn play_mp4(&self, _file_name: &str) {
        // Do nothing
    }
}

struct Mp4Player;

impl AdvancedMediaPlayer for Mp4Player {
    fn play_vlc(&self, _file_name: &str) {
        // Do nothing
    }
    fn play_mp4(&self, file_name: &str) {
        println!("Playing mp4 file. Name: {}", file_name);
    }
}

struct MediaAdapter {
    advanced_music_player: Box<AdvancedMediaPlayer>,
}

impl MediaAdapter {
    fn new(audio_type: &str) -> MediaAdapter {
        match audio_type {
            "vlc" => MediaAdapter {
                advanced_music_player: Box::new(VlcPlayer),
            },
            "mp4" => MediaAdapter {
                advanced_music_player: Box::new(Mp4Player),
            },
            _ => panic!("Non supported audio_type"),
        }
    }
}

impl MediaPlayer for MediaAdapter {
    fn play(&mut self, audio_type: &str, file_name: &str) {
        match audio_type {
            "vlc" => self.advanced_music_player.play_vlc(file_name),
            "mp4" => self.advanced_music_player.play_mp4(file_name),
            _ => println!("Not playing music, unsupported audio_type"),
        };
    }
}

pub struct AudioPlayer {
    media_adapter: Option<MediaAdapter>,
}

impl AudioPlayer {
    pub fn new() -> AudioPlayer {
        AudioPlayer {
            media_adapter: None,
        }
    }
}

impl MediaPlayer for AudioPlayer {
    fn play(&mut self, audio_type: &str, file_name: &str) {
        match audio_type {
            "mp3" => println!("Playing mp3 file. Name: {}", file_name),
            "vlc" | "mp4" => {
                self.media_adapter = Some(MediaAdapter::new(audio_type));
                if let Some(ref mut media_adapter) = self.media_adapter {
                    media_adapter.play(audio_type, file_name);
                }
            },
            _ => println!("Invalid media. {} format not supported", audio_type),
        }
    }
}
