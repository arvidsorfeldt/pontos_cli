use chrono::{Duration, NaiveDate};

#[test]
fn datetime_test() {
    let picked_date = NaiveDate::parse_from_str("2023-11-07", "%Y-%m-%d").unwrap();
    assert_eq!(
        picked_date + Duration::days(1),
        NaiveDate::from_ymd_opt(2023, 11, 08).unwrap()
    );
}
