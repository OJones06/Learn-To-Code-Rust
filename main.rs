#![allow(unused_variables)]
#![allow(unused_assignments)] // for the mut
type Meters = i32;
const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    let favourite_season: &str = "summer";
    let mut points_scored: i32 = 28;
    points_scored = 35;
    let event_time: &str = "6:00";
    let event_time: i32 = 6;
    println!(
        "Touchdown pts: {}, Season {}, Points {}, time {}",
        TOUCHDOWN_POINTS, favourite_season, points_scored, event_time
    );
    let favourite_beverage: &str = "Orance crush";

    let _mile_race_length: Meters = 1600;
    let _two_mile_race_length: Meters = 3200;
}
