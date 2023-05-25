use windows::core::{Result, HRESULT, HSTRING};
use windows::h;
use windows::Security::Cryptography::DataProtection::DataProtectionProvider;
use windows::Storage::Streams::{Buffer, IBuffer, InMemoryRandomAccessStream};
use windows::Storage::{CreationCollisionOption, FileAccessMode, FileIO, StorageFile};

async fn get_file(path: &HSTRING, copy_name: &HSTRING) -> Result<(StorageFile, StorageFile)> {
    let picked = StorageFile::GetFileFromPathAsync(path)?.await?;
    let parent_dir = picked.GetParentAsync()?.await?;
    let new_file = parent_dir
        .CreateFileAsync(copy_name, CreationCollisionOption::GenerateUniqueName)?
        .await?;
    return Ok((picked, new_file));
}

async fn protect_file(to_protect: &(StorageFile, StorageFile)) -> Result<()> {
    let (source, target) = to_protect;
    let provider = DataProtectionProvider::CreateOverloadExplicit(h!("LOCAL=user"))?;

    let source_stream = source.OpenAsync(FileAccessMode::Read)?.await?;
    let target_stream = target.OpenAsync(FileAccessMode::ReadWrite)?.await?;

    provider
        .ProtectStreamAsync(&source_stream, &target_stream)?
        .await?;

    Ok(())
}

async fn unprotect_file(to_unprotect: &(StorageFile, StorageFile)) -> Result<()> {
    let provider = DataProtectionProvider::new()?;
    let (source, target) = to_unprotect;
    let source_stream = source.OpenAsync(FileAccessMode::Read)?.await?;
    let target_stream = target
        .OpenAsync(FileAccessMode::ReadWrite)?
        .await?
        .GetOutputStreamAt(0)?;

    provider.UnprotectStreamAsync(&source_stream, &target_stream)?;

    target_stream.FlushAsync()?.await?;
    Ok(())
}

fn map_to_win_error(error: std::io::Error) -> windows::core::Error {
    let msg = error.to_string();
    let error_msg = HSTRING::from(msg);
    return windows::core::Error::new(HRESULT(1), error_msg);
}

pub async fn run_protection() -> Result<()> {
    let mut original = std::env::current_dir().map_err(map_to_win_error)?;
    original.push("Cargo.toml");

    let mut protected = std::env::current_dir().map_err(map_to_win_error)?;
    protected.push("Cargo-protected.toml");

    let to_protect = get_file(
        &HSTRING::from(original.to_str().unwrap()),
        h!("Cargo-protected.toml"),
    )
    .await?;

    protect_file(&to_protect).await?;

    let to_unprotect = get_file(
        &HSTRING::from(protected.to_str().unwrap()),
        h!("Cargo-unprotected.toml"),
    )
    .await?;

    unprotect_file(&to_unprotect).await?;

    Ok(())
}
