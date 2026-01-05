use anyhow::Result;
use bytes::Bytes;
use once_cell::sync::Lazy;
use reqwest::{
    header::{HeaderMap, ACCEPT, CACHE_CONTROL, USER_AGENT},
    Client, Url,
};
use std::{ffi::OsStr, path::Path, time::Duration};

static HTTP_CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert(ACCEPT, "*/*".parse().unwrap());
    headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
    headers
}

pub fn client() -> Client {
    Client::new()
}

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
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_file_extension() -> Result<()> {
        assert_eq!(Some("pdf".to_string()), file_extension("https://www.example.com/test.pdf")?);
        assert_eq!(Some("jpg".to_string()), file_extension("https://example.com/image.jpg")?);
        assert_eq!(Some("png".to_string()), file_extension("http://example.com/photo.png")?);

        assert_eq!(Some("pdf".to_string()), file_extension("https://example.com/file.pdf?param=value")?);

        assert_eq!(None, file_extension("https://example.com/")?);
        assert_eq!(None, file_extension("https://example.com")?);
        assert_eq!(None, file_extension("https://example.com/path/")?);

        assert_eq!(Some("gz".to_string()), file_extension("https://example.com/archive.tar.gz")?);

        Ok(())
    }

    #[test]
    fn test_file_extension_invalid_url() {
        assert!(file_extension("not-a-url").is_err());
    }
}
