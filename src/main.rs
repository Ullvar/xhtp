use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use clap::Parser;
use serde_json::Value;

#[derive(Parser)]
struct Cli {
    first_arg: Option<String>,
}

fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("file not found");
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    lines
}

fn get_url_with_https(url: &str) -> String {
    if url.starts_with("http") {
        return url.to_string();
    } else {
        return format!("https://{}", url);
    }
}

fn get_content_type_from_header(headers: &reqwest::header::HeaderMap) -> String {
    let content_type = headers.get("content-type").unwrap();
    let content_type = content_type.to_str().unwrap();
    let content_type = content_type.split(";").collect::<Vec<&str>>();
    content_type[0].to_string()
}

fn arg_is_number(option: &str) -> bool {
    option.parse::<i32>().is_ok()
}

fn convert_option_to_number(option: &str) -> usize {
    option.parse::<usize>().unwrap()
}

fn get_url_from_saved_requests(saved_requests: &Vec<String>, index: usize) -> String {
    let url = &saved_requests[index - 1];
    let url = url.split(" ").collect::<Vec<&str>>();
    url[1].to_string()
}

fn print_saved_requests(saved_requests: &Vec<String>) {
    for (index, line) in saved_requests.iter().enumerate() {
        println!("{}: {}", index + 1, line);
    }
}
fn handle_add() {
    print!("Enter the http request method: ");
    std::io::stdout().flush().unwrap();
    let mut method = String::new();
    std::io::stdin().read_line(&mut method).unwrap();
    method = method.trim().to_string();

    print!("Enter the url: ");
    std::io::stdout().flush().unwrap();
    let mut url = String::new();
    std::io::stdin().read_line(&mut url).unwrap();
    url = url.trim().to_string();

    let full_request = format!("{} {}\n", method.to_uppercase(), url);
    println!("Saving request...");
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("saved_requests.txt")
        .unwrap();
    file.write_all(full_request.as_bytes()).unwrap();
}

fn handle_delete(saved_requests: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    print_saved_requests(&saved_requests);
    print!("Select the number of the request you want to delete: ");
    std::io::stdout().flush().unwrap();
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).unwrap();
    number = number.trim().to_string();

    let index = convert_option_to_number(&number);
    if index > saved_requests.len() {
        return Err("The number you passed is too big!".into());
    }
    saved_requests.remove(index - 1);
    let mut new_file = File::create("saved_requests.txt").unwrap();
    for line in saved_requests {
        let new_line = format!("{}\n", line);
        new_file.write_all(new_line.as_bytes()).unwrap();
    }
    return Ok(());
}

fn too_big(saved_requests: &Vec<String>) -> Result<(), reqwest::Error> {
        println!("The number you passed is too big!");
        println!("Here are your available options:");
        print_saved_requests(&saved_requests);
        return Ok(());
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut saved_requests = read_file("saved_requests.txt");

    let args = Cli::parse();

    let help_text = Some("help".to_string());
    let help_text2 = Some("h".to_string());

    if args.first_arg.is_none()
        || args.first_arg == help_text
        || args.first_arg == help_text2
        || args.first_arg == Some("".to_string())
    {
        println!(
            "
        Usage: 
        Do a simple GET request by passing a url as an argument, alternatively you can select one of the following options:
            list - list all the urls in the config file
            add - add a new url to the config file
            delete - delete a url from the config file
            help/h - show help
        "
        );
        return Ok(());
    }

    let first_arg = args.first_arg.as_ref().unwrap();

    if first_arg == "list" {
        println!("Pass the number of the request you want to use as an argument.");
        print_saved_requests(&saved_requests);
        return Ok(());
    } else if first_arg == "add" {
        handle_add();
        return Ok(());
    } else if first_arg == "delete" {
        let result = handle_delete(&mut saved_requests);
        if result.is_err() {
            return too_big(&saved_requests);
        }
        return Ok(());
    }

    let mut full_url = get_url_with_https(&first_arg);

    if arg_is_number(&first_arg) {
        let index = convert_option_to_number(&first_arg);
        if index > saved_requests.len() {
            return too_big(&saved_requests);
        }
        let partial_url = get_url_from_saved_requests(&saved_requests, index);
        full_url = get_url_with_https(&partial_url);
    }

    let res = reqwest::get(full_url).await?;

    println!("{}", res.status());

    let content_type = get_content_type_from_header(res.headers());

    if content_type == "application/json" {
        let res_text = res.text().await?;
        let json: Value = serde_json::from_str(&res_text).unwrap();

        println!("{:#}", json);
    } else {
        println!("{}", res.text().await?);
    }

    Ok(())
}
