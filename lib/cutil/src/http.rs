//! HTTP client utilities for making HTTP requests and handling URLs.

use anyhow::Result;
use bytes::Bytes;
use once_cell::sync::Lazy;
use reqwest::{
    header::{HeaderMap, ACCEPT, CACHE_CONTROL, USER_AGENT},
    Client, Url,
};
use std::{ffi::OsStr, path::Path, time::Duration};

static HTTP_CLIENT: Lazy<Client> = Lazy::new(Client::new);

/// Creates a set of common HTTP headers for web requests.
///
/// The headers include:
/// - User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36
/// - Accept: */*
/// - Cache-Control: no-cache
///
/// # Returns
///
/// Returns a `HeaderMap` with the common headers set.
///
/// # Examples
///
/// ```
/// use cutil::http::headers;
///
/// let headers = headers();
/// assert!(headers.contains_key("user-agent"));
/// ```
pub fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert(ACCEPT, "*/*".parse().unwrap());
    headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
    headers
}

/// Creates a new HTTP client.
///
/// # Returns
///
/// Returns a new `reqwest::Client` instance.
///
/// # Examples
///
/// ```
/// use cutil::http::client;
///
/// let client = client();
/// // Use the client for HTTP requests
/// ```
pub fn client() -> Client {
    Client::new()
}

/// Fetches data from a URL as bytes with a timeout.
///
/// # Arguments
///
/// * `url` - The URL to fetch data from
/// * `timeout` - The maximum time to wait for the request
///
/// # Returns
///
/// Returns the response data as `Bytes` on success.
///
/// # Examples
///
/// ```no_run
/// use cutil::http::get_bytes;
/// use std::time::Duration;
///
/// // Note: This function requires an async runtime
/// // let data = get_bytes("https://example.com", Duration::from_secs(10)).await.unwrap();
/// // println!("Fetched {} bytes", data.len());
/// ```
pub async fn get_bytes(url: &str, timeout: Duration) -> Result<Bytes> {
    let data = HTTP_CLIENT
        .get(url)
        .timeout(timeout)
        .send()
        .await?
        .bytes()
        .await?;
    Ok(data)
}

/// Extracts the file extension from a URL.
///
/// # Arguments
///
/// * `url` - The URL to extract the file extension from
///
/// # Returns
///
/// Returns `Some(String)` with the file extension if found, or `None` if no extension exists.
///
/// # Examples
///
/// ```
/// use cutil::http::file_extension;
///
/// assert_eq!(file_extension("https://example.com/file.pdf").unwrap(), Some("pdf".to_string()));
/// assert_eq!(file_extension("https://example.com/").unwrap(), None);
/// ```
pub fn file_extension(url: &str) -> Result<Option<String>> {
    let url = Url::parse(url)?;
    let path = url.path();

    Ok(Path::new(path)
        .file_name()
        .map(Path::new)
        .and_then(Path::extension)
        .and_then(OsStr::to_str)
        .map(String::from))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headers() {
        let headers = headers();
        assert!(headers.contains_key("user-agent"));
        assert!(headers.contains_key("accept"));
        assert!(headers.contains_key("cache-control"));
        
        let user_agent = headers.get("user-agent").unwrap();
        assert!(user_agent.to_str().unwrap().contains("Mozilla"));
    }

    #[test]
    fn test_client() {
        let client = client();
        // Just verify we can create a client without panicking
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_file_extension() -> Result<()> {
        // Test with common extensions
        assert_eq!(Some("pdf".to_string()), file_extension("https://www.example.com/test.pdf")?);
        assert_eq!(Some("jpg".to_string()), file_extension("https://example.com/image.jpg")?);
        assert_eq!(Some("png".to_string()), file_extension("http://example.com/photo.png")?);
        
        // Test with query parameters
        assert_eq!(Some("pdf".to_string()), file_extension("https://example.com/file.pdf?param=value")?);
        
        // Test without extension
        assert_eq!(None, file_extension("https://example.com/")?);
        assert_eq!(None, file_extension("https://example.com")?);
        assert_eq!(None, file_extension("https://example.com/path/")?);
        
        // Test with multiple dots
        assert_eq!(Some("gz".to_string()), file_extension("https://example.com/archive.tar.gz")?);
        
        Ok(())
    }

    #[test]
    fn test_file_extension_invalid_url() {
        // Test with invalid URL
        assert!(file_extension("not-a-url").is_err());
    }
}
