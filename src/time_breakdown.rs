use anyhow::{anyhow, Result};
use chrono::{DateTime, Datelike, FixedOffset, TimeDelta, Timelike, Weekday};

pub struct TimeBreakdown {}

impl TimeBreakdown {
    pub fn slots(
        created: String,
        time_for_complete: i64,
        ignore_weekends: bool,
    ) -> Result<Vec<(String, String, String)>> {
        let completed_task_time =
            match DateTime::parse_from_str(&created, "%Y-%m-%dT%H:%M:%S%.3f%z") {
                Ok(time) => time,
                Err(err) => return Err(anyhow!("Failed to parse creation time: {}", err)),
            };
        let duration_to_complete = TimeDelta::try_seconds(time_for_complete).unwrap();
        let working_hours = TimeDelta::try_hours(8).unwrap();

        let assumed_start_time = completed_task_time - duration_to_complete;
        let slots_count = (duration_to_complete.num_seconds() as f64
            / working_hours.num_seconds() as f64)
            .ceil() as i32;
        let mut slots = Vec::new();

        let task_duration_seconds = duration_to_complete.num_seconds();
        let working_hours_seconds = working_hours.num_seconds();

        for i in 0..slots_count {
            let mut slot_start_time = assumed_start_time + working_hours * i as i32;

            if ignore_weekends {
                while slot_start_time.weekday() == Weekday::Sat
                    || slot_start_time.weekday() == Weekday::Sun
                {
                    slot_start_time = slot_start_time + TimeDelta::try_days(1).unwrap()
                }
            }

            let working_duration = TimeDelta::try_seconds(
                task_duration_seconds - i as i64 * working_hours_seconds as i64,
            )
            .unwrap();

            let mut slot_end_time =
                slot_start_time + std::cmp::min(working_hours, working_duration);

            // Check if slot starts before 9 AM or if it crosses into the next day
            if slot_start_time.time().hour() < 9
                || (slot_start_time.time().hour() == 9 && slot_start_time.time().minute() != 0)
            {
                let adjustment =
                    TimeDelta::try_hours((9 - slot_start_time.time().hour()).into()).unwrap();
                slot_start_time = slot_start_time + adjustment;
                slot_end_time = slot_start_time + std::cmp::min(working_hours, working_duration);
            }

            // Check if slot ends after 5 PM or if it crosses into the next day
            if slot_end_time.time().hour() > 17
                || (slot_end_time.time().hour() == 17 && slot_end_time.time().minute() != 0)
            {
                let adjustment =
                    TimeDelta::try_hours((slot_end_time.time().hour() - 17).into()).unwrap();
                slot_end_time = slot_end_time - adjustment;
                slot_start_time = slot_end_time - std::cmp::min(working_hours, working_duration);
            }

            let offset = FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap();

            let start_time_str = slot_start_time
                .to_utc()
                .with_timezone(&offset)
                .format("%Y-%m-%dT%H:%M:%S%.3f%:z")
                .to_string();

            let end_time_str = slot_end_time
                .to_utc()
                .with_timezone(&offset)
                .format("%Y-%m-%dT%H:%M:%S%.3f%:z")
                .to_string();

            let start_time_jira = slot_start_time
                .to_utc()
                .with_timezone(&offset)
                .format("%Y-%m-%dT%H:%M:%S%.3f%z")
                .to_string();

            slots.push((start_time_str, end_time_str, start_time_jira))
        }

        Ok(slots)
    }
}
