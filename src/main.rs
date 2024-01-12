use anyhow::Result;
use windows::Media::Control::GlobalSystemMediaTransportControlsSession as GSMTCSession;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager as GSMTCSessionManager;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionPlaybackControls as GSMTCPlaybackControls;

#[tokio::main]
async fn main() -> Result<()> {
    let session_manager = GSMTCSessionManager::RequestAsync()?.await?;
    let current_session = session_manager.GetCurrentSession()?;

    let app_user_model_id = current_session.SourceAppUserModelId()?;
    println!("app_user_model_id: {app_user_model_id}");

    println!();

    println!("media_properties:");
    let _ = print_media_properties(&current_session, 1)
        .await
        .map_err(|err| println!("{err}"));

    println!("playback_info:");
    let _ = print_playback_info(&current_session, 1).map_err(|err| println!("{err}"));

    println!("timeline_properties:");
    let _ = print_timeline_properties(&current_session, 1).map_err(|err| println!("{err}"));

    Ok(())
}

fn print_timeline_properties(session: &GSMTCSession, depth: usize) -> Result<()> {
    let prefix = " ".chars().cycle().take(depth * 4).collect::<String>();
    let timeline_properties = session.GetTimelineProperties()?;

    let start_time = timeline_properties.StartTime()?;
    let end_time = timeline_properties.EndTime()?;
    let max_seek_time = timeline_properties.MaxSeekTime()?;
    let min_seek_time = timeline_properties.MinSeekTime()?;
    let position = timeline_properties.Position()?;
    let last_updated_time = timeline_properties.LastUpdatedTime()?;

    println!("{prefix}start_time: {start_time:?}");
    println!("{prefix}end_time: {end_time:?}");
    println!("{prefix}max_seek_time: {max_seek_time:?}");
    println!("{prefix}min_seek_time: {min_seek_time:?}");
    println!("{prefix}position: {position:?}");
    println!("{prefix}last_updated_time: {last_updated_time:?}");
    Ok(())
}

fn print_playback_info(session: &GSMTCSession, depth: usize) -> Result<()> {
    let prefix = " ".chars().cycle().take(depth * 4).collect::<String>();
    let playback_info = session.GetPlaybackInfo()?;

    let auto_repeat_mode = playback_info.AutoRepeatMode()?.Value()?;
    println!("{prefix}auto_repeat_mode: {auto_repeat_mode:?}");

    let controls = playback_info.Controls()?;
    println!("{prefix}controls:");
    print_playback_controls(controls, depth + 1)?;

    let is_shuffle_active = playback_info.IsShuffleActive()?.Value()?;
    println!("{prefix}is_shuffle_active: {is_shuffle_active}");

    if let Ok(playback_rate) = playback_info.PlaybackRate().and_then(|info| info.Value()) {
        println!("{prefix}playback_rate: {playback_rate}x");
    }

    let playback_status = playback_info.PlaybackStatus()?;
    println!("{prefix}playback_status: {playback_status:?}");

    let playback_type = playback_info.PlaybackType()?.Value()?;
    println!("{prefix}playback_type: {playback_type:?}");

    Ok(())
}

fn print_playback_controls(controls: GSMTCPlaybackControls, depth: usize) -> Result<()> {
    let prefix = " ".chars().cycle().take(depth * 4).collect::<String>();

    let is_channel_down_enabled = controls.IsChannelDownEnabled()?;
    let is_channel_up_enabled = controls.IsChannelUpEnabled()?;
    let is_fast_forward_enabled = controls.IsFastForwardEnabled()?;
    let is_next_enabled = controls.IsNextEnabled()?;
    let is_pause_enabled = controls.IsPauseEnabled()?;
    let is_playback_position_enabled = controls.IsPlaybackPositionEnabled()?;
    let is_playback_rate_enabled = controls.IsPlaybackRateEnabled()?;
    let is_play_enabled = controls.IsPlayEnabled()?;
    let is_play_pause_toggle_enabled = controls.IsPlayPauseToggleEnabled()?;
    let is_previous_enabled = controls.IsPreviousEnabled()?;
    let is_record_enabled = controls.IsRecordEnabled()?;
    let is_repeat_enabled = controls.IsRepeatEnabled()?;
    let is_rewind_enabled = controls.IsRewindEnabled()?;
    let is_shuffle_enabled = controls.IsShuffleEnabled()?;
    let is_stop_enabled = controls.IsStopEnabled()?;

    println!("{prefix}is_channel_down_enabled: {is_channel_down_enabled}");
    println!("{prefix}is_channel_up_enabled: {is_channel_up_enabled}");
    println!("{prefix}is_fast_forward_enabled: {is_fast_forward_enabled}");
    println!("{prefix}is_next_enabled: {is_next_enabled}");
    println!("{prefix}is_pause_enabled: {is_pause_enabled}");
    println!("{prefix}is_playback_position_enabled: {is_playback_position_enabled}");
    println!("{prefix}is_playback_rate_enabled: {is_playback_rate_enabled}");
    println!("{prefix}is_play_enabled: {is_play_enabled}");
    println!("{prefix}is_play_pause_toggle_enabled: {is_play_pause_toggle_enabled}");
    println!("{prefix}is_previous_enabled: {is_previous_enabled}");
    println!("{prefix}is_record_enabled: {is_record_enabled}");
    println!("{prefix}is_repeat_enabled: {is_repeat_enabled}");
    println!("{prefix}is_rewind_enabled: {is_rewind_enabled}");
    println!("{prefix}is_shuffle_enabled: {is_shuffle_enabled}");
    println!("{prefix}is_stop_enabled: {is_stop_enabled}");
    Ok(())
}

async fn print_media_properties(session: &GSMTCSession, depth: usize) -> Result<()> {
    let prefix = " ".chars().cycle().take(depth * 4).collect::<String>();
    let media_properties = session.TryGetMediaPropertiesAsync()?.await?;

    let album_artist = media_properties.AlbumArtist()?;
    let album_title = media_properties.AlbumTitle()?;
    let album_track_count = media_properties.AlbumTrackCount()?;
    let artist = media_properties.Artist()?;
    let genres = media_properties.Genres()?;
    let playback_type = media_properties.PlaybackType()?.Value()?;
    let subtitle = media_properties.Subtitle()?;
    let thumbnail = media_properties.Thumbnail()?.OpenReadAsync()?.await?;
    let title = media_properties.Title()?;
    let track_number = media_properties.TrackNumber()?;

    println!("{prefix}album_artist: {album_artist}");
    println!("{prefix}album_title: {album_title}");
    println!("{prefix}album_track_count: {album_track_count}");
    println!("{prefix}artist: {artist}");
    println!("{prefix}genres:");
    for genre in genres {
        println!("{prefix}     - {genre}, ");
    }
    println!("{prefix}playback_type: {playback_type:?}");
    println!("{prefix}subtitle: {subtitle}");
    println!("{prefix}thumbnail:");
    println!("{prefix}    content_type: {}", thumbnail.ContentType()?);
    println!("{prefix}    size: {}", thumbnail.Size()?);
    println!("{prefix}title: {title}");
    println!("{prefix}track_number: {track_number}");

    Ok(())
}
