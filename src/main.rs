// extern crate jsonlib;

use jsonlib::seasons_average_parser;

fn main() {

	let test_json = r#"{
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

    let typename = seasons_average_parser::get_typename(test_json);

    println!("The type is {}", typename);
}
