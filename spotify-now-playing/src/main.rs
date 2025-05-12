use windows::{
    Media::Control::{
        GlobalSystemMediaTransportControlsSessionManager as MediaSessionManager,
        GlobalSystemMediaTransportControlsSessionPlaybackStatus as PlaybackStatus,
    },
    Storage::{
        FileIO, StorageFile,
        Streams::{Buffer, InputStreamOptions},
    },
    core::HSTRING,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
enum MediaPlaybackState {
    // Unknown media state
    Unknown = -1,

    // The media is closed.
    Closed = 0,

    // The media is opened.
    Opened,

    // The media is changing.
    Changing,

    // The media is stopped.
    Stopped,

    // The media is playing.
    Playing,

    // The media is paused.
    Paused,
}

impl From<PlaybackStatus> for MediaPlaybackState {
    fn from(value: PlaybackStatus) -> Self {
        match value.0 {
            0 => Self::Closed,
            1 => Self::Opened,
            2 => Self::Changing,
            3 => Self::Stopped,
            4 => Self::Playing,
            5 => Self::Paused,
            _ => Self::Unknown,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let session_manager = MediaSessionManager::RequestAsync()?.get()?;
    let sessions = session_manager.GetSessions()?;

    let spotify = sessions.into_iter().find(|session| {
        session
            .SourceAppUserModelId()
            .is_ok_and(|id| id == "Spotify.exe")
    });

    if let Some(session) = spotify {
        let props = session.TryGetMediaPropertiesAsync()?.get()?;
        let playback_state: MediaPlaybackState =
            session.GetPlaybackInfo()?.PlaybackStatus()?.into();

        let stream = props.Thumbnail()?.OpenReadAsync()?.get()?;

        let length = stream.Size()?;
        let buf = Buffer::Create(length as u32)?;
        stream
            .ReadAsync(&buf, length as u32, InputStreamOptions::None)?
            .get()?;

        let mut message = std::env::temp_dir();
        message.push("now-playing");

        std::fs::create_dir_all(&message)?;

        message.push("thumbnail.png");

        let _ = std::fs::File::create_new(&message);

        let file =
            StorageFile::GetFileFromPathAsync(&HSTRING::from(message.to_str().unwrap()))?.get()?;

        FileIO::WriteBufferAsync(&file, &buf)?.get()?;

        println!("{}", props.Title()?);
        println!("{}", props.Artist()?);
        println!("{:#?}", playback_state);
        println!("{}", message.to_str().ok_or("")?);
    } else {
        println!("Nothing playing.");
    }

    Ok(())
}
