use crate::
{
	classes,
	renshuu
};

pub fn profile_embed(
	renshuu_user: &classes::renshuu_user::RenshuuUser
) -> poise::serenity_prelude::CreateEmbed
{
	let embed: poise::serenity_prelude::CreateEmbed = poise::serenity_prelude::CreateEmbed::new();

	embed
		.color(serenity::model::Colour(8544685))
		.author(
			poise::serenity_prelude::CreateEmbedAuthor::new("")
				.name("→ Profile on Renshuu")
				.url(format!("https://www.renshuu.org/me/{}&profile", &renshuu_user.profile.id))
		)
		.thumbnail(&renshuu_user.profile.kao)
		.title(format!("Renshuu profile of {}", &renshuu_user.profile.real_name))
		.description(
			format!(
				"Registered since: __{}__\nCurrent adventure level: __{}__ ({})",
				&renshuu_user.profile.user_length,
				&renshuu_user.profile.adventure_level,
				renshuu_user.format_levels_vector()
			)
		)
		.fields(
			vec![
				(
					format!("Vocab ({} known)", &renshuu_user.profile.studied.total_vocab),
					renshuu::tools::studied_vocab(&renshuu_user.profile),
					true
				),
				(
					format!("Kanji ({} known)", &renshuu_user.profile.studied.total_kanji),
					renshuu::tools::studied_kanji(&renshuu_user.profile),
					true
				),
				(
					format!("Grammar ({} known)", &renshuu_user.profile.studied.total_grammar),
					renshuu::tools::studied_grammar(&renshuu_user.profile),
					true
				),
				(
					format!("Sent ({} known)", &renshuu_user.profile.studied.total_sent),
					renshuu::tools::studied_sent(&renshuu_user.profile),
					true
				),
				(
					String::from("Conj"),
					renshuu::tools::studied_conj(&renshuu_user.profile),
					true
				),
				(
					String::from("A-Conj"),
					renshuu::tools::studied_aconj(&renshuu_user.profile),
					true
				)
			]
		)
}

pub fn schedule_embed(content: &str) -> poise::serenity_prelude::CreateEmbed
{
	let _ = content;
	let embed: poise::serenity_prelude::CreateEmbed = poise::serenity_prelude::CreateEmbed::new();
	embed
}