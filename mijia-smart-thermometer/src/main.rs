use std::{thread::spawn, time::{Duration, SystemTime, UNIX_EPOCH}};

use chrono::{DateTime, Datelike, Local, NaiveDateTime, TimeZone, Timelike, Weekday};
use rand::{distributions::{Distribution, Uniform}, thread_rng};
use slint::{SharedString, Timer, TimerMode, Weak};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_render_week_day(|ts| {
        let d = UNIX_EPOCH + Duration::from_secs(ts as u64);
        let naive = DateTime::<Local>::from(d);
        let weekday_str = match naive.weekday() {
            Weekday::Mon => { "周一" },
            Weekday::Tue => { "周二" },
            Weekday::Wed => { "周三" },
            Weekday::Thu => { "周四" },
            Weekday::Fri => { "周五" },
            Weekday::Sat => { "周六" },
            Weekday::Sun => { "周日" },
        };
        weekday_str.into()
    });
    ui.on_render_time(|ts| {
        let d = UNIX_EPOCH + Duration::from_secs(ts as u64);
        let naive = DateTime::<Local>::from(d);
        let hour = naive.hour();
        let min = naive.minute();
        format!("{hour}:{min}").into()
    });
    ui.on_render_date(|ts| {
        let d = UNIX_EPOCH + Duration::from_secs(ts as u64);
        let naive = DateTime::<Local>::from(d);
        let mon = naive.month();
        let day = naive.day();
        format!("{mon}/{day}").into()
    });
    ui.on_render_temperature(|temperature| {
        format!("{temperature:.1}").into()
    });
    ui.on_render_humidity(|humidity| {
        format!("{humidity:.0}").into()
    });
    ui.on_render_comfort(|temperature, humidity| {
        temperature > 18.0 && temperature < 25.0 && humidity > 47.0 && humidity < 70.0
    });

    let timer = Timer::default();
    {
        let ui_weak = ui.as_weak();
        timer.start(TimerMode::Repeated, Duration::from_secs(1), move || {
            let now = SystemTime::now();
            let now = now.duration_since(UNIX_EPOCH).unwrap();
            let ui = ui_weak.unwrap();let timestamp = now.as_secs() as i32;
            let mut thread_rng = thread_rng();
            let temperaturre = Uniform::from(15.0..30.0).sample(&mut thread_rng);
            let humidity = Uniform::from(40.0..75.0).sample(&mut thread_rng);
            // println!("timestamp:{timestamp},temperaturre:{temperaturre},humidity:{humidity}");
            ui.set_timestamp(timestamp);
            ui.set_temperature(temperaturre);
            ui.set_humidity(humidity);
        })
    }

    ui.run()
}
