use std::fs;

const MODULE_TEMPLATE: &str = r###"
pub fn main() {
    let problem_input = include_str!("../inputs/<DAY>.txt");
}
"###;

fn main() {
    let day = std::env::args().nth(1).expect("no day given");
    download_input(&day);
    scaffold_bin(&day);
}

fn scaffold_bin(day: &String) {
    fs::write(
        format!("src/bin/day{}.rs", day),
        MODULE_TEMPLATE.replace("<DAY>", day)
    ).unwrap();
}

fn download_input(day: &String) {
    let client = reqwest::blocking::Client::new();
    let input = client
        .get(format!("https://adventofcode.com/2022/day/{}/input", day))
        .header("Cookie", "session=xxx")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:107.0) Gecko/20100101 Firefox/107.0")
        .send()
        .unwrap();
    fs::write(format!("src/inputs/{}.txt", day), &input.bytes().unwrap()).unwrap();
}


