use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const SUBFOLDER: &str = "input";
const LOGIN_FAILED_RESPONSE: &str =
    "Puzzle inputs differ by user.  Please log in to get your puzzle input.";

/// Fetches a puzzle input from the aoc website and caches the result under the subfolder `./input` in a text file.
/// Subsequent calls will use the cached result.
///
/// # Arguments
/// * `year` - year of the event, i.e. "2023"
/// * `day` - day of the event, i.e. "24"
/// * `path_to_cookie` - relative or absolute path to the file containing the session cookie
pub fn get_input(year: &str, day: &str, path_to_cookie: &str) -> String {
    let cookie = read_cookie(&path_to_cookie);
    let input_path = get_input_path(year, day);
    match fs::read_to_string(&input_path) {
        Err(_reason) => return fetch_input_from_site(year, day, &input_path, &cookie),
        Ok(value) => return value,
    };
}

fn read_cookie(path_to_cookie: &str) -> String {
    return fs::read_to_string(path_to_cookie).expect("Failed to read session cookie.");
}

fn get_input_path(year: &str, day: &str) -> PathBuf {
    let mut path = env::current_dir().expect("Couldn't read current dir.");
    path.push(SUBFOLDER);
    let mut yearday = String::from(year);
    yearday.push_str("_");
    yearday.push_str(day);
    path.push(yearday);
    path.set_extension("txt");
    path
}

fn fetch_input_from_site(year: &str, day: &str, input_path: &PathBuf, cookie: &str) -> String {
    let url = build_url(year, day);

    let jar = std::sync::Arc::new(reqwest::cookie::Jar::default());
    jar.add_cookie_str(cookie, &url);
    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .cookie_provider(std::sync::Arc::clone(&jar))
        .build()
        .unwrap();

    let response;
    match client.get(url).send() {
        Err(reason) => panic!("{}", reason),
        Ok(value) => response = value.text(),
    }
    match response {
        Err(reason) => panic!("{}", reason),
        Ok(value) if value == LOGIN_FAILED_RESPONSE => {
            panic!("Failed to fetch puzzle input. Make sure your session cookie is correct.")
        }
        Ok(value) => {
            if !Path::exists(&Path::new(SUBFOLDER)) {
                fs::create_dir(SUBFOLDER).unwrap();
            }
            fs::write(input_path, &value).unwrap();
            value
        }
    }
}

fn build_url(year: &str, day: &str) -> reqwest::Url {
    let mut url_as_str = String::from("https://adventofcode.com/");
    url_as_str.push_str(year);
    url_as_str.push_str("/day/");
    url_as_str.push_str(day);
    url_as_str.push_str("/input");
    return url_as_str.parse().unwrap();
}
