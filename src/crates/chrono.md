# ⏰ [chrono](https://crates.io/crates/chrono) & [chrono-tz](https://crates.io/crates/chrono-tz)
## 📗 **`NaiveDate` Struct**
In Chrono, `NaiveDate` models a calendar day without any associated time zone or time day information.
```rust
# use chrono::NaiveDate;
# 
# fn main() {
let birthday = NaiveDate::from_ymd_opt(1999, 1, 19).unwrap();
println!("Birthday: {}", birthday);
# }
```

```rust
# use chrono::NaiveDate;
# 
# fn main() {
let birthday = "1999-01-19";

let naive_dt = NaiveDate::parse_from_str(birthday, "%Y-%m-%d")
    .expect("Error parsing");

println!("Birthday: {naive_dt}");
# }
```

```rust
# use chrono::NaiveDate;
# 
# fn main() {
let birthday = "1999-01-19";

let naive_dt = birthday.parse::<NaiveDate>()
    .expect("Error parsing");

println!("Birthday: {naive_dt}");
# }
```

## 📗 **`NaiveTime` Struct**
In Chrono, `NaiveTime` models time without any associated time zone or time day information.
```rust
# use chrono::NaiveTime;
# 
# fn main() {
let time = NaiveTime::from_hms_opt(15, 45, 10).unwrap();
println!("Time: {}", time);
# }
```



## 📗 **`NaiveDateTime` Struct**
In Chrono, `NaiveDateTime` represents a date and time without an associated time zone.

```rust
# use chrono::NaiveDateTime;
# 
# fn main() {
let input: &str = "2023-01-20 12:34:56";
let format: &str = "%Y-%m-%d %H:%M:%S";

let naive_dt: NaiveDateTime = NaiveDateTime::parse_from_str(input, format)
    .expect("Error parsing");

println!("NaiveDateTime: {}", naive_dt);
# }
```

```rust
# use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
# 
# fn main() {
let day = NaiveDate::from_ymd_opt(1969, 7, 20).unwrap();
let time = NaiveTime::from_hms_opt(20, 17, 0).unwrap();

let moon_landing = NaiveDateTime::new(day, time);

println!("Moon landing: {}", moon_landing);
# }
```

## 📗 **`DateTime<T>` Struct**
In Chrono, `DateTime<T>` represents a date and time with an associated time zone. The `T` indicates the time zone type, such as `Utc` or `Local`.

**UTC**
```rust
# use chrono::prelude::*;
# 
# fn main() {
let now: DateTime<Utc> = Utc::now();
let format: String = now.format("%Y-%m-%d %H:%M:%S").to_string();
println!("Current UTC time: {}", now);
println!("Current UTC format time: {}", format);
# }
```
**Local**
```rust
# use chrono::prelude::*;
# 
# fn main() {
let now: DateTime<Local> = Local::now();
println!("Current local time: {}", now);
println!("Naive Date now: {}", now.date_naive());
println!("Time now: {}", now.time());
println!("Year now: {}", now.year());
println!("Month now: {}", now.month());
println!("Day now: {}", now.day());
println!("Hour now: {}", now.hour());
println!("Minute now: {}", now.minute());
println!("Second now: {}", now.second());
println!("Offset now: {}", now.offset());
# }
```

## 📗 **`TimeDelta` Struct**
In Chrono, `TimeDelta` represents a span of time. `chrono::Duration` is an alias for `chrono::TimeDelta`, so you can use either name.
```rust
# use chrono::prelude::*;
# use chrono::{Duration, TimeDelta};
# 
# fn main() {
let five_seconds = TimeDelta::seconds(5);
let negative_five_seconds = TimeDelta::seconds(-5);
let five_minutes = TimeDelta::minutes(5);
let five_hours = TimeDelta::hours(5);
let five_days = TimeDelta::days(5);
let five_weeks = TimeDelta::weeks(5);

println!("Days in five weeks: {}", five_weeks.num_days());
println!("Hours in five weeks: {}", five_weeks.num_hours());
println!("Minutes in five weeks: {}", five_weeks.num_minutes());

let total_duration = five_weeks + five_days + five_hours + five_minutes;
println!("Total duration: {total_duration}");

println!(
    "{} weeks, {} days, {} hours, {} minutes",
    total_duration.num_weeks(),
    total_duration.num_days(),
    total_duration.num_hours(),
    total_duration.num_minutes()
);

let now: DateTime<Utc> = Utc::now();
println!("Current UTC time: {}", now);

let now_after_week = now + Duration::days(7);
println!("Current UTC + week: {}", now_after_week);

let now_before_2hours = now - Duration::hours(2);
println!("Current UTC - 2 hours: {}", now_before_2hours);
# }
```

## 📗 **Converting Timezones**
We use `chrono-tz` crate to convert between tiemzones 

```rust,ignore
# use chrono::prelude::*;
# use chrono_tz::America::Los_Angeles;
# 
# fn main() {
let local_time: DateTime<Local> = Local::now();
let utc_time: DateTime<Utc> = local_time.with_timezone(&Utc);
let los_angeles_time = local_time.with_timezone(&Los_Angeles);

println!("Current Local time: {}", local_time);
println!("Current UTC: {}", utc_time);
println!("Current Los Angeles time: {}", los_angeles_time);
# }
```
Expected output:
```bash
Current Local time: 2026-10-21 13:38:48.232049415 +02:00
Current UTC: 2026-10-21 11:38:48.232049415 UTC
Current Los Angeles time: 2026-10-21 04:38:48.232049415 PDT
```

## 📗 **`parse_from_str` Function**
`DateTime::parse_from_str` parses a string into a `DateTime<FixedOffset>`.

```rust
# use chrono::prelude::*;
# 
# fn main() {
let string_datetime = "31-Oct-1995 18:07:54 -0600";

let dt = DateTime::parse_from_str(string_datetime, "%d-%b-%Y %H:%M:%S %z").unwrap();

println!("Date Time: {}", dt);
# }
```

## 📗 **`format` Method**
`format` returns a value that displays a date or time using the format string you provide.
```rust
# use chrono::prelude::*;
# 
# fn main() {
let utc_now = Utc::now();

println!("Date: {}", utc_now.format("%m-%d-%Y"));
println!("Date and time: {}", utc_now.format("%A %I %p %Z"));
# }
```
