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
    fn test_file_extension() -> Result<()> {
        let url = "https://www.example.com/test.pdf";
        assert_eq!(Some("pdf".to_string()), file_extension(url)?);
        Ok(())
    }
}
