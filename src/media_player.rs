use windows::core::HSTRING;
use windows::Foundation::Collections::*;
use windows::Media::Core::*;
use windows::Media::Playback::*;
use windows::Media::*;
use windows::Storage::Search::*;
use windows::Storage::Streams::*;
use windows::Storage::*;

use futures::executor::block_on;

async fn get_files() -> windows::core::Result<IVectorView<StorageFile>> {
    let query_opts = QueryOptions::new()?;
    query_opts.SetApplicationSearchFilter(&HSTRING::from(".mp3"))?;
    let query = KnownFolders::MusicLibrary()?.CreateFileQueryWithOptions(&query_opts)?;
    return query.GetFilesAsync(0, 5)?.await;
}

async fn get_media_playback_item(file: StorageFile) -> windows::core::Result<MediaPlaybackItem> {
    let music_props = file.Properties()?.GetMusicPropertiesAsync()?.await?;
    let file_stream = file.OpenAsync(FileAccessMode::Read)?.await;
    let source = MediaSource::CreateFromStream(&file_stream?, &file.ContentType()?);
    let playback_item = MediaPlaybackItem::Create(&source?)?;
    let display_props = playback_item.GetDisplayProperties()?;
    display_props.SetType(MediaPlaybackType::Music)?;
    display_props
        .MusicProperties()?
        .SetTitle(&music_props.Title()?)?;
    display_props
        .MusicProperties()?
        .SetArtist(&music_props.Artist()?)?;
    display_props
        .MusicProperties()?
        .SetAlbumArtist(&music_props.AlbumArtist()?)?;
    display_props
        .MusicProperties()?
        .SetTrackNumber(music_props.TrackNumber()?)?;
    playback_item.ApplyDisplayProperties(&display_props)?;
    let thumbnail = file
        .GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(
            FileProperties::ThumbnailMode::MusicView,
        )?
        .await?;
    display_props.SetThumbnail(&RandomAccessStreamReference::CreateFromStream(&thumbnail)?)?;
    return Ok(playback_item);
}

async fn get_playlist(files: IVectorView<StorageFile>) -> windows::core::Result<MediaPlaybackList> {
    let pl = MediaPlaybackList::new().unwrap();
    //    let files = vec![];
    for file in files {
        let item = get_media_playback_item(file).await?;
        pl.Items()?.Append(&item)?;
    }
    return Ok(pl);
}

pub async fn play() -> windows::core::Result<()> {
    let player = MediaPlayer::new()?;
    player.SetAutoPlay(true)?;
    let files = get_files().await?;
    let playlist = get_playlist(files).await?;
    player.SetSource(&playlist)?;
    player.SetIsLoopingEnabled(true)?;
    while player.IsLoopingEnabled()? {}
    Ok(())
}
