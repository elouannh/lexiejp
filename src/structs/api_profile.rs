#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiProfileStudied
{
	pub today_all: u32,
	pub today_vocab: u32,
	pub today_grammar: u32,
	pub today_kanji: u32,
	pub today_sent: u32,
	pub today_conj: u32,
	pub today_aconj: u32,
	pub total: u32,
	pub total_vocab: u32,
	pub total_grammar: u32,
	pub total_kanji: u32,
	pub total_sent: u32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiProfileProgressPercsJLPT
{
	pub n1: u32,
	pub n2: u32,
	pub n3: u32,
	pub n4: u32,
	pub n5: u32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiProfileProgressPercs
{
	pub vocab: ApiProfileProgressPercsJLPT,
	pub kanji: ApiProfileProgressPercsJLPT,
	pub grammar: ApiProfileProgressPercsJLPT,
	pub sent: ApiProfileProgressPercsJLPT
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiProfileStreaksEachData
{
	pub correct_in_a_row: String,
	pub correct_in_a_row_alltime: String,
	pub days_studied_in_a_row: String,
	pub days_studied_in_a_row_alltime: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiProfileStreaks
{
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
	pub streaks: ApiProfileStreaks
}

impl TryFrom<Result<ApiProfile, serde_json::error::Error>> for ApiProfile {
	type Error = serde_json::error::Error;

	fn try_from(value: Result<ApiProfile, serde_json::error::Error>) -> Result<Self, serde_json::error::Error> {
		match value {
			Ok(profile) => Ok(profile),
			Err(e) => Err(e),
		}
	}
}