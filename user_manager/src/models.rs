
use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub email: String,
    pub created_at: DateTime,
    // missing crypto
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoteToken {
    pub _id: ObjectId,
    pub token: String, // 唯一的随机字符串
    pub email: String, // 对应的邮箱
    pub user_id: Option<ObjectId>, // 如果之前有账号，则关联之前的账号
    pub vote_id: u32 // 第几届
}
