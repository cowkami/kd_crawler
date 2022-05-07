use reqwest::Url;
use std::error::Error;
struct KdUrl {
    url: Url,
}

impl KdUrl {
    fn new() -> Result<KdUrl, Box<dyn Error>> {
        Ok(Self {
            url: Url::parse("https://keirin.kdreams.jp/")?,
        })
    }
    fn base(base_url: &str) -> Result<KdUrl, Box<dyn Error>> {
        Ok(Self {
            url: Url::parse(base_url)?,
        })
    }
    fn kaisai(self) -> Result<KdUrl, Box<dyn Error>> {
        let url = self.url.join("kaisai")?;
        Ok(Self {
            url: Url::parse(url.as_str())?,
        })
    }
}

struct KdApi {
    base_url: Url,
}

impl KdApi {
    fn get_race_by_date(date: &str) {}
}

#[cfg(test)]
mod kdurl_tests {
    use super::*;

    #[test]
    fn base() {
        assert_eq!(
            KdUrl::new().unwrap().url.as_str(),
            KdUrl::base("https://keirin.kdreams.jp/")
                .unwrap()
                .url
                .as_str()
        );
    }
    #[test]
    fn kaisai() {
        assert_eq!(
            KdUrl::new().unwrap().kaisai().unwrap().url.as_str(),
            "https://keirin.kdreams.jp/kaisai"
        )
    }
}
