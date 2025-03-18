use crate::datasets::*;
use crate::env::Config;
use crate::search::Client;
use anyhow::{Error, Result};
use url::Url;

pub async fn run_cli() -> Result<(), Error> {
    let config = Config::new()?;

    let json_path = config.json_path;
    let primary_key = config.json_key;

    //   let player_data: PlayerData = LoadData::from_json_file_to_struct(&json_path)?;
    let player_data: PlayerReductionData = LoadData::from_json_file_to_struct(&json_path)?;
    //   let json_files = LoadData::from_json_file(&json_path, &json_key)?;
    //   let ndjson_string = LoadData::from_vec_json_to_ndjson(&json_files);

    //   println!("JSON files: {:?}", json_files);

    let meilisearch_host = Url::parse(&config.meilisearch_host)?;
    let meilisearch_token = config.meilisearch_token;

    let client = Client::connect(&meilisearch_host, &meilisearch_token)?;
    // let datasets = generate_movies();
    //
    // let index_name = "movies";
    // let primary_key = "id";
    //   let created_index = client
    //       .create_index(index_name, &datasets, primary_key)
    //       .await?;

    let index_name = "players_reduction";
    let delete_result = client.delete_index(index_name).await?;
       println!("Index deleted: {:?}", delete_result);
    let created_index = client
        .create_index(index_name, &player_data.data, &primary_key)
        .await?;
    println!("Index created: {:?}", created_index);

    //   println!("Index created: {:?}", created_index);

    // let title = "caorl";
    // let search_results: SearchData<Movie> = client.search_title(index_name, title).await?;
    //
    // println!("Search results: {:?}", search_results);

    Ok(())
}
