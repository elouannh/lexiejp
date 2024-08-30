#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum UserPrivacyVisibility {
    On,
    Off,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserPrivacy {
    pub visibility: UserPrivacyVisibility,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub discord_id: String,
    pub renshuu_api_key: String,
    pub privacy: UserPrivacy,
    pub cache: String,
    pub refresh_timeout: u64,
    pub last_caching_timestamp: u64,
}

impl User {
    pub fn default(discord_id: &String, renshuu_api_key: &String) -> User {
        let user_privacy: UserPrivacy = UserPrivacy {
            visibility: UserPrivacyVisibility::Off,
        };
        User {
            discord_id: discord_id.to_string(),
            renshuu_api_key: renshuu_api_key.to_string(),
            privacy: user_privacy,
            cache: String::new(),
            refresh_timeout: 21_600,
            last_caching_timestamp: 0,
        }
    }
}
