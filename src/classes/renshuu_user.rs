use crate::renshuu::rest_agent;
use crate::structs::{api_profile, user};
use crate::types::ctx;

pub struct RenshuuUser {
    pub db_data: user::User,
    pub user: serenity::all::User,
    pub content: String,
    pub profile: api_profile::ProfileEnum,
    pub levels_vec: Vec<i32>,
    pub is_beginner: bool,
    pub already_exists: bool,
}

impl RenshuuUser {
    pub async fn new(ctx: &ctx::Context<'_>, renshuu_api_key: &String) -> Result<Self, Box<dyn std::error::Error>> {
        let user_exists: bool = renshuu_api_key.is_some();
        let db_data: user::User = Self::create_db_data(ctx, renshuu_api_key);
        let mut content = Self::fetch_content(&db_data, renshuu_api_key).await?; // maybe can remove the mut here?
        let profile: api_profile::ProfileEnum =
            api_profile::ProfileEnum::create(&content)
                .map_err(|_| "Invalid user.")?;
        let is_beginner: bool = profile.is_beginner();

        Ok(RenshuuUser {
            db_data,
            user: ctx.author().to_owned(),
            content,
            profile,
            levels_vec: vec![],
            is_beginner,
            already_exists: user_exists,
        })
    }

    fn create_db_data(ctx: &ctx::Context<'_>, renshuu_api_key: &String) -> user::User {
        user::User::default(&ctx.author().id.to_string(), renshuu_api_key)
    }

    async fn fetch_content(db_data: &user::User, renshuu_api_key: &String) -> Result<String, Box<dyn std::error::Error>> {
        if db_data.cache.len() >= 2 {
            return Ok(db_data.cache.clone());
        }

        let rest_agent: rest_agent::RestAgent = rest_agent::RestAgent::new(renshuu_api_key);
        rest_agent.get_method("https://api.renshuu.org/v1/profile").await
            .map_err(|e| e.into())
    }

    pub fn register_data(&self) -> user::User {
        let mut user_privacy: user::UserPrivacy = user::UserPrivacy {
            visibility: user::UserPrivacyVisibility::Off,
        };

        if self.db_data.privacy.visibility == user::UserPrivacyVisibility::On {
            user_privacy.visibility = user::UserPrivacyVisibility::On;
        }

        user::User {
            discord_id: self.db_data.discord_id.to_string(),
            renshuu_api_key: self.db_data.renshuu_api_key.to_string(),
            privacy: user_privacy,
            cache: self.db_data.cache.to_string(),
            refresh_timeout: self.db_data.refresh_timeout,
            last_caching_timestamp: self.db_data.last_caching_timestamp,
        }
    }

    #[warn(dead_code)]
    pub fn correct_adventure_level(&mut self) -> &Vec<i32> {
        let mut levels: Vec<i32> = Vec::new();
        let mut current: i32 = self.profile.adventure_level().parse::<i32>().unwrap();

        while current > 100 {
            current += 1;
            levels.push(100);
            current -= 100;
        }
        levels.push(current);
        self.levels_vec = levels;
        &self.levels_vec
    }

    pub fn format_levels_vector(&self) -> String {
        let formatted_strings: Vec<String> = self
            .levels_vec
            .iter()
            .map(|&num| format!("**{}**", num))
            .collect();

        formatted_strings.join(", ")
    }

    pub fn studied_vocab(&self) -> String {
        let mut vocab: String = String::new();
        let percs_vocab = &self.profile.level_progress_percs().vocab;

        writeln!(vocab, "Today: **{}**", self.profile.studied().today_vocab()).unwrap();
        writeln!(vocab, "N5: **{}**%", percs_vocab.n5).unwrap();
        writeln!(vocab, "N4: **{}**%", percs_vocab.n4).unwrap();
        writeln!(vocab, "N3: **{}**%", percs_vocab.n3).unwrap();
        writeln!(vocab, "N2: **{}**%", percs_vocab.n2).unwrap();
        write!(vocab, "N1: **{}**%", percs_vocab.n1).unwrap();

        vocab
    }

    pub fn studied_kanji(&self) -> String {
        let mut kanji: String = String::new();
        let percs_kanji: &api_profile::ApiProfileProgressPercsJLPT = &self.profile.level_progress_percs().kanji;

        writeln!(kanji, "Today: **{}**", self.profile.studied().today_vocab()).unwrap();
        writeln!(kanji, "N5: **{}**%", percs_kanji.n5).unwrap();
        writeln!(kanji, "N4: **{}**%", percs_kanji.n4).unwrap();
        writeln!(kanji, "N3: **{}**%", percs_kanji.n3).unwrap();
        writeln!(kanji, "N2: **{}**%", percs_kanji.n2).unwrap();
        write!(kanji, "N1: **{}**%", percs_kanji.n1).unwrap();

        kanji
    }

    pub fn studied_grammar(&self) -> String {
        let mut grammar: String = String::new();
        let percs_grammar: &api_profile::ApiProfileProgressPercsJLPT = &self.profile.level_progress_percs().grammar;

        writeln!(grammar, "Today: **{}**", self.profile.studied().today_vocab()).unwrap();
        writeln!(grammar, "N5: **{}**%", percs_grammar.n5).unwrap();
        writeln!(grammar, "N4: **{}**%", percs_grammar.n4).unwrap();
        writeln!(grammar, "N3: **{}**%", percs_grammar.n3).unwrap();
        writeln!(grammar, "N2: **{}**%", percs_grammar.n2).unwrap();
        write!(grammar, "N1: **{}**%", percs_grammar.n1).unwrap();

        grammar
    }

    pub fn studied_sent(&self) -> String {
        let mut sent: String = String::new();
        let percs_sent: &api_profile::ApiProfileProgressPercsJLPT = &self.profile.level_progress_percs().sent;

        writeln!(sent, "Today: **{}**", self.profile.studied().today_vocab()).unwrap();
        writeln!(sent, "N5: **{}**%", percs_sent.n5).unwrap();
        writeln!(sent, "N4: **{}**%", percs_sent.n4).unwrap();
        writeln!(sent, "N3: **{}**%", percs_sent.n3).unwrap();
        writeln!(sent, "N2: **{}**%", percs_sent.n2).unwrap();
        write!(sent, "N1: **{}**%", percs_sent.n1).unwrap();

        sent
    }

    pub fn studied_conj(&self) -> String {
        let mut conj: String = String::new();
        writeln!(conj, "Today: **{}**", self.profile.studied().today_conj()).unwrap();

        conj
    }

    pub fn studied_aconj(&self) -> String {
        let mut aconj: String = String::new();
        writeln!(aconj, "Today: **{}**", self.profile.studied().today_aconj()).unwrap();

        aconj
    }
}
