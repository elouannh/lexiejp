use crate::classes::renshuu_user;

pub fn profile_embed(
    renshuu_user: &renshuu_user::RenshuuUser,
) -> poise::serenity_prelude::CreateEmbed {
    let embed: poise::serenity_prelude::CreateEmbed = poise::serenity_prelude::CreateEmbed::new();

    embed
        .color(serenity::model::Colour(8544685))
        .author(
            poise::serenity_prelude::CreateEmbedAuthor::new("")
                .name("→ Profile on Renshuu")
                .url(format!(
                    "https://www.renshuu.org/me/{}&profile",
                    &renshuu_user.profile.id()
                )),
        )
        .thumbnail(renshuu_user.profile.kao().to_string())
        .title(format!(
            "Renshuu profile of {}",
            &renshuu_user.profile.real_name()
        ))
        .description(format!(
            "Registered since: __{}__\nCurrent adventure level: __{}__ ({})",
            &renshuu_user.profile.user_length(),
            &renshuu_user.profile.adventure_level(),
            renshuu_user.format_levels_vector()
        ))
        .fields(vec![
            (
                format!(
                    "Vocab ({} known)",
                    &renshuu_user.profile.studied().total_vocab()
                ),
                &renshuu_user.studied_vocab(),
                true,
            ),
            (
                format!(
                    "Kanji ({} known)",
                    &renshuu_user.profile.studied().total_kanji()
                ),
                &renshuu_user.studied_kanji(),
                true,
            ),
            (
                format!(
                    "Grammar ({} known)",
                    &renshuu_user.profile.studied().total_grammar()
                ),
                &renshuu_user.studied_grammar(),
                true,
            ),
            (
                format!(
                    "Sent ({} known)",
                    &renshuu_user.profile.studied().total_sent()
                ),
                &renshuu_user.studied_sent(),
                true,
            ),
            (String::from("Conj"), &renshuu_user.studied_conj(), true),
            (String::from("A-Conj"), &renshuu_user.studied_aconj(), true),
        ])
}

pub fn schedule_embed(content: &str) -> poise::serenity_prelude::CreateEmbed {
    let _ = content;
    let embed: poise::serenity_prelude::CreateEmbed = poise::serenity_prelude::CreateEmbed::new();
    embed
}
