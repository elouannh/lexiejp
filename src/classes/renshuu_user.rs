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
    pub async fn new(ctx: &ctx::Context<'_>, renshuu_api_key: &String) -> Self {
        let mut user_exists: bool = true;

        if renshuu_api_key == &String::from("none") {
            user_exists = false;
        }

        let db_data: user::User =
            user::User::default(&ctx.author().id.to_string(), renshuu_api_key);
        let mut content: String = db_data.cache.to_owned();

        if content.len() < 2 {
            let rest_agent: rest_agent::RestAgent = rest_agent::RestAgent::new(renshuu_api_key);
            content = rest_agent
                .get_method("https://api.renshuu.org/v1/profile")
                .await
                .unwrap();
        }

        let profile: api_profile::ProfileEnum =
            api_profile::ProfileEnum::create(&content).expect("Invalid user.");
        let is_beginner: bool = profile.is_beginner();
        let self_data = RenshuuUser {
            db_data,
            user: ctx.author().to_owned(),
            content,
            profile,
            levels_vec: vec![],
            is_beginner,
            already_exists: user_exists,
        };
        self_data
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
        vocab.push_str(
            &format!("Today: **{}**\n", self.profile.studied().today_vocab()).to_string(),
        );
        vocab.push_str(
            &format!(
                "N5: **{}**%\n",
                self.profile.level_progress_percs().vocab.n5
            )
            .to_string(),
        );
        vocab.push_str(
            &format!(
                "N4: **{}**%\n",
                self.profile.level_progress_percs().vocab.n4
            )
            .to_string(),
        );
        vocab.push_str(
            &format!(
                "N3: **{}**%\n",
                self.profile.level_progress_percs().vocab.n3
            )
            .to_string(),
        );
        vocab.push_str(
            &format!(
                "N2: **{}**%\n",
                self.profile.level_progress_percs().vocab.n2
            )
            .to_string(),
        );
        vocab.push_str(
            &format!("N1: **{}**%", self.profile.level_progress_percs().vocab.n1).to_string(),
        );

        vocab
    }

    pub fn studied_kanji(&self) -> String {
        let mut kanji: String = String::new();
        kanji.push_str(
            &format!("Today: **{}**\n", self.profile.studied().today_kanji()).to_string(),
        );
        kanji.push_str(
            &format!(
                "N5: **{}**%\n",
                self.profile.level_progress_percs().kanji.n5
            )
            .to_string(),
        );
        kanji.push_str(
            &format!(
                "N4: **{}**%\n",
                self.profile.level_progress_percs().kanji.n4
            )
            .to_string(),
        );
        kanji.push_str(
            &format!(
                "N3: **{}**%\n",
                self.profile.level_progress_percs().kanji.n3
            )
            .to_string(),
        );
        kanji.push_str(
            &format!(
                "N2: **{}**%\n",
                self.profile.level_progress_percs().kanji.n2
            )
            .to_string(),
        );
        kanji.push_str(
            &format!("N1: **{}**%", self.profile.level_progress_percs().kanji.n1).to_string(),
        );

        kanji
    }

    pub fn studied_grammar(&self) -> String {
        let mut grammar: String = String::new();
        grammar.push_str(
            &format!("Today: **{}**\n", self.profile.studied().today_grammar()).to_string(),
        );
        grammar.push_str(
            &format!(
                "N5: **{}**%\n",
                self.profile.level_progress_percs().grammar.n5
            )
            .to_string(),
        );
        grammar.push_str(
            &format!(
                "N4: **{}**%\n",
                self.profile.level_progress_percs().grammar.n4
            )
            .to_string(),
        );
        grammar.push_str(
            &format!(
                "N3: **{}**%\n",
                self.profile.level_progress_percs().grammar.n3
            )
            .to_string(),
        );
        grammar.push_str(
            &format!(
                "N2: **{}**%\n",
                self.profile.level_progress_percs().grammar.n2
            )
            .to_string(),
        );
        grammar.push_str(
            &format!(
                "N1: **{}**%",
                self.profile.level_progress_percs().grammar.n1
            )
            .to_string(),
        );

        grammar
    }

    pub fn studied_sent(&self) -> String {
        let mut sent: String = String::new();
        sent.push_str(&format!("Today: **{}**\n", self.profile.studied().today_sent()).to_string());
        sent.push_str(
            &format!("N5: **{}**%\n", self.profile.level_progress_percs().sent.n5).to_string(),
        );
        sent.push_str(
            &format!("N4: **{}**%\n", self.profile.level_progress_percs().sent.n4).to_string(),
        );
        sent.push_str(
            &format!("N3: **{}**%\n", self.profile.level_progress_percs().sent.n3).to_string(),
        );
        sent.push_str(
            &format!("N2: **{}**%\n", self.profile.level_progress_percs().sent.n2).to_string(),
        );
        sent.push_str(
            &format!("N1: **{}**%", self.profile.level_progress_percs().sent.n1).to_string(),
        );

        sent
    }

    pub fn studied_conj(&self) -> String {
        let mut conj: String = String::new();
        conj.push_str(&format!("Today: **{}**\n", self.profile.studied().today_conj()).to_string());

        conj
    }

    pub fn studied_aconj(&self) -> String {
        let mut aconj: String = String::new();
        aconj.push_str(
            &format!("Today: **{}**\n", self.profile.studied().today_aconj()).to_string(),
        );

        aconj
    }
}
