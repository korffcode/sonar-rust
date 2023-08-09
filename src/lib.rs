extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SeasonsAverage {
    __typename: String,
    bracket_id: u32,
    game_state: String,
    start_date: String,
    home_team_name: String,
    away_team_name: String,
    home_team_score: u32,
    away_team_score: u32,
    stadium: String
}

pub mod seasons_average_parser {
    pub fn get_typename(json: &str) -> String {
        let seasons_average: super::SeasonsAverage =
            serde_json::from_str(json).unwrap();

        seasons_average.__typename
    }

    pub fn get_game_state(json: &str) -> String {
        let seasons_average: super::SeasonsAverage =
            serde_json::from_str(json).unwrap();

        seasons_average.game_state
    }

    pub fn get_start_date(json: &str) -> String {
        let seasons_average: super::SeasonsAverage =
            serde_json::from_str(json).unwrap();

        seasons_average.start_date
    }

    pub fn get_bracket_id(json: &str) -> u32 {
        let seasons_average: super::SeasonsAverage =
            serde_json::from_str(json).unwrap();

        seasons_average.bracket_id
    }

    pub fn get_home_team_name(json: &str) -> String {
        let seasons_average: super::SeasonsAverage =
            serde_json::from_str(json).unwrap();

        seasons_average.home_team_name
    }

    pub fn get_away_team_name(json: &str) -> String {
        let seasons_average: super::SeasonsAverage =
            serde_json::from_str(json).unwrap();

        seasons_average.away_team_name
    }

    pub fn get_home_team_score(json: &str) -> u32 {
        let seasons_average: super::SeasonsAverage =
            serde_json::from_str(json).unwrap();

        seasons_average.home_team_score
    }

    pub fn get_away_team_score(json: &str) -> u32 {
        let seasons_average: super::SeasonsAverage =
            serde_json::from_str(json).unwrap();

        seasons_average.away_team_score
    }

    pub fn get_stadium(json: &str) -> String {
        let seasons_average: super::SeasonsAverage =
            serde_json::from_str(json).unwrap();

        seasons_average.stadium
    }
}
