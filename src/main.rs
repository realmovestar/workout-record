use std::io::{self, BufRead};

trait Record {
    fn parse_record(&self) -> (&str, u32);
}

impl Record for &str {
    /// 10p 15d 12 와 같은 값을 kind, count 로 분리한다.
    fn parse_record(&self) -> (&str, u32) {
        match self.find(char::is_alphabetic) {
            Some(i) => (
                &self[i..],
                self[..i]
                    .parse::<u32>()
                    .unwrap_or_else(|_| panic!("파싱 불가 - {}", &self)),
            ),
            None => (
                "p",
                self.parse()
                    .unwrap_or_else(|_| panic!("파싱 불가 - {}", &self)),
            ),
        }
    }
}

fn long_kind(kind: &str) -> &str {
    match kind {
        "p" => "pullup",
        "d" => "dips",
        "m" => "muscleup",
        "s" => "squat",
        "c" => "chinup",
        _ => "unknown",
    }
}

fn main() {
    println!("y,ym,ymd,kind,count");
    let stdin = io::stdin();
    for line in stdin
        .lock()
        .lines()
        .filter(|l| !l.as_ref().unwrap().is_empty())
        .filter(|l| !l.as_ref().unwrap().starts_with('#'))
    {
        let line = line.expect("Could not read line from standard in");
        let mut line_iter = line.split_whitespace();
        let ymd_part = line_iter.next().unwrap();
        let ymd: Vec<&str> = ymd_part.split('-').collect();
        let y = ymd[0];
        let m = ymd[1];
        let d = ymd[2];
        for record in line_iter {
            let (kind, count) = record.parse_record();
            println!(
                "{},{}-{},{}-{}-{},{},{}",
                y,
                y,
                m,
                y,
                m,
                d,
                long_kind(kind),
                count
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_record() {
        assert_eq![("d", 10), "10d".parse_record()];
        assert_eq![("p", 15), "15".parse_record()];
    }

    #[test]
    #[should_panic]
    fn test_parse_error() {
        "p10".parse_record();
    }
}
