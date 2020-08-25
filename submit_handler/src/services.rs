use bson::{doc, Document, oid::ObjectId};
use mongodb::{results::InsertOneResult, Collection, Database};

use crate::models;
use crate::shared::Error;

#[derive(Clone)]
pub struct SubmitService {
    pub character_coll: Collection,
    pub music_coll: Collection,
    pub cp_coll: Collection,
    pub work_coll: Collection,
    pub paper_coll: Collection
}

impl SubmitService {
    pub fn new(db: Database) -> SubmitService {
        SubmitService { 
            character_coll: db.collection("character"),
            music_coll: db.collection("music"),
            cp_coll: db.collection("cp"),
            work_coll: db.collection("work"),
            paper_coll: db.collection("paper")
        }
    }

    pub async fn submitCharcater(&self, verified_data: models::CharacterSubmitRest) -> Result<ObjectId, Error> {
        let mut attempts: u32 = 0;
        while attempts < 3 {
            match self.character_coll.insert_one(bson::to_document(&verified_data).unwrap(), None).await {
                Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
                Err(_) => attempts += 1,
            };
        };
        Err(Error::new("failed to insert into character collection"))
    }

    pub async fn submitMusic(&self, verified_data: models::MusicSubmitRest) -> Result<ObjectId, Error> {
        let mut attempts: u32 = 0;
        while attempts < 3 {
            match self.music_coll.insert_one(bson::to_document(&verified_data).unwrap(), None).await {
                Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
                Err(_) => attempts += 1,
            };
        };
        Err(Error::new("failed to insert into music collection"))
    }
    
    pub async fn submitCP(&self, verified_data: models::CPSubmitRest) -> Result<ObjectId, Error> {
        let mut attempts: u32 = 0;
        while attempts < 3 {
            match self.cp_coll.insert_one(bson::to_document(&verified_data).unwrap(), None).await {
                Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
                Err(_) => attempts += 1,
            };
        };
        Err(Error::new("failed to insert into CP collection"))
    }

    pub async fn submitWork(&self, verified_data: models::WorkSubmitRest) -> Result<ObjectId, Error> {
        let mut attempts: u32 = 0;
        while attempts < 3 {
            match self.work_coll.insert_one(bson::to_document(&verified_data).unwrap(), None).await {
                Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
                Err(_) => attempts += 1,
            };
        };
        Err(Error::new("failed to insert into work collection"))
    }

    pub async fn submitPaper(&self, verified_data: models::PaperSubmitRest) -> Result<ObjectId, Error> {
        let mut attempts: u32 = 0;
        while attempts < 3 {
            match self.paper_coll.insert_one(bson::to_document(&verified_data).unwrap(), None).await {
                Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
                Err(_) => attempts += 1,
            };
        };
        Err(Error::new("failed to insert into paper collection"))
    }
}
