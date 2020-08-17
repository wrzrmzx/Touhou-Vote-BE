use juniper::FieldResult;
use juniper::RootNode;

#[derive(Clone)]
pub struct Context {
}

impl juniper::Context for Context {}

use crate::common::PostResult;

#[path="submit_handler/mod.rs"]
mod submit_handler;

#[path="result_query/mod.rs"]
mod result_query;

use submit_handler::{NewCharacterSubmit, NewMusicSubmit, NewWorkSubmit, NewCPSubmit, NewPaperSubmit};

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {

}


pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
	
	fn apiVersion() -> &str {
		"1.0"
	}

	// ------------------------------------------------
	//     submit_handler
	// ------------------------------------------------

	/// Character
	fn submitCharacterVote(content: NewCharacterSubmit) -> FieldResult<PostResult> {
		submit_handler::submitCharacterVote_impl(&content)
	}

	/// music
	fn submitMusicVote(content: NewMusicSubmit) -> FieldResult<PostResult> {
	   submit_handler::submitMusicVote_impl(&content)
	}

	/// work
	fn submitWorkVote(content: NewWorkSubmit) -> FieldResult<PostResult> {
		submit_handler::submitWorkVote_impl(&content)
	}

	/// CP
	fn submitCPVote(content: NewCPSubmit) -> FieldResult<PostResult> {
		submit_handler::submitCPVote_impl(&content)
	}

	/// paper
	fn submitPaperVote(content: NewPaperSubmit) -> FieldResult<PostResult> {
		submit_handler::submitPaperVote_impl(&content)
	}
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
	Schema::new(QueryRoot {}, MutationRoot {})
}
