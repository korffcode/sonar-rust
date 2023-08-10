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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[cfg(test)]
mod seasons_average_test {
    use super::seasons_average_parser::*;

	const TEST_JSON: &str = r#"{
                "__typename" : "ChampionshipGame",
                "bracket_id" : 201,
                "game_state" : "P",
                "start_date" : "",
                "home_team_name" : "Reds",
                "away_team_name" : "Cubs",
                "home_team_score" : 35,
                "away_team_score" : 0,
				"stadium" : "Riverfront"
            }"#;

    #[test]
    fn test_get_bracket_id() {
        let expected = 201;

        assert_eq!(expected, get_bracket_id(TEST_JSON));
    }

    #[test]
    fn test_get_game_state() {
        let expected = "P";

        assert_eq!(expected, get_game_state(TEST_JSON));
    }

    #[test]
    fn test_get_start_date() {
        let expected = "";

        assert_eq!(expected, get_start_date(TEST_JSON));
    }

    #[test]
    fn test_get_home_team_name() {
        let expected = "Reds";

        assert_eq!(expected, get_home_team_name(TEST_JSON));
    }

    #[test]
    fn test_get_away_team_name() {
        let expected = "Cubs";

        assert_eq!(expected, get_away_team_name(TEST_JSON));
    }

    #[test]
    fn test_get_home_team_score() {
        let expected = 35;

        assert_eq!(expected, get_home_team_score(TEST_JSON));
    }

    #[test]
    fn test_get_away_team_score() {
        let expected = 0;

        assert_eq!(expected, get_away_team_score(TEST_JSON));
    }

    #[test]
    fn test_get_typename() {
        let expected = "ChampionshipGame";

        assert_eq!(expected, get_typename(TEST_JSON));
    }

    #[test]
    fn test_get_stadium() {
        let expected = "Riverfront";

        assert_eq!(expected, get_stadium(TEST_JSON));
    }
}
