// Use regular expressions

use regex::Regex;

pub fn regexp() {
    let re = Regex::new(r"(?x)
(?P<year>\d{4})  # the year
-
(?P<month>\d{2}) # the month
-
(?P<day>\d{2})   # the day
").unwrap();
    let caps = re.captures("2010-03-14").unwrap();

    println!("yaer: {}", &caps["year"]);
    println!("month: {}", &caps["month"]);
    println!("day: {}", &caps["day"]);

    let re = Regex::new(r"ぇ").unwrap();
    let res = re.find("ふぇぇ").unwrap();
    println!("{:?}", res);
    println!("{:?}", res.as_str());
}
