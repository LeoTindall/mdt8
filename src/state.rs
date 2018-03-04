use chrono::{Datelike, DateTime, Duration, Local};
use serde_json;

use std::io;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct TrackedDay {
    year: i32,
    ordinal: u32,
    completed_seconds: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct State {
    goal_minutes: u64,
    tracking_date: DateTime<Local>,
    completed_today_seconds: u64,
    current_session_start: Option<DateTime<Local>>,
    prior_days: Vec<TrackedDay>
}

impl Default for State {
    fn default() -> Self {
        State {
            tracking_date: Local::now(),
            goal_minutes: 30,
            completed_today_seconds: 0,
            current_session_start: None,
            prior_days: Vec::new(),
        }
    }
}

impl State {
    /// Returns true if a session is in progress and false otherwise
    pub fn in_session(&self) -> bool {
        match self.current_session_start {
            Some(_) => true,
            None => false
        }
    }

    /// Starts a new session. Returns an error if a session is already in progress.
    pub fn start_session(&mut self) -> Result<(), String> {
        if self.in_session() {
            return Err("There is already a session in progress.".into())
        }

        self.current_session_start = Some(Local::now());
        Ok(())
    }

    /// Stops and commits time from a session. Returns an error if a session is not in progress.
    pub fn stop_session(&mut self) -> Result<(), String> {
        if !self.in_session() {
            return Err("There is no session in progress.".into())
        }

        self.completed_today_seconds += Local::now()
            .signed_duration_since(self.current_session_start.unwrap())
            .num_seconds() as u64;
        self.current_session_start = None;
        Ok(())
    }

    /// Stops and removes time from a session. Returns an error if a session is not in progress.
    pub fn cancel_session(&mut self) -> Result<(), String> {
        if !self.in_session() {
            return Err("There is no session in progress.".into())
        }

        self.current_session_start = None;
        Ok(())
    }

    /// Returns true if today is the date currently being tracked, and false otherwise.
    pub fn is_today_tracked(&self) -> bool {
        let now = Local::now();
        let then = self.tracking_date;

        now.year() == then.year() && now.ordinal() == then.ordinal()
    }

    /// Set the tracked day to today. Commits the previous day to the list if needed.
    /// Returns true if it had to change the day being tracked, and false otherwise.
    pub fn set_today_tracked(&mut self) -> bool {
        if self.is_today_tracked() {
            return false;
        }

        if self.in_session() {
            self.stop_session().unwrap();
        }

        self.prior_days.push(TrackedDay {
            completed_seconds: self.completed_today_seconds,
            year: self.tracking_date.year(),
            ordinal: self.tracking_date.ordinal(),
        });

        self.tracking_date = Local::now();
        self.completed_today_seconds = 0;
        self.current_session_start = None;

        return true;
    }

    /// Changes the recorded duration by the given value.
    pub fn modify(&mut self, value: i32) {
        let value = value * 60;
        if value >= 0 {
            self.completed_today_seconds += value as u64
        } else if ((-value) as u64) < self.completed_today_seconds {
            self.completed_today_seconds -= (-value) as u64
        } else {
            self.completed_today_seconds = 0;
        }
    }

    /// Returns the configured goal as a `chrono::Duration`.
    pub fn goal_duration(&self) -> Duration {
        Duration::minutes(self.goal_minutes as i64)
    }

    /// Returns a formatted string representing the configured goal.
    pub fn goal_string(&self) -> String {
        let goal = self.goal_duration();
        let s = if goal.num_hours() < 1 {
            format!("{} minutes", goal.num_minutes())
        } else {
            format!("{} hours and {} minutes", goal.num_hours(), goal.num_minutes() - (goal.num_hours() * 60))
        };
        s
    }

    /// Returns the recorded time today as a `chrono::Duration`.
    pub fn completed_today_duration(&self) -> Duration {
        Duration::seconds(self.completed_today_seconds as i64)
    }

    /// Returns a formatted string representing the recorded time today.
    pub fn completed_today_string(&self) -> String {
        let today = self.completed_today_duration();
        let s = if today.num_hours() < 1 {
            format!("{} minutes", today.num_minutes())
        } else {
            format!("{} hours and {} minutes", today.num_hours(), today.num_minutes() - (today.num_hours() * 60))
        };
        s
    }

    /// Read the state from a file.
    pub fn load_from(path: &Path) -> io::Result<Self> {
        let f = File::open(path)?;
        Ok(serde_json::from_reader(f)?)
    }

    /// Write the state to a file.
    pub fn write_to(&self, path: &Path) {
        let f = File::create(path).expect("Could not open config file");
        serde_json::to_writer_pretty(f, self).expect("Could not write to config file");
    }
}
