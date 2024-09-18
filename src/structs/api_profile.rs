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

macro_rules! impl_study_getter {
    ($name:ident, $beginner_field:ident, $normal_field:ident) => {
        pub fn $name(&self) -> &i32 {
            match self {
                StudiedEnum::Beginner(content) => &content.$beginner_field,
                StudiedEnum::Normal(content) => &content.$normal_field,
            }
        }
    };
}

impl StudiedEnum<'_> {
    impl_study_getter!(today_all, today_all, today_all);
    impl_study_getter!(today_vocab, today_vocab, today_vocab);
    impl_study_getter!(today_grammar, today_grammar, today_grammar);
    impl_study_getter!(today_kanji, today_kanji, today_kanji);
    impl_study_getter!(today_sent, today_sent, today_sent);
    impl_study_getter!(today_conj, today_conj, today_conj);
    impl_study_getter!(today_aconj, today_aconj, today_aconj);
    impl_study_getter!(total, today_all, total);
    impl_study_getter!(total_vocab, today_vocab, total_vocab);
    impl_study_getter!(total_grammar, today_grammar, total_grammar);
    impl_study_getter!(total_kanji, today_kanji, total_kanji);
    impl_study_getter!(total_sent, today_sent, total_sent);
}

macro_rules! impl_try_from_json_value {
    ($type:ty) => {
        impl TryFrom<serde_json::Value> for $type {
            type Error = Box<dyn std::error::Error>;

            fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
                serde_json::from_value(value).map_err(|e| e.into())
            }
        }
    };
}

impl_try_from_json_value!(ApiProfile);
impl_try_from_json_value!(ApiBeginnerProfile);

macro_rules! impl_profile_getter {
    ($method:ident, $return_type:ty) => {
        pub fn $method(&self) -> &$return_type {
            match self {
                ProfileEnum::Beginner(content) => &content.$method,
                ProfileEnum::Normal(content) => &content.$method
            }
        }
    };
}

impl ProfileEnum {
    pub fn create(content: &String) -> Result<ProfileEnum, Box<dyn std::error::Error>> {
        let json: serde_json::Value = serde_json::from_str(content)?;

        match (ApiProfile::try_from(json.clone()), ApiBeginnerProfile::try_from(json)) {
            (Ok(profile), _) => Ok(ProfileEnum::Normal(profile)),
            (_, Ok(beginner_profile)) => Ok(ProfileEnum::Beginner(beginner_profile)),
            (Err(e1), Err(e2)) => Err(format!("Failed to parse as either profile type. Normal error: {}, beginner error: {}", e1, e2).into())
        }
    }

    pub fn is_beginner(&self) -> bool {
        matches!(self, ProfileEnum::Beginner(_))
    }

    pub fn studied(&self) -> StudiedEnum {
        match self {
            ProfileEnum::Beginner(content) => StudiedEnum::Beginner(&content.studied),
            ProfileEnum::Normal(content) => StudiedEnum::Normal(&content.studied),
        }
    }

    impl_profile_getter!(id, String);
    impl_profile_getter!(real_name, String);
    impl_profile_getter!(adventure_level, String);
    impl_profile_getter!(user_length, String);
    impl_profile_getter!(kao, String);
    impl_profile_getter!(level_progress_percs, ApiProfileProgressPercs);
    impl_profile_getter!(streaks, ApiProfileStreaks);
}
