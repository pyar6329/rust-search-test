mod deserializer;

use deserializer::*;

use super::{DecimalValue, Deserialize, GetAttributes, Serialize, default_if_empty};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerReductionData {
    #[serde(rename = "data")]
    pub data: Vec<PlayerReduction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerReduction {
    #[serde(rename = "id", deserialize_with = "string_to_hash_data")]
    pub id: u32,
    // #[serde(rename = "bio")]
    // pub bio: String,
    #[serde(rename = "foot", deserialize_with = "string_to_hash_data")]
    pub foot: u32,
    // #[serde(
    //     rename = "clips",
    //     deserialize_with = "default_if_empty", // default is "" if key doesn't exist
    //     default // default is "" if key exists but value is null
    // )]
    // pub clips: Clips,
    // #[serde(rename = "gsmId")]
    // pub gsm_id: String,
    #[serde(rename = "level", deserialize_with = "string_to_hash_data")]
    pub level: u32,
    #[serde(rename = "gender", deserialize_with = "string_to_hash_data")]
    pub gender: u32,
    #[serde(
        rename = "height",
        deserialize_with = "string_height_to_u8",
        // serialize_with = "u64_as_number"
    )]
    pub height: u8,
    #[serde(rename = "status", deserialize_with = "string_to_hash_data")]
    pub status: u32,
    #[serde(rename = "teamId")]
    pub team_id: String,
    #[serde(rename = "v7Uuid", deserialize_with = "string_to_hash_data")]
    pub v7_uuid: u32,
    #[serde(rename = "weight", deserialize_with = "string_weight_to_u8")]
    pub weight: u8,
    // #[serde(rename = "agentId")]
    // pub agent_id: String,
    // #[serde(rename = "agencyId")]
    // pub agency_id: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    // #[serde(rename = "ogolLink")]
    // pub ogol_link: String,
    #[serde(rename = "agentName")]
    pub agent_name: String,
    #[serde(rename = "birthDate")]
    pub birth_date: String,
    // #[serde(rename = "createdAt")]
    // pub created_at: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    // #[serde(rename = "vimeoLink")]
    // pub vimeo_link: String, // JSON文字列はそのままString型で
    #[serde(rename = "wyscoutId", deserialize_with = "string_to_u64")]
    pub wyscout_id: u64,
    // #[serde(rename = "agencyName")]
    // pub agency_name: String,
    // #[serde(rename = "agentEmail")]
    // pub agent_email: String,
    // #[serde(rename = "agentImage")]
    // pub agent_image: String,
    // #[serde(rename = "agencyEmail")]
    // pub agency_email: String,
    // #[serde(rename = "agencyImage")]
    // pub agency_image: String,
    #[serde(rename = "playerEmail", deserialize_with = "string_to_hash_data")]
    pub player_email: u32,
    // #[serde(rename = "youtubeLink")]
    // pub youtube_link: String, // JSON文字列はそのままString型で
    #[serde(rename = "annualSalary", deserialize_with = "string_cost_to_u64")]
    pub annual_salary: u64,
    // #[serde(rename = "clubContacts")]
    // pub club_contacts: String,
    #[serde(rename = "currentValue", deserialize_with = "string_cost_to_u64")]
    pub current_value: u64,
    #[serde(rename = "imageDataURL", deserialize_with = "string_to_hash_data")]
    pub image_data_url: u32,
    #[serde(rename = "subPositions")]
    pub sub_positions: String,
    #[serde(rename = "mainPositions")]
    pub main_positions: String,
    #[serde(rename = "playingStyles")]
    pub playing_styles: String,
    // #[serde(rename = "agentCellphone")]
    // pub agent_cellphone: String,
    // #[serde(rename = "agencyCellphone")]
    // pub agency_cellphone: String,
    #[serde(rename = "currentClubName")]
    pub current_club_name: String,
    // #[serde(rename = "isCreatedByClub")]
    // pub is_created_by_club: String,
    // #[serde(rename = "playerCellphone")]
    // pub player_cellphone: String,
    // #[serde(rename = "playerInstagram")]
    // pub player_instagram: String,
    #[serde(rename = "willContainLabel")]
    pub will_contain_label: String, // TODO: boolっぽい
    #[serde(
        rename = "minutesPercentage",
        deserialize_with = "default_if_empty", // default is "" if key doesn't exist
        default // default is "" if key exists but value is null
    )]
    pub minutes_percentage: MinutesPercentageReduction,
    #[serde(rename = "transferCondition", deserialize_with = "string_to_hash_data")]
    pub transfer_condition: u32,
    // #[serde(rename = "transfermarktData")]
    // pub transfermarkt_data: String,
    #[serde(rename = "contractExpiration")]
    pub contract_expiration: String,
    #[serde(rename = "currentCountryCode")]
    pub current_country_code: String,
    // #[serde(rename = "playerCurrentState")]
    // pub player_current_state: String,
    // #[serde(rename = "seeCurrentClubLink")]
    // pub see_current_club_link: String,
    // #[serde(rename = "transfermarketLink")]
    // pub transfermarket_link: String,
    // #[serde(rename = "transferFeeInDollar")]
    // pub transfer_fee_in_dollar: String,
    // #[serde(rename = "lastConditionUpdateAt")]
    // pub last_condition_update_at: String,
    #[serde(rename = "nationalityCountryCode")]
    pub nationality_country_code: String,
    // #[serde(rename = "playerSecondNationality")]
    // pub player_second_nationality: String,
    // #[serde(rename = "playerPossiblesNationality")]
    // pub player_possibles_nationality: String,
    #[serde(rename = "PoolPlayerTmpId")]
    pub user_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clips(String);

impl Default for Clips {
    fn default() -> Self {
        Self("".to_string())
    }
}

impl GetAttributes for PlayerReduction {
    fn get_filter_attributes() -> Vec<String> {
        let data = [
            "bio",
            "foot",
            "level",
            "gender",
            "height",
            "v7Uuid",
            "fullName",
            "status",
            "annualSalary",
            "currentValue",
            "currentClubName",
            "minutesPercentage",
            "transferCondition",
            "currentCountryCode",
            "nationalityCountryCode",
            "PoolPlayerTmpId",
        ];

        data.iter().map(|x| x.to_string()).collect()
    }

    fn get_sort_attributes() -> Vec<String> {
        let data = [
            "level",
            "height",
            "v7Uuid",
            "fullName",
            "annualSalary",
            "currentValue",
            "minutesPercentage",
            "PoolPlayerTmpId",
        ];

        data.iter().map(|x| x.to_string()).collect()
    }

    fn get_query_attributes() -> Vec<String> {
        let data = ["fullName", "agentName", "shortName", "currentClubName"];
        data.iter().map(|x| x.to_string()).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinutesPercentageReductionValue;
pub type MinutesPercentageReduction = DecimalValue<MinutesPercentageReductionValue>;
