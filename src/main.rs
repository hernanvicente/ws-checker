use std::env;
use std::error::Error;

use color_print::cprintln;
use csv::Writer;
use reqwest::{Url};
use serde::{Serialize, Deserialize};

use std::thread;
use std::time::Duration;
use indicatif::ProgressBar;

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    code: i32,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Row {
    domain: String,
    code: i32,
    protocol: String,
    www: bool,
}

// Set the user agent as Chrome
static APP_USER_AGENT: &str = concat!(
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"
);

fn main() {
    println!("Checking...");

    let mut args: Vec<String> = env::args().collect();

    // Remove first argument which is self
    args.remove(0);

    match args.pop() {
        Some(domain) => {
            check_domain(&domain, None);
        },
        None => {
            let domains = read_domains_from_file();

            println!("No argument domain provided, checking {} domains from file", domains.len());
            let all_responses = check_all_domains(domains);

            write_domains_to_file(all_responses);
        },
    };
}

fn check_domain(domain: &str, try_number: Option<usize>) -> Response {
    let index = try_number.unwrap_or(0) as usize;
    let urls = build_urls(&domain);

    // Try next url if index is less than urls length
    match index {
      0..=3 => {
          let url = &urls[index];
          let response = fetch_homepage(&url);

          match response {
              Ok(resp) => Response { code: resp.code, url: resp.url },
              _ => { check_domain(domain, Some(index + 1)) },
          }
      },
      _ => {
          return Response { code: 0, url: domain.to_string(), };
      }
    }
}

fn write_domains_to_file(responses: Vec<Response>) -> Result<(), Box<dyn Error>> {
    cprintln!("<cyan>Writing to file...</cyan>");
    let mut wtr = Writer::from_path("./report.csv")?;

    for response in responses {
        let url = Url::parse(&response.url);

        let protocol = match url {
            Ok(url) => url.scheme().to_string(),
            _ => "unknown".to_string(),
        };

        let www = response.url.contains("www");

        let row = Row {
            domain: response.url.to_string(),
            code: response.code,
            protocol,
            www,
        };
        wtr.serialize(row)?;
    }

    // let data = String::from_utf8(wtr.into_inner()?)?;
    // wtr.flush()?;
    cprintln!("<cyan>Done writing to file!</cyan>");
    Ok(())
}

fn check_all_domains(domains: Vec<String>) -> Vec<Response> {
    let pb = ProgressBar::new(domains.len() as u64);
    let mut responses: Vec<Response> = Vec::new();

    for domain in domains {
        pb.inc(1);
        responses.push(check_domain(&domain, None));
        thread::sleep(Duration::from_millis(5));
    }

    pb.finish_with_message("All request done");
    responses
}

fn read_domains_from_file() -> Vec<String> {
    // Build the CSV reader and iterate over each record.
    let mut vec = Vec::new();
    let reader = csv::Reader::from_path("./domains.csv");

    for result in reader.unwrap().records() {
        let record = result.unwrap();
        vec.push(record[0].to_string());
    }

    // vec.push("google.com".to_string());
    vec
}

fn build_urls(domain: &str) -> Vec<String> {
    vec![
        format!("https://www.{}", domain),
        format!("https://{}", domain),
        format!("http://www.{}", domain),
        format!("http://{}", domain),
    ]
}

fn fetch_homepage(url: &str) -> Result<Response, Box<dyn Error>> {
    let resp = client().get(url).send()?;
    let headers = resp.headers();

    cprintln!("<blue>{:?} - {:?}</blue>", url, resp.status());
    cprintln!("<red>{:?} - {:?}</red>", url, headers);

    Ok(Response { code: resp.status().as_u16() as i32, url: url.to_string() })
}

fn client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap()
}
