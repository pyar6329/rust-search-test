use super::*;

pub struct LoadData;

impl LoadData {
    pub fn from_json_file<P>(path: P, key: &str) -> Result<Vec<JsonValue>, Error>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let all_data: JsonValue = serde_json::from_reader(reader)?;
        let data_value = all_data
            .get(key)
            .ok_or(anyhow!("this json cannot have this key"))?;

        let vec_data: Vec<JsonValue> = data_value
            .as_array()
            .map(|x| x.to_owned())
            .ok_or(anyhow!("this json cannot be converted to array"))?;

        Ok(vec_data)
    }

    pub fn from_json_file_to_struct<P, T>(path: P) -> Result<T, Error>
    where
        P: AsRef<Path>,
        T: DeserializeOwned,
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let all_data: T = serde_json::from_reader(reader)?;

        Ok(all_data)
    }

    pub fn from_vec_json_to_ndjson(vec_data: &[JsonValue]) -> String {
        let mut ndjson = String::new();
        for data in vec_data {
            let json_line = serde_json::to_string(&data).unwrap_or_default();
            if json_line.is_empty() {
                continue;
            }

            ndjson.push_str(&json_line);
            ndjson.push('\n');
        }
        ndjson
    }
}
