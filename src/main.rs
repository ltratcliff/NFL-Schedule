// https://gist.github.com/nntrn/ee26cb2a0716de0947a0a4e9a157bc1c
// scoreboard - https://cdn.espn.com/core/nfl/scoreboard?xhr=1&limit=50
// schedule - https://cdn.espn.com/core/nfl/schedule?xhr=1&year=2020&week=2
pub mod structs;

use crate::structs::Root;
use chrono::{TimeZone, Utc};
use chrono_tz::US::Eastern;
use clap::Parser;
use colored::{ColoredString, Colorize};
use reqwest::Client;
use std::env;
use std::fs;
use std::fs::File;
use std::time::{Duration, SystemTime};
use tracing::{debug};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// week number
    week: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let args = Args::parse();
    let tmpdir = env::temp_dir();
    //let cached = Path::new(tmpdir.join("nfl.json")).exists();
    //Forcing API grab
    let cached = false;
    let metadata = fs::metadata(tmpdir.join("nfl.json"));
    let mtime = match metadata {
        Ok(m) => {
            let modified_time = m.modified().unwrap();
            let one_hour = Duration::from_secs(3600);
            let current_time = SystemTime::now();
            let duration = current_time.duration_since(modified_time).unwrap();
            if duration > one_hour {
                debug!("File modified more than 1 hour ago");
                false
            } else {
                debug!("File modified less than 1 hour ago");
                true
            }
        }
        Err(_err) => {
            false
        }
    };
    let response = if cached & mtime {
        debug!("Reading cached JSON file");
        fs::read_to_string(tmpdir.join("nfl.json"))
            .unwrap()
            .parse()
            .unwrap()
    } else {
        debug!("NFL Json does not exist, pulling from API");
        let c = Client::new();
        let api_url = format!(
            "https://cdn.espn.com/core/nfl/schedule?xhr=1&year=2023&week={}",
            args.week
        );
        let response = c.get(api_url).send().await.unwrap().text().await.unwrap();
        debug!("Writing JSON to tmp");
        let deserialized: Root = serde_json::from_str(&response).unwrap();
        serde_json::to_writer(
            &File::create(tmpdir.join("nfl.json")).unwrap(),
            &deserialized,
        )
        .unwrap();
        response
    };

    let deserialized: Root = serde_json::from_str(&response).unwrap();
    // let g = deserialized.content.schedule.n20230911.games;
    let games = deserialized.content.schedule;

    //TODO: sqlite DB?
    for (_k, v) in games.into_iter() {
        for g in v.games.into_iter() {
            // let d = NaiveDateTime::parse_from_str(&g.date, "%Y-%m-%dT%H:%MZ").unwrap();
            let d = Utc.datetime_from_str(&g.date, "%Y-%m-%dT%H:%M%Z").unwrap();
            let mut home_team: ColoredString = "".to_string().white();
            let mut away_team: ColoredString = "".to_string().white();
            let mut home_score: ColoredString = "".to_string().white();
            let mut away_score: ColoredString = "".to_string().white();
            for c in g.competitions {
                for h in c.competitors {
                    if h.home_away == "home" {
                        if let Some(true) = h.winner {
                            home_team = h.team.abbreviation.green();
                            home_score = h.score.green()
                        } else {
                            home_team = h.team.abbreviation.white();
                            home_score = h.score.white()
                        }
                    } else if h.home_away == "away" {
                        if let Some(true) = h.winner {
                            away_team = h.team.abbreviation.green();
                            away_score = h.score.green()
                        } else {
                            away_team = h.team.abbreviation.white();
                            away_score = h.score.white();
                        }
                    }
                }
            }
            println!(
                "{} @ {} on {} :\t {} - {}",
                away_team,
                home_team,
                d.with_timezone(&Eastern),
                away_score,
                home_score
            )
        }
    }

    Ok(())
}
