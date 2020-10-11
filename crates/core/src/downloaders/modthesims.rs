use chrono::{Duration, NaiveDateTime, NaiveTime, Utc};
use regex::Regex;

use super::{DateInfoSelectors, Downloader, ModInfoSelectors, SameDateInfoSelectors};

#[derive(Default)]
pub struct ModTheSimsDownloader;

impl Downloader for ModTheSimsDownloader {
    fn name(&self) -> String {
        "ModTheSims".into()
    }

    fn hostname_matchers(&self) -> Vec<String> {
        vec!["modthesims.info".into()]
    }

    fn get_info_selectors(&self) -> ModInfoSelectors {
        ModInfoSelectors {
            name: "div.well.profilepic.well-small.well-inline.clearfix > h2".into(),
        }
    }

    fn get_date_selectors(&self) -> DateInfoSelectors {
        DateInfoSelectors::Same(SameDateInfoSelectors {
            created_regex: r"Posted\s((Today|Yesterday|(\d+\w+\s\w+\s+\d+))\sat\s\d+:\d+\s(PM|AM))"
                .into(),
            updated_regex:
                r"Updated\s((Today|Yesterday|(\d+\w+\s\w+\s+\d+))\sat\s\d+:\d+\s(PM|AM))".into(),
            selector: "div.well.profilepic.well-small.well-inline.clearfix > div:nth-last-child(1)"
                .into(),
        })
    }

    fn get_download_selector(&self) -> String {
        "#actualtab1 > table".into()
    }

    fn parse_date(&self, date: &str) -> Option<NaiveDateTime> {
        if date.starts_with("Today") || date.starts_with("Yesterday") {
            parse_relative_date(date)
        } else {
            parse_regular_date(date)
        }
    }
}

fn parse_regular_date(date: &str) -> Option<NaiveDateTime> {
    let ordinal_regex = Regex::new("st|nd|rd|th").unwrap();
    let date_without_ordinals = ordinal_regex.replace_all(date, "");

    NaiveDateTime::parse_from_str(&date_without_ordinals, "%e %b %Y at %l:%M %p").ok()
}

fn parse_relative_date(date: &str) -> Option<NaiveDateTime> {
    let is_today = date.starts_with("Today");
    let time_regex = Regex::new(r"(\d+:\d+\s(AM|PM))").unwrap();
    time_regex
        .captures(date)
        .and_then(|captures| captures.get(1))
        .and_then(|regex_match| NaiveTime::parse_from_str(regex_match.as_str(), "%l:%M %p").ok())
        .map(|time| {
            let relative_date = if is_today {
                get_today_date()
            } else {
                get_yesterday_date()
            };
            NaiveDateTime::new(relative_date.date(), time)
        })
}

fn get_today_date() -> NaiveDateTime {
    Utc::now().naive_utc()
}

fn get_yesterday_date() -> NaiveDateTime {
    get_today_date() - Duration::days(1)
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    use super::*;

    #[test]
    fn test_parse_regular_date() {
        let downloader = ModTheSimsDownloader::default();

        assert_eq!(
            NaiveDateTime::new(
                NaiveDate::from_ymd(2020, 2, 1),
                NaiveTime::from_hms(8, 10, 0)
            ),
            downloader.parse_date("1st Feb 2020 at 8:10 AM").unwrap()
        );

        assert_eq!(
            NaiveDateTime::new(
                NaiveDate::from_ymd(2020, 12, 2),
                NaiveTime::from_hms(22, 10, 0)
            ),
            downloader.parse_date("2nd Dec 2020 at 10:10 PM").unwrap()
        );

        assert_eq!(
            NaiveDateTime::new(
                NaiveDate::from_ymd(2020, 1, 3),
                NaiveTime::from_hms(21, 10, 0)
            ),
            downloader.parse_date("3rd Jan 2020 at 9:10 PM").unwrap()
        );

        assert_eq!(
            NaiveDateTime::new(
                NaiveDate::from_ymd(2020, 10, 7),
                NaiveTime::from_hms(11, 26, 0)
            ),
            downloader.parse_date("7th Oct 2020 at 11:26 AM").unwrap()
        );
    }

    #[test]
    fn test_parse_relative_date() {
        let downloader = ModTheSimsDownloader::default();

        assert_eq!(
            NaiveDateTime::new(
                Utc::now().naive_utc().date(),
                NaiveTime::from_hms(22, 39, 0)
            ),
            downloader.parse_date("Today at 10:39 PM").unwrap(),
        );

        assert_eq!(
            NaiveDateTime::new(
                Utc::now().naive_utc().date() - Duration::days(1),
                NaiveTime::from_hms(1, 9, 0)
            ),
            downloader.parse_date("Yesterday at 1:09 AM").unwrap(),
        );
    }
}
