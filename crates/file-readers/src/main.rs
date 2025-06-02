use crate::json::{
    MatchResult, Navigator,
};
use std::io::Write;
use std::path::Path;
use std::{
    fs::File, io::Read,
};

mod json;
mod xml;

pub fn main() {
    let path = Path::new(
        env!(
            "CARGO_MANIFEST_DIR"
        ),
    );
    let path = path
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let path = path.join("resources\\animations.json");

    let file =
        File::open(&path);
    match file {
        Ok(mut file) => {
            let mut data: String = String::new();
            if let Err(e) = file.read_to_string(&mut data) {
                println!("Read to string error:  {}", e)
            };
            let nav = Navigator::new(&data);
            let mut
            path_to_write =
                path.clone();
            path_to_write.set_extension("");
            // Make the directory
            // Check for it first
            if !path_to_write
                .exists()
            {
                std::fs::create_dir(&path_to_write).unwrap();
            }
            match nav.find_by_key_value_adv("name", Some("Wild Strikes"), false, true) {
                MatchResult::Single(obj) | MatchResult::SingleExact(obj) => {
                    let mut events_nav = Navigator::new(&obj.to_string());
                    if let Some(events) = obj.get("events") {
                        if events.is_array() {
                            events_nav = Navigator::new(&events.to_string());
                        }
                    }
                    let second_key = "events";
                    match events_nav.find_by_key_value_adv(second_key, None, true, false) {
                        MatchResult::Keys(_keys) => println!("Multiple matches"),
                        MatchResult::All(all) => {
                            println!("All matches");
                            let file_to_write = path_to_write
                                .clone()
                                .join(format!("wild-strikes.{second_key}.json"));
                            println!("Writing to file: {}", file_to_write.to_string_lossy());
                            let mut file = File::create(&file_to_write).unwrap();
                            file.write_all(serde_json::to_string_pretty(&all).unwrap().as_bytes())
                                .unwrap();
                        }
                        MatchResult::None => {
                            println!("No match for {second_key} on name 'Wild Strikes'");
                            // Serialize the current object to a file
                            let file_to_write = path_to_write.clone().join("wild-strikes.json");

                            let mut file = File::create(&file_to_write).unwrap();
                            file.write_all(serde_json::to_string_pretty(obj).unwrap().as_bytes())
                                .unwrap();
                        }
                        MatchResult::Single(single) | MatchResult::SingleExact(single) => {
                            println!("Single match");
                            let file_to_write = path_to_write
                                .clone()
                                .join(format!("wild-strikes.{second_key}.json"));
                            let mut file = File::create(&file_to_write).unwrap();
                            file.write_all(
                                serde_json::to_string_pretty(single).unwrap().as_bytes(),
                            )
                            .unwrap();
                            let third_key = "activeFrames";
                            let activeframes = single.get(third_key);
                            let activeframes_nav =
                                Navigator::new(&activeframes.unwrap().to_string());
                            let result = activeframes_nav
                                .find_by_key_and_matching_sibling_key_value_pair(
                                    "range", "type", "Attack", true, true,
                                );
                            match result {
                                MatchResult::Single(single) | MatchResult::SingleExact(single) => {
                                    println!("Single match");
                                    let file_to_write = path_to_write.clone().join(format!(
                                        "wild-strikes.{second_key}.{third_key}.json"
                                    ));
                                    let mut file = File::create(&file_to_write).unwrap();
                                    file.write_all(
                                        serde_json::to_string_pretty(single).unwrap().as_bytes(),
                                    )
                                    .unwrap();
                                }
                                MatchResult::Keys(_keys) => println!("Multiple matches"),
                                MatchResult::All(all) => {
                                    println!("All matches");
                                    let file_to_write = path_to_write.clone().join(format!(
                                        "wild-strikes.{second_key}.{third_key}.json"
                                    ));
                                    let mut file = File::create(&file_to_write).unwrap();
                                    file.write_all(
                                        serde_json::to_string_pretty(&all).unwrap().as_bytes(),
                                    )
                                    .unwrap();
                                }
                                MatchResult::None => println!("No match for activeframes"),
                            }
                        }
                    }
                }
                // TODO: Handle the other match results
                MatchResult::Keys(_keys) => println!("Multiple matches"),
                MatchResult::All(_all) => println!("All matches"),
                MatchResult::None => println!("No match for name"),
            }
        }
        Err(_) => {
            panic!(
                "Could not read path {}",
                path.display(
                )
            )
        }
    }
}
