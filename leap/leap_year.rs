pub fn is_leap_year(year: int) -> bool {
    mod4(year) && (mod400(year) || _mod100(year))
}

fn mod4(year: int) -> bool {
    year % 4 == 0
}

fn _mod100(year: int) -> bool {
    year % 100 > 0
}

fn mod400(year: int) -> bool {
    year % 400 == 0
}
