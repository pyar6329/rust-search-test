use crate::datasets::*;
use crate::search::{Client, SearchData};
use anyhow::{Error, Result};

pub async fn run_cli() -> Result<(), Error> {
    let url = "http://localhost:7700".try_into()?;
    let token = "masterKey";

    let client = Client::connect(&url, token)?;
    let datasets = generate_movies();

    let index_name = "movies";
    let primary_key = "id";
    let created_index = client
        .create_index(index_name, &datasets, primary_key)
        .await?;

    println!("Index created: {:?}", created_index);

    let title = "caorl";
    let search_results: SearchData<Movie> = client.search_title(index_name, title).await?;

    println!("Search results: {:?}", search_results);

    Ok(())
}
