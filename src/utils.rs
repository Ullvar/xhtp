use crate::openapi_structs::OpenAPI;
use crate::structs::{ExtractVariable, GlobalVariable, HttpRequest};
use dirs::home_dir;
use serde_json::Value;
use std::fs::{self, File};
use std::io::{BufReader, Write};

pub fn get_home_path() -> String {
    if let Some(path) = home_dir() {
        return path.to_str().unwrap().to_string();
    }
    return "".to_string();
}

pub fn get_http_requests_file_path() -> String {
    format!("{}/.xhtp/requests.json", get_home_path())
}

pub fn get_global_variables_file_path() -> String {
    format!("{}/.xhtp/global_variables.json", get_home_path())
}

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

pub fn get_ansi_colored_request_method(method: &str) -> String {
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

    let mut file = File::create(get_http_requests_file_path())?;
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
    if !File::open(get_global_variables_file_path()).is_ok() {
        // File does not exist, create it
        let mut file =
            File::create(get_global_variables_file_path()).expect("Failed to create file");
        file.write_all("[]".as_bytes())
            .expect("Failed to create file");
    }
    let file = File::open(get_global_variables_file_path()).expect("Failed to open file");
    let reader = BufReader::new(file);
    let global_variables: Vec<GlobalVariable> =
        serde_json::from_reader(reader).expect("Failed to parse JSON");
    global_variables
}

pub fn save_to_global_variables(key: String, value: String) {
    let mut global_variables = get_global_variables();
    let global_variable = GlobalVariable { key, value };

    if let Some(index) = global_variables
        .iter()
        .position(|x| x.key == global_variable.key)
    {
        global_variables[index] = global_variable;
    } else {
        global_variables.push(global_variable);
    }

    let mut file = File::create(get_global_variables_file_path()).unwrap();
    let json = serde_json::to_string(&global_variables).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn delete_global_variable(index_str: String) {
    let mut global_variables = get_global_variables();

    let index = convert_option_to_number(&index_str);
    global_variables.remove(index - 1);

    let json = serde_json::to_string(&global_variables).unwrap();

    let mut file = File::create(get_global_variables_file_path()).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn list_global_variables() {
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
}

//pub fn handle_global_variables() -> Result<(), reqwest::Error> {
//    print_line("Select (a) to add a variable, (d) to delete a variable, or (q) to quit");
//    print("Enter your choice: ");
//    std::io::stdout().flush().unwrap();
//    let mut choice = String::new();
//    std::io::stdin().read_line(&mut choice).unwrap();
//    choice = choice.trim().to_string();
//
//    if choice == "a" {
//        let res = add_global_variable();
//        if res.is_err() {
//            print_line("Failed to add global variable: ");
//            println!("{}", res.err().unwrap());
//            return Ok(());
//        } else {
//            print_line("Successfully added global variable!");
//            return Ok(());
//        }
//    } else if choice == "d" {
//        let res = delete_global_variable();
//        if res.is_err() {
//            print("Failed to delete global variable: ");
//            println!("{}", res.err().unwrap());
//            return Ok(());
//        } else {
//            print_line("Successfully deleted global variable!");
//            return Ok(());
//        }
//    } else {
//        return Ok(());
//    }
//}

pub fn map_open_api_spec_to_http_requests(base_url: &str, open_api: OpenAPI) -> Vec<HttpRequest> {
    let mut requests = Vec::new();
    for (path, path_item) in open_api.paths {
        if path_item.get.is_some() {
            let mut url = format!("{}{}", base_url, path);
            let mut parameters = None;

            let operation = path_item.get.unwrap();
            if operation.parameters.is_some() {
                parameters = operation.parameters;
            }
            if parameters.is_some() {
                for parameter in parameters.unwrap() {
                    if parameter.required.is_some() && parameter.required.unwrap() {
                        url = url.replace(&format!("{{{}}}", parameter.name), "1");
                    } else {
                        url = url.replace(&format!("{{{}}}", parameter.name), "0");
                    }
                }
            }
            let request = HttpRequest {
                method: "GET".to_string(),
                url,
                headers: Vec::new(),
                body_type: None,
                body: None,
                extract_variables: None,
            };
            requests.push(request);
        }

        if path_item.post.is_some() {
            let mut url = format!("{}{}", base_url, path);
            let mut parameters = None;

            let operation = path_item.post.unwrap();
            if operation.parameters.is_some() {
                parameters = operation.parameters;
            }
            if parameters.is_some() {
                for parameter in parameters.unwrap() {
                    if parameter.required.is_some() && parameter.required.unwrap() {
                        url = url.replace(&format!("{{{}}}", parameter.name), "1");
                    } else {
                        url = url.replace(&format!("{{{}}}", parameter.name), "0");
                    }
                }
            }
            let request = HttpRequest {
                method: "POST".to_string(),
                url,
                headers: Vec::new(),
                body_type: None,
                body: None,
                extract_variables: None,
            };
            requests.push(request);
        }

        if path_item.put.is_some() {
            let mut url = format!("{}{}", base_url, path);
            let mut parameters = None;

            let operation = path_item.put.unwrap();
            if operation.parameters.is_some() {
                parameters = operation.parameters;
            }
            if parameters.is_some() {
                for parameter in parameters.unwrap() {
                    if parameter.required.is_some() && parameter.required.unwrap() {
                        url = url.replace(&format!("{{{}}}", parameter.name), "1");
                    } else {
                        url = url.replace(&format!("{{{}}}", parameter.name), "0");
                    }
                }
            }
            let request = HttpRequest {
                method: "PUT".to_string(),
                url,
                headers: Vec::new(),
                body_type: None,
                body: None,
                extract_variables: None,
            };
            requests.push(request);
        }

        if path_item.delete.is_some() {
            let mut url = format!("{}{}", base_url, path);
            let mut parameters = None;

            let operation = path_item.delete.unwrap();
            if operation.parameters.is_some() {
                parameters = operation.parameters;
            }
            if parameters.is_some() {
                for parameter in parameters.unwrap() {
                    if parameter.required.is_some() && parameter.required.unwrap() {
                        url = url.replace(&format!("{{{}}}", parameter.name), "1");
                    } else {
                        url = url.replace(&format!("{{{}}}", parameter.name), "0");
                    }
                }
            }
            let request = HttpRequest {
                method: "DELETE".to_string(),
                url,
                headers: Vec::new(),
                body_type: None,
                body: None,
                extract_variables: None,
            };
            requests.push(request);
        }
    }
    return requests;
}

pub fn read_http_request_file() -> Vec<HttpRequest> {
    if !File::open(get_http_requests_file_path()).is_ok() {
        if let Err(err) = fs::create_dir(format!("{}/.xhtp", get_home_path())) {
            eprintln!("Error creating directory: {}", err);
        }
        // File does not exist, create it
        let mut file = File::create(get_http_requests_file_path()).expect("Failed to create file");
        file.write_all("[]".as_bytes())
            .expect("Failed to create file");
    }
    let file = File::open(get_http_requests_file_path()).expect("Failed to open file");
    let reader = BufReader::new(file);
    let requests: Vec<HttpRequest> = serde_json::from_reader(reader).expect("Failed to parse JSON");
    requests
}

pub fn merge_requests(
    saved_requests: &Vec<HttpRequest>,
    imported_requests: &Vec<HttpRequest>,
) -> Vec<HttpRequest> {
    let mut merged_requests = saved_requests.clone();

    for imported_request in imported_requests {
        if !merged_requests.contains(imported_request) {
            merged_requests.push(HttpRequest {
                method: imported_request.method.clone(),
                url: imported_request.url.clone(),
                headers: imported_request.headers.clone(),
                body_type: imported_request.body_type.clone(),
                body: imported_request.body.clone(),
                extract_variables: None,
            });
        }
    }

    return merged_requests;
}

pub async fn handle_open_api_sepc_import(spec_url: &str) -> Result<(), reqwest::Error> {
    let base_url = spec_url.split("/").collect::<Vec<&str>>()[0..3].join("/");
    println!("{}", base_url);
    let spec_url = get_url_with_https(spec_url);
    let spec = reqwest::get(spec_url).await?.text().await?;
    let spec: OpenAPI = serde_json::from_str(&spec).unwrap();
    let imported_requests = map_open_api_spec_to_http_requests(&base_url, spec);
    let saved_requests = read_http_request_file();

    let merged_requests = merge_requests(&saved_requests, &imported_requests);

    print_line("Saving requests...");

    let json = serde_json::to_string_pretty(&merged_requests).unwrap();

    print_line("Created json");

    let mut file = File::create(get_http_requests_file_path()).unwrap();

    print_line("Created file");

    file.write_all(json.as_bytes()).unwrap();

    print_line("Wrote to file");

    return Ok(());
}
