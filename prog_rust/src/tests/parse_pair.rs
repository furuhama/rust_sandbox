use std::str::FromStr;

#[allow(dead_code)]
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(idx) => {
            match (T::from_str(&s[..idx]), T::from_str(&s[idx + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_pair() {
        assert_eq!(parse_pair::<i32>("", ','), None);
        assert_eq!(parse_pair::<i32>("10,", ','), None);
        assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
        assert_eq!(parse_pair::<i32>("10x20", 'x'), Some((10, 20)));
    }
}
