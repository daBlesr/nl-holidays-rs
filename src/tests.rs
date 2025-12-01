
#[cfg(test)]
mod tests {
    use crate::get_holiday;
    use crate::Holiday::*;
    use chrono::NaiveDate;

    #[test]
    fn test_get_holiday() {
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()), Some(Nieuwjaarsdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2023, 4, 7).unwrap()), Some(GoedeVrijdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2023, 4, 9).unwrap()), Some(PaasZondag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2023, 4, 10).unwrap()), Some(PaasMaandag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2023, 4, 27).unwrap()), Some(Koningsdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2023, 5, 5).unwrap()), Some(Bevrijdingsdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2023, 5, 18).unwrap()), Some(Hemelvaartsdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2023, 5, 28).unwrap()), Some(EerstePinksterdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2023, 5, 29).unwrap()), Some(TweedePinksterdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2023, 12, 25).unwrap()), Some(EersteKerstdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2023, 12, 26).unwrap()), Some(TweedeKerstdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()), Some(Oudejaarsdag));

        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()), Some(Nieuwjaarsdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2024, 3, 29).unwrap()), Some(GoedeVrijdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2024, 3, 31).unwrap()), Some(PaasZondag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2024, 4, 1).unwrap()), Some(PaasMaandag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2024, 4, 27).unwrap()), Some(Koningsdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2024, 5, 5).unwrap()), Some(Bevrijdingsdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2024, 5, 9).unwrap()), Some(Hemelvaartsdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2024, 5, 19).unwrap()), Some(EerstePinksterdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2024, 5, 20).unwrap()), Some(TweedePinksterdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2024, 12, 25).unwrap()), Some(EersteKerstdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2024, 12, 26).unwrap()), Some(TweedeKerstdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()), Some(Oudejaarsdag));

        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()), Some(Nieuwjaarsdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2025, 4, 18).unwrap()), Some(GoedeVrijdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2025, 4, 20).unwrap()), Some(PaasZondag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2025, 4, 21).unwrap()), Some(PaasMaandag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2025, 4, 26).unwrap()), Some(Koningsdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2025, 5, 5).unwrap()), Some(Bevrijdingsdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2025, 5, 29).unwrap()), Some(Hemelvaartsdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2025, 6, 8).unwrap()), Some(EerstePinksterdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2025, 6, 9).unwrap()), Some(TweedePinksterdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2025, 12, 25).unwrap()), Some(EersteKerstdag));
        assert_eq!(get_holiday(NaiveDate::from_ymd_opt(2025, 12, 26).unwrap()), Some(TweedeKerstdag));
    }
}