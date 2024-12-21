use chrono::prelude::*;
use std::env;

const EPOCH_SEC_LEN: i32 = 10;
const BASE: f32 = 10.0;


fn parse_timestamp(epoch: i64) -> DateTime<Utc> {
    let length = epoch.to_string().chars().count();
    let length = i32::try_from(length).unwrap();
    let exp: i32 = EPOCH_SEC_LEN - length;
    let seconds = epoch * i64::try_from(BASE.powi(exp)).unwrap();
    
    println!("{} -> {} ({}) => {} <> {}", epoch, seconds, exp, EPOCH_SEC_LEN, length);
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(seconds, 0), Utc)
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let epoch: i64 = args[1].parse().unwrap();

    let date = parse_timestamp(epoch);
    println!("{}", date.format("%Y-%m-%d %H:%M:%S.%f"));

}

// TODO
// 1, make into function
// 2. divide the number to parse
// 3. create test cases
