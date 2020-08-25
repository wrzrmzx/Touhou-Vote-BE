
use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CharacterSubmit {
	pub name: String,
	pub reason: Option<String>,
	pub rank: i32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CharacterSubmitRest {
	pub vote_token: String,
	pub characters: Vec<CharacterSubmit>,
	pub created_at: DateTime<Utc>,
	pub user_ip: String // 防刷票
}

// 人物部分
// {
//   vote_id: 2020, 
//   user_id: 用户投票验证码,
//   charcaters:[{
//     name: '',
//     reason: '', // 理由
//     rank: // [0, 6], 0本命
//   }, ...]
// }
// CP部分
// {
//   vote_id: 2020, 
//   user_id:
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
//   user_id:
//   musics:[{
//     name: '',
//     reason: '',
//     rank: // [0, 6], 0本命
//   }, ...]
// }
// 作品部分：
// {
//   vote_id: 2020, 
//   user_id:
//   works:[{
//     name: '',
//     reason: '',
//     rank: // [0, 6], 0本命
//   }, ...]
// }
// 问卷部分
// {
//   vote_id: 2020, 
//   user_id:
//   items:[{
//     item: '' //问卷项代码
//     answer: '' //回答内容
//   }, ...]
// }
