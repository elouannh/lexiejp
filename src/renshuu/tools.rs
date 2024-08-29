use crate::structs;

pub fn studied_vocab(profile: &structs::api_profile::ApiProfile) -> String
{
	let mut vocab: String = String::new();
	vocab.push_str(&format!("Today: **{}**\n", profile.studied.today_vocab).to_string());
	vocab.push_str(&format!("N5: **{}**%\n", profile.level_progress_percs.vocab.n5).to_string());
	vocab.push_str(&format!("N4: **{}**%\n", profile.level_progress_percs.vocab.n4).to_string());
	vocab.push_str(&format!("N3: **{}**%\n", profile.level_progress_percs.vocab.n3).to_string());
	vocab.push_str(&format!("N2: **{}**%\n", profile.level_progress_percs.vocab.n2).to_string());
	vocab.push_str(&format!("N1: **{}**%", profile.level_progress_percs.vocab.n1).to_string());

	vocab
}

pub fn studied_kanji(profile: &structs::api_profile::ApiProfile) -> String
{
	let mut kanji: String = String::new();
	kanji.push_str(&format!("Today: **{}**\n", profile.studied.today_kanji).to_string());
	kanji.push_str(&format!("N5: **{}**%\n", profile.level_progress_percs.kanji.n5).to_string());
	kanji.push_str(&format!("N4: **{}**%\n", profile.level_progress_percs.kanji.n4).to_string());
	kanji.push_str(&format!("N3: **{}**%\n", profile.level_progress_percs.kanji.n3).to_string());
	kanji.push_str(&format!("N2: **{}**%\n", profile.level_progress_percs.kanji.n2).to_string());
	kanji.push_str(&format!("N1: **{}**%", profile.level_progress_percs.kanji.n1).to_string());

	kanji
}

pub fn studied_grammar(profile: &structs::api_profile::ApiProfile) -> String
{
	let mut grammar: String = String::new();
	grammar.push_str(&format!("Today: **{}**\n", profile.studied.today_grammar).to_string());
	grammar.push_str(&format!("N5: **{}**%\n", profile.level_progress_percs.grammar.n5).to_string());
	grammar.push_str(&format!("N4: **{}**%\n", profile.level_progress_percs.grammar.n4).to_string());
	grammar.push_str(&format!("N3: **{}**%\n", profile.level_progress_percs.grammar.n3).to_string());
	grammar.push_str(&format!("N2: **{}**%\n", profile.level_progress_percs.grammar.n2).to_string());
	grammar.push_str(&format!("N1: **{}**%", profile.level_progress_percs.grammar.n1).to_string());

	grammar
}

pub fn studied_sent(profile: &structs::api_profile::ApiProfile) -> String
{
	let mut sent: String = String::new();
	sent.push_str(&format!("Today: **{}**\n", profile.studied.today_sent).to_string());
	sent.push_str(&format!("N5: **{}**%\n", profile.level_progress_percs.sent.n5).to_string());
	sent.push_str(&format!("N4: **{}**%\n", profile.level_progress_percs.sent.n4).to_string());
	sent.push_str(&format!("N3: **{}**%\n", profile.level_progress_percs.sent.n3).to_string());
	sent.push_str(&format!("N2: **{}**%\n", profile.level_progress_percs.sent.n2).to_string());
	sent.push_str(&format!("N1: **{}**%", profile.level_progress_percs.sent.n1).to_string());

	sent
}

pub fn studied_conj(profile: &structs::api_profile::ApiProfile) -> String
{
	let mut conj: String = String::new();
	conj.push_str(&format!("Today: **{}**\n", profile.studied.today_conj).to_string());

	conj
}

pub fn studied_aconj(profile: &structs::api_profile::ApiProfile) -> String
{
	let mut aconj: String = String::new();
	aconj.push_str(&format!("Today: **{}**\n", profile.studied.today_aconj).to_string());

	aconj
}