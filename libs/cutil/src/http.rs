use anyhow::Result;
use bytes::Bytes;
use reqwest::{
    header::{HeaderMap, ACCEPT, CACHE_CONTROL, USER_AGENT},
    Client, Proxy, Url,
};
use std::{ffi::OsStr, path::Path, time::Duration};

pub enum ProxyType {
    Http,
    Socks5,
    Unknown,
}

impl From<&str> for ProxyType {
    fn from(pt: &str) -> Self {
        match pt.to_lowercase().as_str() {
            "http" => ProxyType::Http,
            "socks5" => ProxyType::Socks5,
            _ => ProxyType::Unknown,
        }
    }
}

pub fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert(ACCEPT, "*/*".parse().unwrap());

    headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
    headers
}

pub fn client(conf: Option<(ProxyType, String, u16)>) -> Result<Client> {
    match conf {
        Some((proxy, url, port)) => {
            let proxy_url = match proxy {
                ProxyType::Http => Proxy::all(format!("http://{}:{}", url, port))?,
                ProxyType::Socks5 => Proxy::all(format!("socks5://{}:{}", url, port))?,
                _ => return Ok(Client::new()),
            };
            Ok(Client::builder().proxy(proxy_url).build()?)
        }
        None => Ok(Client::new()),
    }
}

pub async fn get_bytes(url: &str, conf: Option<(ProxyType, String, u16)>) -> Result<Bytes> {
    let client = client(conf)?;
    let data = client
        .get(url)
        .timeout(Duration::from_secs(60))
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
        .and_then(|item| Some(Path::new(item)))
        .and_then(Path::extension)
        .and_then(OsStr::to_str)
        .and_then(|item| Some(String::from(item))))
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
