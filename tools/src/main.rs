use serde::*;
use std::io::Write;

static REPO_PATH: &str = "tinted-theming/schemes/blob/spec-0.11/base16";
static BASE_URL: &str = "https://api.github.com/repos";
static VERSION: &str = "spec-0.11";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Request failed")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Parsing error")]
    JSONParsingError(#[from] serde_json::Error),
    #[error("YAML Parsing error")]
    YAMLParsingError(#[from] serde_yml::Error),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("Unknown error")]
    Unknown(String),
}

#[derive(Debug, Deserialize)]
struct Links {
    #[serde(rename = "self")]
    _self_link: String,
    #[serde(rename = "git")]
    _git: String,
    #[serde(rename = "html")]
    _html: String,
}

#[derive(Debug, Deserialize)]
struct GithubFile {
    name: String,
    download_url: String,
    file_type: String,
    #[serde(rename = "path")]
    _path: String,
    #[serde(rename = "sha")]
    _sha: String,
    #[serde(rename = "size")]
    _size: u32,
    #[serde(rename = "url")]
    _url: String,
    #[serde(rename = "html_url")]
    _html_url: String,
    #[serde(rename = "git_url")]
    _git_url: String,
    #[serde(rename = "_links")]
    _links: Links,
}

impl GithubFile {
    fn get_download_url(&self) -> String {
        self.download_url.clone()
    }

    async fn download_file(&self) -> Result<Vec<u8>, Error> {
        let client = reqwest::Client::new();
        let response = client
            .get(self.get_download_url())
            .header("User-Agent", "stormlightlabs.org")
            .send();

        match response.await {
            Ok(res) => {
                println!("File downloaded successfully");
                let bytes = res.bytes().await?;

                Ok(bytes.to_vec())
            }
            Err(err) => Err(Error::RequestFailed(err)),
        }
    }

    async fn parse_color_scheme(&self) -> Result<Theme, Error> {
        let bytes = self.download_file().await?;
        match serde_yml::from_slice(&bytes) {
            Ok(palette) => Ok(palette),
            Err(why) => Err(Error::YAMLParsingError(why)),
        }
    }

    async fn create_theme_file(&self, theme: Theme, output_dir: Option<&str>) -> Result<(), Error> {
        let file_name = format!("{}.yaml", self.name);
        let file_path = match output_dir {
            Some(path) => format!("{path}/{file_name}"),
            None => "themes/{file_name}".to_string(),
        };

        match std::fs::File::create(file_path) {
            Ok(mut file) => {
                let yaml_string = serde_yml::to_string(&theme)?;
                if let Ok(_) = file.write_all(yaml_string.as_bytes()) {
                    return Ok(());
                }

                let err = std::io::Error::new(std::io::ErrorKind::Other, "Failed to write to file");
                Err(Error::IOError(err))
            }
            Err(why) => {
                let err = std::io::Error::new(why.kind(), "Failed to create file");
                Err(Error::IOError(err))
            }
        }
    }
}

async fn get_file_listing() -> Result<Vec<GithubFile>, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{BASE_URL}/{REPO_PATH}?ref={VERSION}"))
        .header("User-Agent", "stormlightlabs.org")
        .send();

    match response.await {
        Ok(res) => {
            let body = res.text().await?;
            match serde_json::from_str(&body) {
                Ok(files) => Ok(files),
                Err(why) => Err(Error::JSONParsingError(why)),
            }
        }
        Err(err) => Err(Error::RequestFailed(err)),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Palette {
    base00: String,
    base01: String,
    base02: String,
    base03: String,
    base04: String,
    base05: String,
    base06: String,
    base07: String,
    base08: String,
    base09: String,
    #[serde(rename = "base0A")]
    base0a: String,
    #[serde(rename = "base0B")]
    base0b: String,
    #[serde(rename = "base0C")]
    base0c: String,
    #[serde(rename = "base0D")]
    base0d: String,
    #[serde(rename = "base0E")]
    base0e: String,
    #[serde(rename = "base0F")]
    base0f: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum ThemeVariant {
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "light")]
    Light,
}

#[derive(Debug, Serialize, Deserialize)]
struct Theme {
    system: String,
    name: String,
    author: String,
    variant: ThemeVariant,
    palette: Palette,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listing = get_file_listing().await?;

    for file in listing {
        log::debug!("File: {:?}", file);

        if file.file_type == "file" {
            let theme = file.parse_color_scheme().await.unwrap();
            log::debug!(
                "Theme: {}",
                format!(
                    "system: {} name: {} variant: {:?} author: {}",
                    theme.system, theme.name, theme.variant, theme.author,
                )
            );
            file.create_theme_file(theme, None).await?;
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }

    Ok(())
}
