use crate::traits::Parallelism;
use anyhow::{Error, Result};
use meilisearch_sdk::{
    client::Client as MeilisearchClient, search::SearchResults as MeilisearchSearchResults,
    task_info::TaskInfo,
};
use serde::{Serialize, de::DeserializeOwned};
use std::ops::Deref;
use url::Url;

type PageNum = u64;
type TotalHitNum = u64;
type TotalPageNum = u64;

#[derive(Debug, Clone)]
pub struct Client(MeilisearchClient);

#[derive(Debug, Clone)]
pub struct SearchData<T> {
    pub results: Vec<T>,
    pub page_num: PageNum,
    pub total_hits: TotalHitNum,
    pub total_pages: TotalPageNum,
}

impl Deref for Client {
    type Target = MeilisearchClient;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Client {
    pub fn connect(url: &Url, token: &str) -> Result<Self, Error> {
        let client = MeilisearchClient::new(url.to_string(), Some(token))?;
        Ok(Self(client))
    }

    pub async fn create_index<T>(
        &self,
        index_name: &str,
        documents: &[T],
        primary_key: &str,
    ) -> Result<TaskInfo, Error>
    where
        T: Serialize + Parallelism,
    {
        let index = self.deref().index(index_name);
        let result = index.add_documents(documents, Some(primary_key)).await?;
        Ok(result)
    }

    pub async fn search_title<T>(
        &self,
        index_name: &str,
        title: &str,
    ) -> Result<SearchData<T>, Error>
    where
        T: DeserializeOwned + Parallelism + Clone,
    {
        let result: MeilisearchSearchResults<T> = self
            .deref()
            .index(index_name)
            .search()
            .with_query(title)
            .execute()
            .await?;

        let search_result = SearchData {
            results: result
                .hits
                .iter()
                .map(|hit| hit.result.to_owned())
                .collect(),
            page_num: result.page.unwrap_or(0) as u64,
            total_hits: result.total_hits.unwrap_or(0) as u64,
            total_pages: result.total_pages.unwrap_or(0) as u64,
        };

        Ok(search_result)
    }
}
