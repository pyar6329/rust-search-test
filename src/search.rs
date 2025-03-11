use crate::datasets::LoadData;
use crate::traits::Parallelism;
use anyhow::{Error, Result};
use meilisearch_sdk::{
    client::Client as MeilisearchClient, search::SearchResults as MeilisearchSearchResults,
    task_info::TaskInfo,
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value as JsonValue;
use std::ops::Deref;
use tokio::sync::oneshot;
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

pub trait GetAttributes {
    fn get_filter_attributes() -> Vec<String>;
    fn get_sort_attributes() -> Vec<String>;
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
        T: Serialize + Parallelism + GetAttributes,
    {
        let index = self.deref().index(index_name);

        let _ = index
            .set_filterable_attributes(&T::get_filter_attributes())
            .await;

        let _ = index
            .set_sortable_attributes(&T::get_sort_attributes())
            .await;

        let result = index.add_documents(documents, Some(primary_key)).await?;

        Ok(result)
    }

    // It cannot use because Meilisearch doesn't support JSONValue
    // pub async fn create_index_from_json(
    //     &self,
    //     index_name: &str,
    //     documents: &str,
    //     primary_key: &str,
    // ) -> Result<TaskInfo, Error> {
    //     let index = self.deref().index(index_name);
    //
    //     let documents_string = documents.to_string();
    //     let primary_key_string = primary_key.to_string();
    //
    //     let (tx, rx) = oneshot::channel();
    //
    //     tokio::spawn(async move {
    //         let bytes = Box::leak(documents_string.into_boxed_str()).as_bytes();
    //         let result = index
    //             .add_documents_ndjson(bytes, Some(&primary_key_string))
    //             .await;
    //
    //         let _ = tx.send(result);
    //     });
    //
    //     let result = rx.await??;
    //     Ok(result)
    // }

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

    pub async fn delete_index(&self, index_name: &str) -> Result<TaskInfo, Error> {
        let index = self.deref().index(index_name);
        let result = index.delete().await?;

        Ok(result)
    }
}
