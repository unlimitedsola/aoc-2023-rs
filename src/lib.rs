use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use reqwest::header::{COOKIE, USER_AGENT};
use time::Month::December;
use time::{Date, OffsetDateTime, PrimitiveDateTime, UtcOffset};

const YEAR: i32 = 2023;

pub fn aoc() -> Result<AoC> {
    let token = include_str!("../.token").trim().to_owned();
    AoC::new(YEAR, token)
}

pub struct AoC {
    year: i32,
    token: String,
    inputs: PathBuf,
    http: Client,
}

impl AoC {
    pub fn new(year: i32, token: String) -> Result<Self> {
        let inputs = PathBuf::from(format!("./inputs/{year}"));
        create_dir_all(&inputs)?;
        let http = Client::new();
        Ok(Self {
            year,
            token,
            inputs,
            http,
        })
    }

    pub fn read_input(&self, day: u8) -> Result<String> {
        let path = self.inputs.join(format!("{day}.txt"));
        let input = if !path.exists() {
            let input = self.fetch_input(day)?;
            let mut file = File::create(path)?;
            file.write_all(input.as_bytes())?;
            input
        } else {
            read_to_string(path)?
        };
        Ok(input)
    }

    fn fetch_input(&self, day: u8) -> Result<String> {
        if let 1..=25 = day {
        } else {
            bail!("day must be in range 1..=25")
        }

        let starts = PrimitiveDateTime::new(
            Date::from_calendar_date(self.year, December, day)?,
            time::Time::from_hms(0, 0, 0)?,
        )
        .assume_offset(UtcOffset::from_hms(-5, 0, 0)?);

        let now = OffsetDateTime::now_utc();

        if starts > now {
            bail!(
                "day {} is not started yet, remaining: {}",
                day,
                starts - now
            )
        }

        self.http
            .get(format!(
                "https://adventofcode.com/{year}/day/{day}/input",
                year = self.year
            ))
            .header(COOKIE, format!("session={token}", token = self.token))
            .header(
                USER_AGENT,
                "Rustacean: @unlimitedsola (dev at sola dot love)",
            )
            .send()?
            .text()
            .context("failed to fetch input")
    }
}
