use extendr_api::prelude::*;

/// Funcion para obtener el mes para un determinado isoweek
///
/// @param years año actual
/// @param weeks semana iso a consultar
///
/// @return isoweek
/// @export
#[extendr]
fn get_iso_month(years: Doubles, weeks: Doubles) -> Vec<i32> {
    let mut months = Vec::with_capacity(years.len());
    for i in 0..years.len() {
        let year = years[i].inner() as i32;
        let week = weeks[i].inner() as i32;
        let iso_month = match (year, week) {
            (2024, 1..=4) => 1,
            (2024, 5..=8) => 2,
            (2024, 9..=13) => 3,
            (2024, 14..=17) => 4,
            (2024, 18..=22) => 5,
            (2024, 23..=26) => 6,
            (2024, 27..=30) => 7,
            (2024, 31..=35) => 8,
            (2024, 36..=39) => 9,
            (2024, 40..=44) => 10,
            (2024, 45..=48) => 11,
            (2024, 49..=52) => 12,
            _ => panic!("Fecha invalida"),
        };
        months.push(iso_month);
    }
    months
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod iso8601;
    fn get_iso_month;
}
