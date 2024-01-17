use chrono::Datelike;

//将时间类型转化成 July 20, 2021
pub fn date_to_string(date: chrono::NaiveDate) -> String {
    let month = match date.month() {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    };
    format!("{} {}, {}",  date.day(),month, date.year())
}
