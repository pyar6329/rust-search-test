use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    #[serde(rename = "data")]
    pub data: Vec<Player>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "bio")]
    pub bio: String,
    #[serde(rename = "foot")]
    pub foot: String,
    #[serde(
        rename = "clips",
        deserialize_with = "default_if_empty", // default is "" if key doesn't exist
        default // default is "" if key exists but value is null
    )]
    pub clips: Clips,
    #[serde(rename = "gsmId")]
    pub gsm_id: String,
    #[serde(rename = "level")]
    pub level: String,
    #[serde(rename = "gender")]
    pub gender: String,
    #[serde(
        rename = "height",
        deserialize_with = "string_height_to_u64",
        // serialize_with = "u64_as_number"
    )]
    pub height: u64,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "teamId")]
    pub team_id: String,
    #[serde(rename = "v7Uuid")]
    pub v7_uuid: String,
    #[serde(rename = "weight")]
    pub weight: String,
    #[serde(rename = "agentId")]
    pub agent_id: String,
    #[serde(rename = "agencyId")]
    pub agency_id: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "ogolLink")]
    pub ogol_link: String,
    #[serde(rename = "agentName")]
    pub agent_name: String,
    #[serde(rename = "birthDate")]
    pub birth_date: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "vimeoLink")]
    pub vimeo_link: String, // JSON文字列はそのままString型で
    #[serde(rename = "wyscoutId")]
    pub wyscout_id: String,
    #[serde(rename = "agencyName")]
    pub agency_name: String,
    #[serde(rename = "agentEmail")]
    pub agent_email: String,
    #[serde(rename = "agentImage")]
    pub agent_image: String,
    #[serde(rename = "agencyEmail")]
    pub agency_email: String,
    #[serde(rename = "agencyImage")]
    pub agency_image: String,
    #[serde(rename = "playerEmail")]
    pub player_email: String,
    #[serde(rename = "youtubeLink")]
    pub youtube_link: String, // JSON文字列はそのままString型で
    #[serde(rename = "annualSalary", deserialize_with = "string_cost_to_u64")]
    pub annual_salary: u64,
    #[serde(rename = "clubContacts")]
    pub club_contacts: String,
    #[serde(rename = "currentValue", deserialize_with = "string_cost_to_u64")]
    pub current_value: u64,
    #[serde(rename = "imageDataURL")]
    pub image_data_url: String,
    #[serde(rename = "subPositions")]
    pub sub_positions: String,
    #[serde(rename = "mainPositions")]
    pub main_positions: String,
    #[serde(rename = "playingStyles")]
    pub playing_styles: String,
    #[serde(rename = "agentCellphone")]
    pub agent_cellphone: String,
    #[serde(rename = "agencyCellphone")]
    pub agency_cellphone: String,
    #[serde(rename = "currentClubName")]
    pub current_club_name: String,
    #[serde(rename = "isCreatedByClub")]
    pub is_created_by_club: String,
    #[serde(rename = "playerCellphone")]
    pub player_cellphone: String,
    #[serde(rename = "playerInstagram")]
    pub player_instagram: String,
    #[serde(rename = "willContainLabel")]
    pub will_contain_label: String,
    #[serde(
        rename = "minutesPercentage",
        deserialize_with = "default_if_empty", // default is "" if key doesn't exist
        default // default is "" if key exists but value is null
    )]
    pub minutes_percentage: MinutesPercentage,
    #[serde(rename = "transferCondition")]
    pub transfer_condition: String,
    #[serde(rename = "transfermarktData")]
    pub transfermarkt_data: String,
    #[serde(rename = "contractExpiration")]
    pub contract_expiration: String,
    #[serde(rename = "currentCountryCode")]
    pub current_country_code: String,
    #[serde(rename = "playerCurrentState")]
    pub player_current_state: String,
    #[serde(rename = "seeCurrentClubLink")]
    pub see_current_club_link: String,
    #[serde(rename = "transfermarketLink")]
    pub transfermarket_link: String,
    #[serde(rename = "transferFeeInDollar")]
    pub transfer_fee_in_dollar: String,
    #[serde(rename = "lastConditionUpdateAt")]
    pub last_condition_update_at: String,
    #[serde(rename = "nationalityCountryCode")]
    pub nationality_country_code: String,
    #[serde(rename = "playerSecondNationality")]
    pub player_second_nationality: String,
    #[serde(rename = "playerPossiblesNationality")]
    pub player_possibles_nationality: String,
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

impl GetAttributes for Player {
    fn get_filter_attributes() -> Vec<String> {
        let data = [
            "bio",
            "foot",
            "level",
            "gender",
            "height",
            "v7Uuid",
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinutesPercentageValue;
pub type MinutesPercentage = DecimalValue<MinutesPercentageValue>;
