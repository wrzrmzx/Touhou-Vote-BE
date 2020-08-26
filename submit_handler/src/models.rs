
use bson;
use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};

pub trait BsonConvertible where Self: serde::Serialize {
    fn to_bson(&self) -> bson::Document {
        bson::to_document(&self).unwrap()
    }
}

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


#[derive(Clone, Serialize, Deserialize)]
pub struct CharacterSubmit {
	pub name: String,
	pub reason: Option<String>,
	pub rank: i32
}
#[derive(Clone, Serialize, Deserialize)]
pub struct CPSubmit {
	pub name_a: String,
	pub name_b: String,
	pub name_c: Option<String>,
	pub active: Option<String>,
	pub reason: Option<String>,
	pub rank: i32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MusicSubmit {
	pub name: String,
	pub reason: Option<String>,
	pub rank: i32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WorkSubmit {
	pub name: String,
	pub reason: Option<String>,
	pub rank: i32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PaperSubmit {
	pub id: String,
	/// 答案
	pub answer: String
}

// 人物部分
// {
//   vote_id: 2020, 
//   vote_token: 用户投票验证码,
//   charcaters:[{
//     name: '',
//     reason: '', // 理由
//     rank: // [0, 6], 0本命
//   }, ...]
// }
// CP部分
// {
//   vote_id: 2020, 
//   vote_token:
//   cps:[{
//     char1: '',
//     char2: '',
//     active: '', // 主动方
//     reason: '',
//     rank: // [0, 6], 0本命
//   }, ...]
// }
// 音乐部分：
// {
//   vote_id: 2020, 
//   vote_token:
//   musics:[{
//     name: '',
//     reason: '',
//     rank: // [0, 6], 0本命
//   }, ...]
// }
// 作品部分：
// {
//   vote_id: 2020, 
//   vote_token:
//   works:[{
//     name: '',
//     reason: '',
//     rank: // [0, 6], 0本命
//   }, ...]
// }
// 问卷部分
// {
//   vote_id: 2020, 
//   vote_token:
//   items:[{
//     item: '' //问卷项代码
//     answer: '' //回答内容
//   }, ...]
// }
