use std::str::FromStr;
use num::complex::Complex;


/// Parse a delimited number pair from string with a given delimiter
///
pub fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
    match s.find(sep) {
        None => None,
        Some(pos) => {
            match (T::from_str(&s[..pos]), T::from_str(&s[pos + 1..])) {
                (Ok(p1), Ok(p2)) => Some((p1, p2)),
                _ => None,
            }
        },
    }
}

/// Parse separated by a comma floating-point number pair from a given string
/// 
/// Return complex number
///
pub fn parse_complex(s: &str /* , sep: char */) -> Option<Complex<f64>> {
    if let Some((re, im)) = parse_pair(s, ',' /*, sep */) {
        return Some(Complex { re, im });
    }
    None
}



#[cfg(test)]
mod test_cli {
    use num::complex::Complex;
    use super::utils;

    #[test]
    fn test_cli_parser() {
        assert_eq!(utils::parse_pair::<i32>("", ','), None);
        assert_eq!(utils::parse_pair::<i32>("10,", ','), None);
        assert_eq!(utils::parse_pair::<i32>("10,20", ','), Some((10, 20)));
        assert_eq!(utils::parse_pair::<i32>("10,20xy", ','), None);
        assert_eq!(utils::parse_pair::<f64>("0.5x", 'x'), None);
        assert_eq!(utils::parse_pair::<f64>("0.5x10", 'x'), 
                   Some((0.5, 10.0))
        );
    }

    #[test]
    fn test_parse_complex_pair() {
        assert_eq!(
            utils::parse_complex("32.2,43.0"),
            Some(Complex { re: 32.2, im: 43.0 })
        );
        assert_eq!(
            utils::parse_complex("2.2,-0.00032"),
            Some(Complex { re: 2.2, im: -0.00032 })
        );
        assert_eq!(utils::parse_complex(",-0.00032"), None);

    }
}
