use once_cell::sync::Lazy;
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::{copy, Write};
use std::path::Path;
use std::time::Duration;

use crate::sys::{create_dir, is_current_dir, parent_dir};

const HTTP_USER_AGENT: &str = "X-RUST-APP";
const DEFAULT_TIMEOUT_SECS: u64 = 30;
const DEFAULT_CONNECT_TIMEOUT_SECS: u64 = 10;

pub static HTTP_CLIENT: Lazy<Result<Client, String>> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
        .connect_timeout(Duration::from_secs(DEFAULT_CONNECT_TIMEOUT_SECS))
        .user_agent(HTTP_USER_AGENT)
        .pool_idle_timeout(Duration::from_secs(90))
        .pool_max_idle_per_host(10)
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))
});

#[derive(Debug)]
pub enum HttpError {
    Network(String),
    FileSystem(String),
    InvalidResponse(String),
    RequestFailed(u16, String),
}

impl HttpError {
    #[inline]
    pub fn to_string(&self) -> String {
        match self {
            Self::Network(msg) => format!("Network error: {}", msg),
            Self::FileSystem(msg) => format!("File system error: {}", msg),
            Self::InvalidResponse(msg) => format!("Invalid response: {}", msg),
            Self::RequestFailed(code, msg) => {
                format!("HTTP request failed with status code {}: {}", code, msg)
            }
        }
    }
}

impl From<HttpError> for String {
    fn from(err: HttpError) -> String {
        err.to_string()
    }
}

pub fn fetch_json<T: DeserializeOwned>(url: &str) -> Result<T, String> {
    fetch_json_impl(url).map_err(|e| e.into())
}

fn fetch_json_impl<T: DeserializeOwned>(url: &str) -> Result<T, HttpError> {
    if url.trim().is_empty() {
        return Err(HttpError::InvalidResponse(
            "URL cannot be empty".to_string(),
        ));
    }

    let client = HTTP_CLIENT
        .as_ref()
        .map_err(|e| HttpError::Network(e.clone()))?;

    let response = client
        .get(url)
        .send()
        .map_err(|e| HttpError::Network(e.to_string()))?;
    if !response.status().is_success() {
        let status = response.status();
        return Err(HttpError::RequestFailed(
            status.as_u16(),
            format!("Failed to fetch JSON from {}", url),
        ));
    }

    response
        .json::<T>()
        .map_err(|e| HttpError::InvalidResponse(e.to_string()))
}

pub fn download_file<P: AsRef<Path>>(url: &str, output_filepath: P) -> Result<(), String> {
    download_file_with_progress(url, output_filepath, None::<fn(u64, u64)>).map_err(|e| e.into())
}

pub fn download_file_with_progress<P, F>(
    url: &str,
    output_filepath: P,
    progress_callback: Option<F>,
) -> Result<(), HttpError>
where
    P: AsRef<Path>,
    F: Fn(u64, u64),
{
    if url.trim().is_empty() {
        return Err(HttpError::InvalidResponse(
            "URL cannot be empty".to_string(),
        ));
    }

    let output_filepath = output_filepath.as_ref();
    if output_filepath.as_os_str().is_empty() {
        return Err(HttpError::FileSystem(
            "Output file path cannot be empty".to_string(),
        ));
    }
    if output_filepath.exists() {
        return Err(HttpError::FileSystem(format!(
            "File or directory already exists: {}",
            output_filepath.display()
        )));
    }

    let client = HTTP_CLIENT
        .as_ref()
        .map_err(|e| HttpError::Network(e.clone()))?;

    let mut response = client
        .get(url)
        .send()
        .map_err(|e| HttpError::Network(e.to_string()))?;
    if !response.status().is_success() {
        let status = response.status();
        return Err(HttpError::RequestFailed(
            status.as_u16(),
            format!("Failed to download from {}", url),
        ));
    }

    let total_size = response.content_length().unwrap_or(0);

    ensure_parent_dir_exists(output_filepath)?;

    let mut out = File::create(output_filepath)
        .map_err(|e| HttpError::FileSystem(format!("Failed to create file: {}", e)))?;

    if let Some(callback) = progress_callback {
        copy_with_progress(&mut response, &mut out, total_size, callback)?;
    } else {
        copy(&mut response, &mut out)
            .map_err(|e| HttpError::Network(format!("Failed to download: {}", e)))?;
    }

    out.flush()
        .map_err(|e| HttpError::FileSystem(format!("Failed to flush file: {}", e)))?;

    Ok(())
}

fn copy_with_progress<R, W, F>(
    reader: &mut R,
    writer: &mut W,
    total_size: u64,
    progress_callback: F,
) -> Result<(), HttpError>
where
    R: std::io::Read,
    W: std::io::Write,
    F: Fn(u64, u64),
{
    let mut buffer = vec![0; 8192];
    let mut downloaded = 0u64;

    loop {
        let bytes_read = reader
            .read(&mut buffer)
            .map_err(|e| HttpError::Network(format!("Read error: {}", e)))?;
        if bytes_read == 0 {
            break;
        }

        writer
            .write_all(&buffer[..bytes_read])
            .map_err(|e| HttpError::FileSystem(format!("Write error: {}", e)))?;

        downloaded = downloaded.saturating_add(bytes_read as u64);

        progress_callback(downloaded, total_size);
    }

    Ok(())
}

fn ensure_parent_dir_exists(filepath: &Path) -> Result<(), HttpError> {
    let parent = parent_dir(filepath);
    if !parent.is_dir() && !is_current_dir(&parent) {
        create_dir(&parent).map_err(|e| HttpError::FileSystem(e))?;
    }

    Ok(())
}
