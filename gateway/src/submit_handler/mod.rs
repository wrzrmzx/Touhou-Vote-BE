
use juniper::graphql_value;

use juniper::FieldResult;

use crate::common::{Error, UserVerifyResult, PostResult};

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;

// ------------------------------------------------
// REST Schemas
// ------------------------------------------------

#[derive(Clone, Serialize, Deserialize)]
pub struct CharacterSubmitRest {
	pub vote_token: String,
	pub characters: Vec<CharacterSubmit>,
	pub created_at: DateTime<Utc>,
	pub user_ip: String // 防刷票
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MusicSubmitRest {
	pub vote_token: String,
	pub music: Vec<MusicSubmit>,
	pub created_at: DateTime<Utc>,
	pub user_ip: String // 防刷票
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WorkSubmitRest {
	pub vote_token: String,
	pub works: Vec<WorkSubmit>,
	pub created_at: DateTime<Utc>,
	pub user_ip: String // 防刷票
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CPSubmitRest {
	pub vote_token: String,
	pub cps: Vec<CPSubmit>,
	pub created_at: DateTime<Utc>,
	pub user_ip: String // 防刷票
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PaperSubmitRest {
	pub vote_token: String,
	pub papers: Vec<PaperSubmit>,
	pub created_at: DateTime<Utc>,
	pub user_ip: String // 防刷票
}

// ------------------------------------------------
// GQL Schemas
// ------------------------------------------------

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single character submit")]
pub struct CharacterSubmit {
	pub name: String,
	pub reason: Option<String>,
	pub rank: i32
}

#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="Character submit")]
pub struct NewCharacterSubmit {
	pub vote_token: String,
	pub characters: Vec<CharacterSubmit>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single cp submit")]
pub struct CPSubmit {
	pub name_a: String,
	pub name_b: String,
	pub name_c: Option<String>,
	pub active: Option<String>,
	pub reason: Option<String>,
	pub rank: i32
}

#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="CP submit")]
pub struct NewCPSubmit {
	pub vote_token: String,
	pub cps: Vec<CPSubmit>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single music submit")]
pub struct MusicSubmit {
	pub name: String,
	pub reason: Option<String>,
	pub rank: i32
}

#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="Music submit")]
pub struct NewMusicSubmit {
	pub vote_token: String,
	pub musics: Vec<MusicSubmit>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single work submit")]
pub struct WorkSubmit {
	pub name: String,
	pub reason: Option<String>,
	pub rank: i32
}

#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="Work submit")]
pub struct NewWorkSubmit {
	pub vote_token: String,
	pub work: Vec<WorkSubmit>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single paper submit")]
pub struct PaperSubmit {
	pub id: String,
	/// 答案
	pub answer: String
}

#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="Paper submit")]
pub struct NewPaperSubmit {
	pub vote_token: String,
	pub papers: Vec<PaperSubmit>
}

// ------------------------------------------------
// Root Quries
// ------------------------------------------------

use crate::services::*;

pub fn submitCharacterVote_impl(content: &NewCharacterSubmit) -> FieldResult<PostResult> {
	// verify vote_token
	getJSON!(UserVerifyResult, format!("http://{}/v1/verify/{}", USER_MANAGER, content.vote_token));
	let submit_json = CharacterSubmitRest {
		vote_token: content.vote_token.clone(),
		characters: content.characters.clone(),
		created_at: Utc::now(),
		user_ip: "test".into() // TODO: how do I get IP using GQL?
	};
	postJSON!(PostResult, format!("http://{}/v1/character/", SUBMIT_HANDLER), submit_json);
	Ok(PostResult::new())
}

pub fn submitMusicVote_impl(content: &NewMusicSubmit) -> FieldResult<PostResult> {
	Ok(PostResult::new())
}

pub fn submitCPVote_impl(content: &NewCPSubmit) -> FieldResult<PostResult> {
	Ok(PostResult::new())
}

pub fn submitWorkVote_impl(content: &NewWorkSubmit) -> FieldResult<PostResult> {
	Ok(PostResult::new())
}

pub fn submitPaperVote_impl(content: &NewPaperSubmit) -> FieldResult<PostResult> {
	Ok(PostResult::new())
}
