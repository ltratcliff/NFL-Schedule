// https://gist.github.com/nntrn/ee26cb2a0716de0947a0a4e9a157bc1c
// scoreboard - https://cdn.espn.com/core/nfl/scoreboard?xhr=1&limit=50
// schedule - https://cdn.espn.com/core/nfl/schedule?xhr=1&year=2020&week=2
pub mod structs;

use crate::structs::Root;
use chrono::{TimeZone, DateTime, Local, NaiveDateTime};
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

struct Game {
    home_team: ColoredString,
    away_team: ColoredString,
    home_score: ColoredString,
    away_score: ColoredString,
    date: DateTime<Local>,
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
        let local = Local::now();
        let api_url = format!(
            "https://cdn.espn.com/core/nfl/schedule?xhr=1&year={}&week={}",
            local.format("%Y"), args.week
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
    let mut sorted_games: Vec<Game> = Vec::new();

    //TODO: sqlite DB?
    for (_k, v) in games.into_iter() {
        for g in v.games.into_iter() {
            let naive = NaiveDateTime::parse_from_str(&g.date, "%Y-%m-%dT%H:%MZ").unwrap();
            let d: DateTime<Local> = Local.from_utc_datetime(&naive);
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
            sorted_games.push(Game {
                home_team,
                away_team,
                home_score,
                away_score,
                date: d,
            });
        }
    }
    sorted_games.sort_by(|a, b| a.date.cmp(&b.date));
    for g in sorted_games {
        println!(
            "{:<3} @ {:<3} on {} :\t {} - {}",
            g.away_team,
            g.home_team,
            g.date.format("%m-%d %I:%M %p"),
            g.away_score,
            g.home_score
        )
    }
    Ok(())
}
