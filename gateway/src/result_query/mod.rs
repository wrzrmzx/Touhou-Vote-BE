
use juniper::graphql_value;

use juniper::FieldResult;

use crate::common::*;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;

// ------------------------------------------------
// REST Schemas
// ------------------------------------------------

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="单个问卷项目结果")]
pub struct SinglePaperResult {
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="问卷项目结果")]
pub struct PaperResults {
    /// 所有问卷项目结果
    pub results: Vec<SinglePaperResult>,
    /// 使用的过滤器
    pub filter_condtions: Option<FilterConditions>
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="单个过滤器条件")]
pub struct SingleFilterCondition {
    /// 来源
    pub section: VoteSection, 
    /// 条件
    pub condition: FilterConditionOp, 
    /// 左值
    pub lhs: String, 
    /// 右值
    pub rhs: String 
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="过滤器条件（所有条件与）")]
pub struct FilterConditions { // 
    pub conditions: Vec<SingleFilterCondition>
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="投票理由集合")]
pub struct Reasons {
    pub reasons: Vec<String>
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="投票时间趋势，返回开始到结束每小时的票数")]
pub struct Trends {
    /// 新增票数
    pub vote_inc: Option<Vec<i32>>,
    /// 减少票数
    pub vote_dec: Option<Vec<i32>>,
    /// 新增本命
    pub first_inc: Option<Vec<i32>>,
    /// 减少本命
    pub first_dev: Option<Vec<i32>>,
    /// 总票数
    pub vote_cum: Option<Vec<i32>>,
    /// 总本命数
    pub first_cum: Option<Vec<i32>>,
    /// 开始时间
    pub from_date: DateTime<Utc>,
    /// 结束时间
    pub to_date: DateTime<Utc>
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="单个人物的结果")]
pub struct SingleCharacterResult {
    /// 名字
    pub name: String,
    /// 排名
    pub rank: i32,    
    /// 票数
    pub vote_count: i32, 
    /// 本名加权后票数
    pub vote_count_weighted: i32, 
    /// 票数占比
    pub vote_ratio: f64, 
    /// 本名票数
    pub vote_first_count: i32, 
    /// 本名占比
    pub vote_first_ratio: f64, 
    /// 男性票数
    pub male_count: i32, 
    /// 男性占比
    pub male_ratio: f64, 
    /// 女性票数
    pub female_count: i32, 
    /// 女性占比
    pub female_ratio: f64, 
    /// 前一次排名
    pub rank_prev: Option<i32>, 
    /// 投票理由
    pub reasons: Option<Reasons>, 
    /// 票数趋势
    pub trends: Option<Trends>,
    /// 根据投票人物过滤的问卷
    pub papers: Option<PaperResults>,
    /// 同投率
    pub cooccurrence_ratio: Option<f64> 
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CharacterRankResult {
    /// 所有人物结果
    pub characters: Vec<SingleCharacterResult>,
    /// 使用的过滤器
    pub filter_condtions: Option<FilterConditions>
}


// ------------------------------------------------
// GQL Schemas
// ------------------------------------------------

// see REST schemas

// ------------------------------------------------
// Root Quries
// ------------------------------------------------

use crate::services::*;

// pub fn character_rank_impl(filters: Option<FilterConditions>) -> FieldResult<CharacterRankResult> {

// }

pub fn character_reasons_impl(name: String) -> FieldResult<Reasons> {
    Ok(Reasons { reasons: vec![] })
}
