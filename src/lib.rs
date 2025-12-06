mod tests;

use std::fmt::{Display, Formatter};
use chrono::{Datelike, Duration, NaiveDate, Weekday};

/// Returns Easter Sunday for a given year (Gregorian calendar)
fn paaszondag(year: i32) -> Option<NaiveDate> {
    // From python dateutil/src/dateutil/easter.py
    // (Apache 2.0)
    // Copyright 2019 The Apache Software Foundation
    let y = year;
    let g = y % 19;
    let e = 0;
    let c = y / 100;
    let h = (c - c / 4 - (8 * c + 13) / 25 + 19 * g + 15) % 30;
    let i = h - (h / 28) * (1 - (h / 28) * (29 / (h + 1)) * ((21 - g) / 11));
    let j = (y + y / 4 + i + 2 - c + c / 4) % 7;
    let p = i - j + e;
    let d = 1 + (p + 27 + (p + 6) / 40) % 31;
    let m = 3 + (p + 26) / 30;
    NaiveDate::from_ymd_opt(y, m as u32, d as u32)
}

pub fn hemelvaartsdag(year: i32) -> Option<NaiveDate> {
    paaszondag(year).map(|d| d + Duration::days(39))
}

pub fn goede_vrijdag(year: i32) -> Option<NaiveDate> {
    paaszondag(year).map(|d| d - Duration::days(2))
}

pub fn paasmaandag(year: i32) -> Option<NaiveDate> {
    paaszondag(year).map(|d| d + Duration::days(1))
}

pub fn eerste_pinksterdag(year: i32) -> Option<NaiveDate> {
    hemelvaartsdag(year).map(|d| d + Duration::days(10))
}

pub fn tweede_pinksterdag(year: i32) -> Option<NaiveDate> {
    hemelvaartsdag(year).map(|d| d + Duration::days(11))
}

pub fn koningsdag(year: i32) -> Option<NaiveDate> {
    let kings_birthday = NaiveDate::from_ymd_opt(year, 4, 27);
    if kings_birthday.is_none() {
        return None;
    }
    if kings_birthday.unwrap().weekday() == Weekday::Sun {
        return Some(kings_birthday.unwrap() - Duration::days(1));
    }
    kings_birthday
}

pub fn oudejaarsdag(year: i32) -> Option<NaiveDate> {
    NaiveDate::from_ymd_opt(year, 12, 31)
}

pub fn nieuwjaarsdag(year: i32) -> Option<NaiveDate> {
    NaiveDate::from_ymd_opt(year, 1, 1)
}

pub fn bevrijdingsdag(year: i32) -> Option<NaiveDate> {
    NaiveDate::from_ymd_opt(year, 5, 5)
}

pub fn eerste_kerstdag(year: i32) -> Option<NaiveDate> {
    NaiveDate::from_ymd_opt(year, 12, 25)
}

pub fn tweede_kerstdag(year: i32) -> Option<NaiveDate> {
    NaiveDate::from_ymd_opt(year, 12, 26)
}

#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Ord, Eq, Hash)]
pub enum Holiday {
    PaasZondag,
    PaasMaandag,
    Hemelvaartsdag,
    GoedeVrijdag,
    EerstePinksterdag,
    TweedePinksterdag,
    Koningsdag,
    Oudejaarsdag,
    Nieuwjaarsdag,
    Bevrijdingsdag,
    EersteKerstdag,
    TweedeKerstdag,
}

impl Display for Holiday {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Holiday::PaasZondag => "Paaszondag",
            Holiday::PaasMaandag => "Paasmaandag",
            Holiday::Hemelvaartsdag => "Hemelvaartsdag",
            Holiday::GoedeVrijdag => "Goede Vrijdag",
            Holiday::EerstePinksterdag => "Eerste Pinksterdag",
            Holiday::TweedePinksterdag => "Tweede Pinksterdag",
            Holiday::Koningsdag => "Koningsdag",
            Holiday::Oudejaarsdag => "Oudejaarsdag",
            Holiday::Nieuwjaarsdag => "Nieuwjaarsdag",
            Holiday::Bevrijdingsdag => "Bevrijdingsdag",
            Holiday::EersteKerstdag => "Eerste Kerstdag",
            Holiday::TweedeKerstdag => "Tweede Kerstdag",
        })
    }
}

pub fn get_holiday(date: NaiveDate) -> Option<Holiday> {
    if paaszondag(date.year()) == Some(date) {
        return Some(Holiday::PaasZondag);
    }
    if hemelvaartsdag(date.year()) == Some(date) {
        return Some(Holiday::Hemelvaartsdag);
    }
    if goede_vrijdag(date.year()) == Some(date) {
        return Some(Holiday::GoedeVrijdag);
    }
    if paasmaandag(date.year()) == Some(date) {
        return Some(Holiday::PaasMaandag);
    }
    if eerste_pinksterdag(date.year()) == Some(date) {
        return Some(Holiday::EerstePinksterdag);
    }
    if tweede_pinksterdag(date.year()) == Some(date) {
        return Some(Holiday::TweedePinksterdag);
    }
    if koningsdag(date.year()) == Some(date) {
        return Some(Holiday::Koningsdag);
    }
    if oudejaarsdag(date.year()) == Some(date) {
        return Some(Holiday::Oudejaarsdag);
    }
    if nieuwjaarsdag(date.year()) == Some(date) {
        return Some(Holiday::Nieuwjaarsdag);
    }
    if bevrijdingsdag(date.year()) == Some(date) {
        return Some(Holiday::Bevrijdingsdag);
    }
    if eerste_kerstdag(date.year()) == Some(date) {
        return Some(Holiday::EersteKerstdag);
    }
    if tweede_kerstdag(date.year()) == Some(date) {
        return Some(Holiday::TweedeKerstdag);
    }
    None
}

impl Holiday {
    pub fn all() -> [Holiday; 12] {
        [
            Holiday::PaasZondag,
            Holiday::PaasMaandag,
            Holiday::Hemelvaartsdag,
            Holiday::GoedeVrijdag,
            Holiday::EerstePinksterdag,
            Holiday::TweedePinksterdag,
            Holiday::Koningsdag,
            Holiday::Oudejaarsdag,
            Holiday::Nieuwjaarsdag,
            Holiday::Bevrijdingsdag,
            Holiday::EersteKerstdag,
            Holiday::TweedeKerstdag,
        ]
    }
}