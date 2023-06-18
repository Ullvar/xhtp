use crate::structs::{ExtractVariable, GlobalVariable, HttpRequest};
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, Write};

pub fn print_line(text: &str) {
    println!("\x1b[94m{}\x1b[0m", text);
}

pub fn print(text: &str) {
    print!("\x1b[94m{}\x1b[0m", text);
}

pub fn get_url_with_https(url: &str) -> String {
    if url.starts_with("http") {
        return url.to_string();
    } else {
        return format!("https://{}", url);
    }
}

pub fn get_content_type_from_header(headers: &reqwest::header::HeaderMap) -> String {
    let content_type = headers.get("content-type").unwrap();
    let content_type = content_type.to_str().unwrap();
    let content_type = content_type.split(";").collect::<Vec<&str>>();
    content_type[0].to_string()
}

pub fn arg_is_number(option: &str) -> bool {
    option.parse::<i32>().is_ok()
}

pub fn convert_option_to_number(option: &str) -> usize {
    option.parse::<usize>().unwrap()
}

pub fn get_request_from_saved_requests(
    saved_requests: &Vec<HttpRequest>,
    index: usize,
) -> &HttpRequest {
    let request = &saved_requests[index - 1];
    request
}

fn get_ansi_colored_request_method(method: &str) -> String {
    match method {
        "GET" => format!("\x1b[32m{}\x1b[0m", method),
        "POST" => format!("\x1b[33m{}\x1b[0m", method),
        "PUT" => format!("\x1b[34m{}\x1b[0m", method),
        "DELETE" => format!("\x1b[31m{}\x1b[0m", method),
        _ => method.to_string(),
    }
}

fn get_extract_variables_list(extract_variables: &Option<Vec<ExtractVariable>>) -> String {
    let mut extract_variables_list = String::new();
    if extract_variables.is_some() {
        let extract_variables = extract_variables.as_ref().unwrap();
        for (index, variable) in extract_variables.iter().enumerate() {
            if index == 0 {
                extract_variables_list = format!(
                    "{} -> {{{{{}}}}}",
                    variable.key_path, variable.variable_name
                );
            } else {
                extract_variables_list = format!(
                    "{}, {} -> {{{{{}}}}}",
                    extract_variables_list, variable.key_path, variable.variable_name
                );
            }
        }
    }
    extract_variables_list
}

pub fn print_saved_requests(saved_requests: &Vec<HttpRequest>) {
    for (index, request) in saved_requests.iter().enumerate() {
        println!(
            "{}: {} {}",
            index + 1,
            get_ansi_colored_request_method(request.method.as_str()),
            request.url,
        );
    }
}

pub fn print_full_saved_request_from_index(saved_requests: &Vec<HttpRequest>, index: usize) {
    let request = get_request_from_saved_requests(saved_requests, index);
    println!(
        "{}: {} {}",
        index,
        get_ansi_colored_request_method(request.method.as_str()),
        request.url,
    );
    if request.extract_variables.is_some() {
        print_line("Extract variables:");
        println!("{}", get_extract_variables_list(&request.extract_variables));
    }
    print_line("Headers:");
    for header in &request.headers {
        println!("- {}", header);
    }
    if request.body.is_some() {
        print_line("Body:");
        let json = serde_json::to_string_pretty(request.body.as_ref().unwrap()).unwrap();
        println!("{:#}", json);
    }
}

pub fn handle_add(requests: &mut Vec<HttpRequest>) {
    print("Enter the http request method: ");
    std::io::stdout().flush().unwrap();
    let mut method = String::new();
    std::io::stdin().read_line(&mut method).unwrap();
    method = method.trim().to_string();
    method = method.to_uppercase();

    let allowed_methods = vec!["GET", "POST", "PUT", "DELETE"];
    if !allowed_methods.contains(&method.trim()) {
        print_line("Invalid method, must be one of GET, POST, PUT, DELETE");
        return;
    }

    print("Enter the url: ");
    std::io::stdout().flush().unwrap();
    let mut url = String::new();
    std::io::stdin().read_line(&mut url).unwrap();
    url = url.trim().to_string();

    print_line("Enter the headers one by one. Input (s) to save when done");
    let mut headers = Vec::new();
    loop {
        print("Enter the header: ");
        std::io::stdout().flush().unwrap();
        let mut header = String::new();
        std::io::stdin().read_line(&mut header).unwrap();
        header = header.trim().to_string();
        if header == "s" {
            break;
        }
        headers.push(header);
    }

    let mut body = Value::Null;
    if method != "GET" && method != "DELETE" {
        let mut string_body = String::new();
        print("Enter the body: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut string_body).unwrap();
        body = serde_json::from_str(&string_body).unwrap();
    }

    let mut extract_variables = None;
    print("Do you want to extract variables from the response? (y/n): ");
    std::io::stdout().flush().unwrap();
    let mut extract_variables_response = String::new();
    std::io::stdin()
        .read_line(&mut extract_variables_response)
        .unwrap();
    extract_variables_response = extract_variables_response.trim().to_string();

    if extract_variables_response == "y" {
        print_line(
            "Enter the variables you want to extract one by one. Input (s) to save when done",
        );
        let mut variables = Vec::new();
        loop {
            print("Enter the name of the variable you want to save to: ");
            std::io::stdout().flush().unwrap();
            let mut variable = String::new();
            std::io::stdin().read_line(&mut variable).unwrap();
            variable = variable.trim().to_string();
            if variable == "s" {
                break;
            }
            let mut key_path = String::new();
            print("Enter the key path: ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut key_path).unwrap();
            key_path = key_path.trim().to_string();
            let extract_variable = ExtractVariable {
                key_path,
                variable_name: variable,
            };
            variables.push(extract_variable);
        }
        extract_variables = Some(variables);
    }

    let request = HttpRequest {
        method,
        url,
        headers,
        body: Some(body),
        extract_variables,
    };

    print_line("Saving request...");

    requests.push(request);

    let mut file = File::create("requests.json").unwrap();

    let json = serde_json::to_string(&requests).unwrap();

    file.write_all(json.as_bytes()).unwrap();
}

pub fn handle_delete(requests: &mut Vec<HttpRequest>) -> Result<(), Box<dyn std::error::Error>> {
    print_saved_requests(&requests);
    print("Select the number of the request you want to delete: ");
    std::io::stdout().flush().unwrap();
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).unwrap();
    number = number.trim().to_string();

    let index = convert_option_to_number(&number);
    if index > requests.len() {
        return Err("The number you passed is too big!".into());
    }
    requests.remove(index - 1);

    let json = serde_json::to_string(&requests)?;

    let mut file = File::create("requests.json")?;
    file.write_all(json.as_bytes())?;

    return Ok(());
}

pub fn too_big(saved_requests: &Vec<HttpRequest>) -> Result<(), reqwest::Error> {
    print_line("The number you passed is too big!");
    print_line("Here are your available options:");
    print_saved_requests(&saved_requests);
    return Ok(());
}

pub fn get_json_value<'a>(json: &'a Value, key: &str) -> Option<&'a Value> {
    json.get(key)
}

pub fn get_global_variables() -> Vec<GlobalVariable> {
    if !File::open("global_variables.json").is_ok() {
        // File does not exist, create it
        let mut file = File::create("global_variables.json").expect("Failed to create file");
        file.write_all("[]".as_bytes())
            .expect("Failed to create file");
    }
    let file = File::open("global_variables.json").expect("Failed to open file");
    let reader = BufReader::new(file);
    let global_variables: Vec<GlobalVariable> =
        serde_json::from_reader(reader).expect("Failed to parse JSON");
    global_variables
}

fn add_global_variable() -> Result<(), Box<dyn std::error::Error>> {
    let mut global_variables = get_global_variables();
    print("Enter the name of the variable: ");
    std::io::stdout().flush().unwrap();
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    print("Enter the value of the variable: ");
    std::io::stdout().flush().unwrap();
    let mut value = String::new();
    std::io::stdin().read_line(&mut value).unwrap();
    value = value.trim().to_string();

    let global_variable = GlobalVariable { key: name, value };
    global_variables.push(global_variable);

    let json = serde_json::to_string(&global_variables)?;

    let mut file = File::create("global_variables.json")?;
    file.write_all(json.as_bytes())?;

    return Ok(());
}

fn delete_global_variable() -> Result<(), Box<dyn std::error::Error>> {
    let mut global_variables = get_global_variables();
    print("Enter the number of the variable you want to delete: ");
    std::io::stdout().flush().unwrap();
    let mut index_str = String::new();
    std::io::stdin().read_line(&mut index_str).unwrap();
    index_str = index_str.trim().to_string();

    let index = convert_option_to_number(&index_str);
    global_variables.remove(index - 1);

    let json = serde_json::to_string(&global_variables)?;

    let mut file = File::create("global_variables.json")?;
    file.write_all(json.as_bytes())?;

    return Ok(());
}

pub fn handle_global_variables() -> Result<(), reqwest::Error> {
    let global_variables = get_global_variables();
    print_line("Here are your global variables:");
    for (index, global_variable) in global_variables.iter().enumerate() {
        println!(
            "{}. {{{{{}}}}}: {}",
            index + 1,
            global_variable.key,
            global_variable.value
        );
    }

    print_line("Select (a) to add a variable, (d) to delete a variable");
    print("Enter your choice: ");
    std::io::stdout().flush().unwrap();
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();
    choice = choice.trim().to_string();

    if choice == "a" {
        let res = add_global_variable();
        if res.is_err() {
            print_line("Failed to add global variable: ");
            println!("{}", res.err().unwrap());
            return Ok(());
        } else {
            print_line("Successfully added global variable!");
            return Ok(());
        }
    } else if choice == "d" {
        let res = delete_global_variable();
        if res.is_err() {
            print("Failed to delete global variable: ");
            println!("{}", res.err().unwrap());
            return Ok(());
        } else {
            print_line("Successfully deleted global variable!");
            return Ok(());
        }
    } else {
        print_line("Invalid choice!");
        return Ok(());
    }
}
