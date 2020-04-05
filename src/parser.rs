use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use nom::branch::alt;
use nom::character::complete::{anychar, one_of};
use nom::combinator::map;
use nom::multi::{many0_count, many1};
use nom::IResult;

static SEP: &str = "T-_:., \t";

static DIGITS: &str = "0123456789";

fn parse_year(s: &str) -> IResult<&str, u32> {
    let (s, d1) = one_of("2")(s)?;
    let (s, d2) = one_of(&DIGITS[0..2])(s)?;
    let (s, d3) = one_of(DIGITS)(s)?;
    let (s, d4) = one_of(DIGITS)(s)?;

    Ok((
        s,
        d1.to_digit(10).unwrap() * 1000
            + d2.to_digit(10).unwrap() * 100
            + d3.to_digit(10).unwrap() * 10
            + d4.to_digit(10).unwrap(),
    ))
}

fn parse_hour(s: &str) -> IResult<&str, u32> {
    let (s, d1) = one_of(&DIGITS[..3])(s)?;
    let d1 = d1.to_digit(10).unwrap();

    let (s, d2) = match d1 {
        0 | 1 => one_of(DIGITS)(s)?,
        2 => one_of(&DIGITS[..4])(s)?,
        _ => unreachable!("one_of(\"012\") gave {}", d1),
    };
    let d2 = d2.to_digit(10).unwrap();

    Ok((s, d1 * 10 + d2))
}

fn parse_minute_second(s: &str) -> IResult<&str, u32> {
    let (s, d1) = one_of(&DIGITS[..6])(s)?;
    let d1 = d1.to_digit(10).unwrap();
    let (s, d2) = one_of(DIGITS)(s)?;
    let d2 = d2.to_digit(10).unwrap();

    Ok((s, d1 * 10 + d2))
}

fn parse_month(s: &str) -> IResult<&str, u32> {
    let (s, d1) = one_of("01")(s)?;
    let d1 = d1.to_digit(10).unwrap();

    let (s, d2) = match d1 {
        0 => one_of(DIGITS)(s)?,
        1 => one_of(&DIGITS[..3])(s)?,
        _ => unreachable!("one_of(\"01\") gave {}", d1),
    };
    let d2 = d2.to_digit(10).unwrap();

    Ok((s, d1 * 10 + d2))
}

fn parse_date(s: &str) -> IResult<&str, u32> {
    let (s, d1) = one_of("0123")(s)?;
    let d1 = d1.to_digit(10).unwrap();

    let (s, d2) = match d1 {
        0 | 1 | 2 => one_of(&DIGITS[1..])(s)?,
        3 => one_of("01")(s)?,
        _ => unreachable!("one_of(\"0123\") gave {}", d1),
    };
    let d2 = d2.to_digit(10).unwrap();

    Ok((s, d1 * 10 + d2))
}

fn parse_calendar_date(i: &str) -> IResult<&str, NaiveDate> {
    let (i, _) = many0_count(one_of(SEP))(i)?;
    let (i, year) = parse_year(i)?;
    let (i, _) = many0_count(one_of(SEP))(i)?;
    let (i, month) = parse_month(i)?;
    let (i, _) = many0_count(one_of(SEP))(i)?;
    let (i, date) = parse_date(i)?;

    Ok((i, NaiveDate::from_ymd(year as i32, month, date)))
}

fn parse_time(i: &str) -> IResult<&str, NaiveTime> {
    let (i, _) = many0_count(one_of(SEP))(i)?;
    let (i, hour) = parse_hour(i)?;
    let (i, _) = many0_count(one_of(SEP))(i)?;
    let (i, minute) = parse_minute_second(i)?;
    let (i, _) = many0_count(one_of(SEP))(i)?;
    let (i, second) = parse_minute_second(i)?;

    Ok((i, NaiveTime::from_hms(hour, minute, second)))
}

enum DateTimePart {
    Date(NaiveDate),
    Time(NaiveTime),
    Sep,
}

fn parse_file_internal(name: &str) -> IResult<&str, NaiveDateTime> {
    let i = name;

    let parse_calendar_date = map(parse_calendar_date, DateTimePart::Date);
    let parse_time = map(parse_time, DateTimePart::Time);
    let parse_any = map(anychar, |_| DateTimePart::Sep);

    let parse_part = alt((parse_calendar_date, parse_time, parse_any));
    let (i, parts) = many1(parse_part)(i)?;

    let mut date = None;
    let mut time = None;

    for part in parts {
        match part {
            DateTimePart::Date(item) => date = Some(item),
            DateTimePart::Time(item) => time = Some(item),
            DateTimePart::Sep => {}
        }
    }

    let date = date.unwrap_or_else(|| NaiveDate::from_ymd(2000, 1, 1));
    let time = time.unwrap_or_else(|| NaiveTime::from_hms(0, 0, 0));
    Ok((i, NaiveDateTime::new(date, time)))
}

pub fn parse_file_name(name: &str) -> Result<NaiveDateTime, String> {
    match parse_file_internal(name) {
        Ok((_, date)) => Ok(date),
        Err(err) => Err(format!("parse error {:?}", err)),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_file_name, parse_hour, parse_year};

    #[test]
    fn parse_year_good() {
        let year = parse_year("2019").expect("match 2019").1;
        assert_eq!(year, 2019);
        let year = parse_year("2020").expect("match 2020").1;
        assert_eq!(year, 2020);
        let year = parse_year("2021").expect("match 2021").1;
        assert_eq!(year, 2021);
    }

    #[test]
    fn parse_year_bad() {
        let _year = parse_year("1990").expect_err("no-match 1990");
        let _year = parse_year("2500").expect_err("no-match 2500");
    }

    #[test]
    fn parse_hour_good() {
        let hour = parse_hour("00").expect("match 00").1;
        assert_eq!(hour, 0);
        let hour = parse_hour("01").expect("match 01").1;
        assert_eq!(hour, 1);
        let hour = parse_hour("09").expect("match 09").1;
        assert_eq!(hour, 9);
        let hour = parse_hour("10").expect("match 10").1;
        assert_eq!(hour, 10);
        let hour = parse_hour("11").expect("match 11").1;
        assert_eq!(hour, 11);
        let hour = parse_hour("19").expect("match 19").1;
        assert_eq!(hour, 19);
        let hour = parse_hour("20").expect("match 20").1;
        assert_eq!(hour, 20);
        let hour = parse_hour("23").expect("match 23").1;
        assert_eq!(hour, 23);
    }

    #[test]
    fn parse_hour_bad() {
        parse_hour("24").expect_err("no-match 24");
        parse_hour("30").expect_err("no-match 30");
    }

    macro_rules! full_test {
        ($name:ident, $str:expr, $expected:expr) => {
            #[test]
            fn $name() {
                use chrono::{NaiveDate, NaiveTime};
                let expected = $expected;
                let datetime = parse_file_name($str).expect($str);
                assert_eq!(
                    datetime.date(),
                    NaiveDate::from_ymd(expected.0, expected.1, expected.2)
                );
                assert_eq!(
                    datetime.time(),
                    NaiveTime::from_hms(expected.3, expected.4, expected.5)
                );
            }
        };
    }

    full_test!(full_date, "2020-02-03", (2020, 2, 3, 0, 0, 0));
    full_test!(full_time, "03:04:05", (2000, 1, 1, 3, 4, 5));
    full_test!(full_date_time, "2020-02-03 03:04:05", (2020, 2, 3, 3, 4, 5));
    full_test!(date_with_junk, "qwe2020-02-03rty", (2020, 2, 3, 0, 0, 0));
    full_test!(time_with_junk, "qwe03:04:05rty", (2000, 1, 1, 3, 4, 5));
    full_test!(
        date_time_junk_around,
        "qwe2020-02-03 03:04:05rty",
        (2020, 2, 3, 3, 4, 5)
    );
    full_test!(
        date_time_junk_between,
        "qwe2020-02-03asd03:04:05rty",
        (2020, 2, 3, 3, 4, 5)
    );
    full_test!(
        full_datetime_max,
        "2199-12-31-23-59-59",
        (2199, 12, 31, 23, 59, 59)
    );
}
