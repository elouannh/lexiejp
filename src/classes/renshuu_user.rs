use crate::structs;

pub struct RenshuuUser
{
	pub user: serenity::all::User,
	pub content: String,
	pub profile: structs::api_profile::ApiProfile,
	pub levels_vec: Vec<u32>,
}

impl RenshuuUser
{
	pub fn new(user: serenity::all::User, content: String) -> Self
	{
		let profile: Result<structs::api_profile::ApiProfile, serde_json::Error> =
			structs::api_profile::ApiProfile::try_from(serde_json::from_str(&content));

		let mut self_data = RenshuuUser
		{
			user,
			content,
			profile: profile.unwrap(),
			levels_vec: vec![],
		};
		self_data.correct_adventure_level();

		self_data
	}

	pub fn correct_adventure_level(&mut self) -> &Vec<u32>
	{
		let mut levels: Vec<u32> = Vec::new();
		let mut current: u32 = self.profile.adventure_level.parse::<u32>().unwrap();

		while current > 100
		{
			current += 1;
			levels.push(100);
			current -= 100;
		}
		levels.push(current);
		self.levels_vec = levels;
		&self.levels_vec
	}

	pub fn format_levels_vector(&self) -> String
	{
		let formatted_strings: Vec<String> = self.levels_vec.iter()
			.map(|&num| format!("**{}**", num))
			.collect();

		formatted_strings.join(", ")
	}
}