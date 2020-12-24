// ==== day 4

use crate::utils;
use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

static HGT_RE: Lazy<regex::Regex> =
    Lazy::new(|| Regex::new(r"^[0-9]+(cm|in)$").expect("regex compilation failure"));
static HCL_RE: Lazy<regex::Regex> =
    Lazy::new(|| Regex::new(r"^#[0-9a-f]{6}$").expect("regex compilation failure"));
static ECL_RE: Lazy<regex::Regex> = Lazy::new(|| {
    Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").expect("regex compilation failure")
});
static PID_RE: Lazy<regex::Regex> =
    Lazy::new(|| Regex::new(r"^[0-9]{9}$").expect("regex compilation failure"));
static PASSPORT_RE: Lazy<regex::Regex> = Lazy::new(|| {
    Regex::new(r"(byr|iyr|eyr|hgt|hcl|ecl|pid|cid):(\S*)").expect("regex compilation failure")
});

impl Passport {
    // true if all the required fields (valid or otherwise) are present (i.e. all but "cid")
    fn required_fields_present(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    // true if all the required fields are both present and valid
    fn is_valid(&self) -> bool {
        self.required_fields_present()
            && self.is_byr_valid()
            && self.is_iyr_valid()
            && self.is_eyr_valid()
            && self.is_hgt_valid()
            && self.is_hcl_valid()
            && self.is_ecl_valid()
            && self.is_pid_valid()
            && self.is_cid_valid()
    }

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    fn is_byr_valid(&self) -> bool {
        self.byr.as_ref().map_or(false, |s| {
            utils::parse_usize_in_range(&s, 1920, 2002).is_some()
        })
    }

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    fn is_iyr_valid(&self) -> bool {
        self.iyr.as_ref().map_or(false, |s| {
            utils::parse_usize_in_range(&s, 2010, 2020).is_some()
        })
    }

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    fn is_eyr_valid(&self) -> bool {
        self.eyr.as_ref().map_or(false, |s| {
            utils::parse_usize_in_range(&s, 2020, 2030).is_some()
        })
    }

    // hgt (Height) - a number followed by either cm or in:
    //    If cm, the number must be at least 150 and at most 193.
    //    If in, the number must be at least 59 and at most 76.
    fn is_hgt_valid(&self) -> bool {
        if let Some(s) = &self.hgt {
            if HGT_RE.is_match(s) {
                if s.ends_with("cm") {
                    return utils::parse_usize_in_range(&s[..s.len() - 2], 150, 193).is_some();
                } else if s.ends_with("in") {
                    return utils::parse_usize_in_range(&s[..s.len() - 2], 59, 76).is_some();
                }
            }
        }
        false
    }

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    fn is_hcl_valid(&self) -> bool {
        self.hcl.as_ref().map_or(false, |s| HCL_RE.is_match(s))
    }

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    fn is_ecl_valid(&self) -> bool {
        self.ecl.as_ref().map_or(false, |s| ECL_RE.is_match(s))
    }

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    fn is_pid_valid(&self) -> bool {
        self.pid.as_ref().map_or(false, |s| PID_RE.is_match(s))
    }

    // cid (Country ID) - ignored, missing or not.
    #[allow(clippy::unused_self)]
    fn is_cid_valid(&self) -> bool {
        true
    }
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, anyhow::Error> {
        let mut ret: Self = Passport::default();
        s.split_whitespace().for_each(|s| {
            PASSPORT_RE.captures_iter(&s).for_each(|cap| match &cap[1] {
                "byr" => ret.byr = Some(cap[2].to_string()),
                "iyr" => ret.iyr = Some(cap[2].to_string()),
                "eyr" => ret.eyr = Some(cap[2].to_string()),
                "hgt" => ret.hgt = Some(cap[2].to_string()),
                "hcl" => ret.hcl = Some(cap[2].to_string()),
                "ecl" => ret.ecl = Some(cap[2].to_string()),
                "cid" => ret.cid = Some(cap[2].to_string()),
                "pid" => ret.pid = Some(cap[2].to_string()),
                _ => panic!("warning: unknown attribute: {}:{}", &cap[1], &cap[2]),
            });
        });
        Ok(ret)
    }
}

pub fn run() -> Result<(usize, usize)> {
    let data = utils::read_file("data/input-04.txt")?;
    let answers = utils::read_i64s("data/output-04.txt")?;

    // parse into an array of Passport{}
    let passports: Vec<Passport> = data
        .split("\n\n")
        .map(|s| Passport::from_str(&s).expect("error parsing passport"))
        .collect();

    // count passports with all req'd fields
    let p1 = passports
        .iter()
        .filter(|p| p.required_fields_present())
        .count();
    assert_eq!(p1, answers[0] as usize);

    // count passports with all req'd and validated fields
    let p2 = passports.iter().filter(|p| p.is_valid()).count();
    assert_eq!(p2, answers[1] as usize);

    Ok((p1, p2))
}
