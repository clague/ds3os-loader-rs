use serde_json::json;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use reqwest::{Client, header::*, Url, IntoUrl, Method};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MASTER_SERVER_ADDR_DEF: &'static str = "http://timleonard.uk:50020/api/v1/servers/";
}


#[derive(Clone)]
pub enum ApiVersion {
    V1,
    V2,
    Unknown
}

impl From<u32> for ApiVersion {
    fn from(n: u32) -> ApiVersion {
        match n {
            1 => ApiVersion::V1,
            2 => ApiVersion::V2,
            _ => ApiVersion::Unknown
        }
    }
}

impl From<&str> for ApiVersion {
    fn from(s: &str) -> ApiVersion {
        let ss = String::from(s);
        if ss.contains("1") {
            ApiVersion::V1
        }
        else if ss.contains("2") {
            ApiVersion::V2
        }
        else {
            ApiVersion::Unknown
        }
    }
}

impl Into<String> for ApiVersion {
    fn into(self) -> String {
        match self {
            ApiVersion::V1 => "v1".to_string(),
            ApiVersion::V2 => "v2".to_string(),
            ApiVersion::Unknown => "".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    status: String,
    #[serde(default)]
    message: String,
    #[serde(default)]
    servers: Vec<Server>,
    #[serde(rename = "PublicKey", default)]
    public_key: String,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Server {
    #[serde(rename = "IpAddress", default)]
    pub ip_addr: String,
    #[serde(rename = "Hostname")]
    pub hostname: String,
    #[serde(rename = "PrivateHostname", default)]
    pub private_hostname: String,
    #[serde(rename = "Description", default)]
    pub description: String,
    #[serde(rename = "Name", default)]
    pub name: String,
    #[serde(rename = "PlayerCount", default)]
    pub player_count: u32,
    #[serde(rename = "PasswordRequired", default)]
    pub password_required: bool,
    #[serde(rename = "Password", default)]
    pub passwd: String,
    #[serde(rename = "ModsWhiteList", default)]
    pub mods_white_list: String,
    #[serde(rename = "ModsBlackList", default)]
    pub mods_black_list: String,
    #[serde(rename = "ModsRequiredList", default)]
    pub mods_required_list: String,
    #[serde(rename = "PublicKey", default)]
    pub pubkey: String,
}

#[derive(Clone)]
pub struct MasterServerApi {
    api_url: Url,
    http_client: Client,
    version: ApiVersion,
}

impl MasterServerApi {
    pub fn new<U, V>(api_url: U, version: V) -> Result<MasterServerApi>
    where
        U: IntoUrl,
        V: Into<ApiVersion>,
    {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let http_client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_default();

        Ok(MasterServerApi {
            api_url: api_url.into_url()?,
            http_client,
            version: version.into(),
        })
    }

    pub async fn request<S>(&self, method: Method, url: &Url, request_body: Option<S>) -> Result<ApiResponse>
    where
        S: Serialize,
    {
        let builder = self.http_client.request(method, url.clone());

        if let Some(r) = request_body {
            Ok(builder.json(&r)
                .send()
                .await?
                .json::<ApiResponse>()
                .await?
            )
        }
        else {
            Ok(builder.send()
                .await?
                .json::<ApiResponse>()
                .await?)
        }
    }

    pub async fn list_servers(self) -> Result<Vec<Server>> {
        let res = self.request::<String>(Method::GET, &self.api_url, None).await?;
        if res.status == "success" && res.servers.len() > 0{
            Ok(res.servers)
        }
        else {
            Err(anyhow!("Master server return error!"))
        }
    }

    pub async fn get_pubkey(&self, ip_addr: &str, password: &str) -> Result<String> {
        let req_body = json!({
            "password": password,
        });
        let res = self.request(Method::POST, &self.api_url.join(&format!("{}/public_key", ip_addr))?, Some(req_body)).await?;
        if res.status == "success" && !res.public_key.is_empty() {
            Ok(res.public_key)
        }
        else {
            //println!("{:#?}", res);
            Err(anyhow!(res.message))
        }
    }
}