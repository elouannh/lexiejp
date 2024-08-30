#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiProfileStudied {
    pub today_all: i32,
    pub today_vocab: i32,
    pub today_grammar: i32,
    pub today_kanji: i32,
    pub today_sent: i32,
    pub today_conj: i32,
    pub today_aconj: i32,
    pub total: i32,
    pub total_vocab: i32,
    pub total_grammar: i32,
    pub total_kanji: i32,
    pub total_sent: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiProfileProgressPercsJLPT {
    pub n1: i32,
    pub n2: i32,
    pub n3: i32,
    pub n4: i32,
    pub n5: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiProfileProgressPercs {
    pub vocab: ApiProfileProgressPercsJLPT,
    pub kanji: ApiProfileProgressPercsJLPT,
    pub grammar: ApiProfileProgressPercsJLPT,
    pub sent: ApiProfileProgressPercsJLPT,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiProfileStreaksEachData {
    pub correct_in_a_row: String,
    pub correct_in_a_row_alltime: String,
    pub days_studied_in_a_row: String,
    pub days_studied_in_a_row_alltime: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiProfileStreaks {
    pub vocab: ApiProfileStreaksEachData,
    pub kanji: ApiProfileStreaksEachData,
    pub grammar: ApiProfileStreaksEachData,
    pub sent: ApiProfileStreaksEachData,
    pub conj: ApiProfileStreaksEachData,
    pub aconj: ApiProfileStreaksEachData,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiProfile {
    pub id: String,
    pub real_name: String,
    pub adventure_level: String,
    pub user_length: String,
    pub kao: String,
    pub studied: ApiProfileStudied,
    pub level_progress_percs: ApiProfileProgressPercs,
    pub streaks: ApiProfileStreaks,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiBeginnerProfileStudied {
    pub today_all: i32,
    pub today_vocab: i32,
    pub today_grammar: i32,
    pub today_kanji: i32,
    pub today_sent: i32,
    pub today_conj: i32,
    pub today_aconj: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiBeginnerProfile {
    pub id: String,
    pub real_name: String,
    pub adventure_level: String,
    pub user_length: String,
    pub kao: String,
    pub studied: ApiBeginnerProfileStudied,
    pub level_progress_percs: ApiProfileProgressPercs,
    pub streaks: ApiProfileStreaks,
}

pub enum StudiedEnum<'a> {
    Beginner(&'a ApiBeginnerProfileStudied),
    Normal(&'a ApiProfileStudied),
}

pub enum ProfileEnum {
    Beginner(ApiBeginnerProfile),
    Normal(ApiProfile),
}

impl StudiedEnum<'_> {
    pub fn today_all(&self) -> &i32 {
        match self {
            StudiedEnum::Beginner(content) => &content.today_all,
            StudiedEnum::Normal(content) => &content.today_all,
        }
    }

    pub fn today_vocab(&self) -> &i32 {
        match self {
            StudiedEnum::Beginner(content) => &content.today_vocab,
            StudiedEnum::Normal(content) => &content.today_vocab,
        }
    }

    pub fn today_grammar(&self) -> &i32 {
        match self {
            StudiedEnum::Beginner(content) => &content.today_grammar,
            StudiedEnum::Normal(content) => &content.today_grammar,
        }
    }

    pub fn today_kanji(&self) -> &i32 {
        match self {
            StudiedEnum::Beginner(content) => &content.today_kanji,
            StudiedEnum::Normal(content) => &content.today_kanji,
        }
    }

    pub fn today_sent(&self) -> &i32 {
        match self {
            StudiedEnum::Beginner(content) => &content.today_sent,
            StudiedEnum::Normal(content) => &content.today_sent,
        }
    }

    pub fn today_conj(&self) -> &i32 {
        match self {
            StudiedEnum::Beginner(content) => &content.today_conj,
            StudiedEnum::Normal(content) => &content.today_conj,
        }
    }

    pub fn today_aconj(&self) -> &i32 {
        match self {
            StudiedEnum::Beginner(content) => &content.today_aconj,
            StudiedEnum::Normal(content) => &content.today_aconj,
        }
    }

    pub fn total(&self) -> &i32 {
        match self {
            StudiedEnum::Beginner(content) => &content.today_all,
            StudiedEnum::Normal(content) => &content.total_sent,
        }
    }

    pub fn total_vocab(&self) -> &i32 {
        match self {
            StudiedEnum::Beginner(content) => &content.today_vocab,
            StudiedEnum::Normal(content) => &content.total_vocab,
        }
    }

    pub fn total_grammar(&self) -> &i32 {
        match self {
            StudiedEnum::Beginner(content) => &content.today_grammar,
            StudiedEnum::Normal(content) => &content.total_grammar,
        }
    }

    pub fn total_kanji(&self) -> &i32 {
        match self {
            StudiedEnum::Beginner(content) => &content.today_kanji,
            StudiedEnum::Normal(content) => &content.total_kanji,
        }
    }

    pub fn total_sent(&self) -> &i32 {
        match self {
            StudiedEnum::Beginner(content) => &content.today_sent,
            StudiedEnum::Normal(content) => &content.total_sent,
        }
    }
}

impl TryFrom<serde_json::Value> for ApiProfile {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value).map_err(|e| e.into())
    }
}

impl TryFrom<serde_json::Value> for ApiBeginnerProfile {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value).map_err(|e| e.into())
    }
}

impl ProfileEnum {
    pub fn create(content: &String) -> Result<ProfileEnum, Box<dyn std::error::Error>> {
        let json: serde_json::Value = serde_json::from_str(content)?;

        if let Ok(profile) = ApiProfile::try_from(json.clone()) {
            Ok(ProfileEnum::Normal(profile))
        } else if let Ok(beginner_profile) = ApiBeginnerProfile::try_from(json) {
            Ok(ProfileEnum::Beginner(beginner_profile))
        } else {
            Err("Could not deserialize into either ApiProfile or ApiBeginnerProfile".into())
        }
    }

    pub fn is_beginner(&self) -> bool {
        match self {
            ProfileEnum::Beginner(_) => true,
            ProfileEnum::Normal(_) => false,
        }
    }

    pub fn id(&self) -> &String {
        match self {
            ProfileEnum::Beginner(content) => &content.id,
            ProfileEnum::Normal(content) => &content.id,
        }
    }

    pub fn real_name(&self) -> &String {
        match self {
            ProfileEnum::Beginner(content) => &content.real_name,
            ProfileEnum::Normal(content) => &content.real_name,
        }
    }

    pub fn adventure_level(&self) -> &String {
        match self {
            ProfileEnum::Beginner(content) => &content.adventure_level,
            ProfileEnum::Normal(content) => &content.adventure_level,
        }
    }

    pub fn user_length(&self) -> &String {
        match self {
            ProfileEnum::Beginner(content) => &content.user_length,
            ProfileEnum::Normal(content) => &content.user_length,
        }
    }

    pub fn kao(&self) -> &String {
        match self {
            ProfileEnum::Beginner(content) => &content.kao,
            ProfileEnum::Normal(content) => &content.kao,
        }
    }

    pub fn studied(&self) -> StudiedEnum {
        match self {
            ProfileEnum::Beginner(content) => StudiedEnum::Beginner(&content.studied),
            ProfileEnum::Normal(content) => StudiedEnum::Normal(&content.studied),
        }
    }

    pub fn level_progress_percs(&self) -> &ApiProfileProgressPercs {
        match self {
            ProfileEnum::Beginner(content) => &content.level_progress_percs,
            ProfileEnum::Normal(content) => &content.level_progress_percs,
        }
    }

    pub fn streaks(&self) -> &ApiProfileStreaks {
        match self {
            ProfileEnum::Beginner(content) => &content.streaks,
            ProfileEnum::Normal(content) => &content.streaks,
        }
    }
}
