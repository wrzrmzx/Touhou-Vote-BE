
use juniper::graphql_value;

use juniper::FieldResult;

use crate::common::Error;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;

// ------------------------------------------------
// REST Schemas
// ------------------------------------------------

#[derive(Clone, Serialize, Deserialize)]
pub struct CharacterSubmitRest {
	pub user_id: String,
	pub characters: Vec<CharacterSubmit>,
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
	pub user_id: String,
	pub characters: Vec<CharacterSubmit>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single cp submit")]
pub struct CPSubmit {
	pub name_a: String,
	pub name_b: String,
	pub active: String,
	pub reason: Option<String>,
	pub rank: i32
}

#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="CP submit")]
pub struct NewCPSubmit {
	pub user_id: String,
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
	pub user_id: String,
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
	pub user_id: String,
	pub work: Vec<WorkSubmit>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single paper submit")]
pub struct PaperSubmit {
	pub id: String,
	pub answer: String
}

#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="Paper submit")]
pub struct NewPaperSubmit {
	pub user_id: String,
	pub papers: Vec<PaperSubmit>
}

// ------------------------------------------------
// Root Quries
// ------------------------------------------------

pub fn submitCharcaterVote_impl(content: &NewCharacterSubmit) -> FieldResult<NewCharacterSubmit> {
	Ok(content.clone())
}

pub fn submitMusicVote_impl(content: &NewMusicSubmit) -> FieldResult<NewMusicSubmit> {
	Ok(content.clone())
}

pub fn submitCPVote_impl(content: &NewCPSubmit) -> FieldResult<NewCPSubmit> {
	Ok(content.clone())
}

pub fn submitWorkVote_impl(content: &NewWorkSubmit) -> FieldResult<NewWorkSubmit> {
	Ok(content.clone())
}

pub fn submitPaperVote_impl(content: &NewPaperSubmit) -> FieldResult<NewPaperSubmit> {
	Ok(content.clone())
}
