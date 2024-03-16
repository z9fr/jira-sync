use chrono::{DateTime, TimeDelta, Timelike};

fn main() {
    let completed_task_time_str = "2024-03-13T16:31:37.379+05:30";
    let completed_task_time =
        DateTime::parse_from_str(&completed_task_time_str, "%Y-%m-%dT%H:%M:%S%.3f%z").unwrap();
    let time_for_complete = 61200;
    //    57600; // 16 hours

    let duration_to_complete = TimeDelta::try_seconds(time_for_complete).unwrap();
    let working_hours = TimeDelta::try_hours(8).unwrap();

    let assumed_start_time = completed_task_time - duration_to_complete;
    let slots_count = (duration_to_complete.num_seconds() as f64
        / working_hours.num_seconds() as f64)
        .ceil() as i32;

    println!("{}", slots_count);

    let task_duration_seconds = duration_to_complete.num_seconds();
    let working_hours_seconds = working_hours.num_seconds();

    let mut slots = Vec::new();
    for i in 0..slots_count {
        let mut slot_start_time = assumed_start_time + working_hours * i as i32;
        let working_duration =
            TimeDelta::try_seconds(task_duration_seconds - i as i64 * working_hours_seconds as i64)
                .unwrap();

        let mut slot_end_time = slot_start_time + std::cmp::min(working_hours, working_duration);

        if slot_start_time.time().hour() < 9 {
            let adjustment =
                TimeDelta::try_hours((9 - slot_start_time.time().hour()).into()).unwrap();
            slot_start_time = slot_start_time + adjustment;
            slot_end_time = slot_start_time + std::cmp::min(working_hours, working_duration);
        }

        if slot_end_time.time().hour() > 17 {
            let adjustment =
                TimeDelta::try_hours((slot_end_time.time().hour() - 17).into()).unwrap();
            slot_end_time = slot_end_time - adjustment;
            slot_start_time = slot_end_time - std::cmp::min(working_hours, working_duration);
        }

        slots.push((slot_start_time.to_string(), slot_end_time.to_string()));
    }

    println!("{:?}", slots);

    // usually work for 8 hours per day starting from 9-5
    // this task took 57600 seconds (16 hours ) to complete
    // was able to complete it on `Wed Mar 13 2024 16:31:37`
    // find hours of slots worked on this day
}
