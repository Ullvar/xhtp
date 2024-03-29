use crate::utils::read_http_request_file;
use clap::Parser;
use serde_json::Value;
use std::fs::{self, File};
mod openapi_structs;
mod structs;
mod utils;
use std::io::Write;
use std::process::{Command, ExitStatus};

fn open_requests_file_in_editor(request_index: &Option<&String>) {
    if request_index.is_some() {
        let index = request_index.unwrap().parse::<usize>().unwrap();
        let requests = read_http_request_file();
        let request = utils::get_request_from_saved_requests(&requests, index);
        let temp_file_path = "xhtp_tmp.json";
        fs::write(
            temp_file_path,
            serde_json::to_string_pretty(request).unwrap(),
        )
        .expect("Failed to create temporary file.");

        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
        let _status: ExitStatus = Command::new(editor)
            .arg(temp_file_path)
            .status()
            .expect("Failed to open the editor.");

        let edited_content =
            fs::read_to_string(temp_file_path).expect("Failed to read the edited file.");

        fs::remove_file(temp_file_path).expect("Failed to remove the temporary file.");

        let mut requests = read_http_request_file();
        requests[index - 1] = serde_json::from_str(&edited_content).unwrap();

        let json = serde_json::to_string_pretty(&requests).unwrap();

        let mut file = File::create(utils::get_http_requests_file_path()).unwrap();

        file.write_all(json.as_bytes()).unwrap();
    } else {
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
        let path = utils::get_http_requests_file_path();
        let mut child = std::process::Command::new(editor)
            .arg(path)
            .spawn()
            .expect("Failed to open file in editor");
        let ecode = child.wait().expect("Failed to wait on child");
        assert!(ecode.success());
    }
}

fn print_http_response_as_json(http_response: &structs::HttpResponse) {
    let json_http_response = serde_json::to_string(&http_response).unwrap();
    println!("{}", json_http_response);
}

async fn handle_response(
    req: &structs::HttpRequest,
    res: reqwest::Response,
) -> Result<(), reqwest::Error> {
    let mut http_response = structs::HttpResponse {
        method: req.method.clone(),
        url: req.url.clone(),
        status_code: res.status().as_u16(),
        json_data: None,
        text_data: None,
    };

    let content_type = utils::get_content_type_from_header(res.headers());

    if content_type == "application/json" {
        let res_text = res.text().await?;
        let json: Value = serde_json::from_str(&res_text).unwrap();

        if req.extract_variables.is_some() {
            let extract_variables = req.extract_variables.as_ref().unwrap();
            for variable in extract_variables {
                if let Some(value) = utils::get_json_value(&json, &variable.key_path) {
                    utils::save_to_global_variables(
                        variable.variable_name.clone(),
                        value.to_string(),
                    );
                } else {
                    println!(
                        "The key '{}' was not found in the JSON.",
                        &variable.key_path
                    );
                }
            }
        }
        http_response.json_data = Some(json.clone());
        print_http_response_as_json(&http_response);
    } else if res.status().as_u16() == 204 {
        println!("204 No Content")
    } else {
        http_response.text_data = Some(res.text().await?);
        print_http_response_as_json(&http_response);
    }

    Ok(())
}

fn get_headers_from_vec(headers: &Vec<String>) -> reqwest::header::HeaderMap {
    let mut header_map = reqwest::header::HeaderMap::new();
    for header in headers {
        let header_split: Vec<&str> = header.split(":").collect();
        let header_name = header_split[0];
        let header_value = header_split[1].trim().replace("\"", "");
        header_map.insert(
            reqwest::header::HeaderName::from_bytes(header_name.as_bytes()).unwrap(),
            reqwest::header::HeaderValue::from_str(header_value.as_str()).unwrap(),
        );
    }
    header_map
}

async fn make_request(request: &structs::HttpRequest) -> Result<(), reqwest::Error> {
    let global_variables = utils::get_global_variables();
    let mut partial_url = request.url.clone();
    let mut headers = request.headers.clone();

    for global_variable in global_variables {
        let key = format!("{{{{{}}}}}", global_variable.key);
        let value = global_variable.value;
        partial_url = partial_url.replace(&key, &value);
        for header in &mut headers {
            *header = header.replace(&key, &value);
        }
    }

    let full_url = utils::get_url_with_https(&partial_url);

    if request.method == "GET" {
        let response = reqwest::Client::new()
            .get(&full_url)
            .headers(get_headers_from_vec(&headers))
            .send()
            .await?;
        handle_response(request, response).await?;
    } else if request.method == "POST" {
        if request.body_type.is_none() {
            let response = reqwest::Client::new()
                .post(&full_url)
                .headers(get_headers_from_vec(&headers))
                .send()
                .await?;
            handle_response(request, response).await?;
            return Ok(());
        } else if request.body_type.as_ref().unwrap() == "form" {
            let response = reqwest::Client::new()
                .post(&full_url)
                .headers(get_headers_from_vec(&headers))
                .form(&request.body)
                .send()
                .await?;
            handle_response(request, response).await?;
            return Ok(());
        } else if request.body_type.as_ref().unwrap() == "json" {
            let response = reqwest::Client::new()
                .post(&full_url)
                .headers(get_headers_from_vec(&headers))
                .json(&request.body)
                .send()
                .await?;
            handle_response(request, response).await?;
            return Ok(());
        } else if request.body_type.as_ref().unwrap() == "text" {
            let response = reqwest::Client::new()
                .post(&full_url)
                .headers(get_headers_from_vec(&headers))
                .body(request.body.as_ref().unwrap().to_string())
                .send()
                .await?;
            handle_response(request, response).await?;
            return Ok(());
        }
    } else if request.method == "PUT" {
        let response = reqwest::Client::new()
            .put(&full_url)
            .headers(get_headers_from_vec(&headers))
            .json(&request.body)
            .send()
            .await?;
        handle_response(request, response).await?;
    } else if request.method == "DELETE" {
        let response = reqwest::Client::new()
            .delete(&full_url)
            .headers(get_headers_from_vec(&headers))
            .send()
            .await?;
        handle_response(request, response).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut requests = read_http_request_file();

    let args = structs::Cli::parse();

    let help_text = Some("help".to_string());
    let help_text2 = Some("h".to_string());

    if args.first_arg.is_none()
        || args.first_arg == help_text
        || args.first_arg == help_text2
        || args.first_arg == Some("".to_string())
    {
        utils::print_line(
            "
        Usage: 
        Do a simple GET request by passing a url as an argument, alternatively you can select one of the following options:
            i <path to openapi spec> - import an openapi spec and save the requests in the config file 
            l - list all the urls in the config file
            l <request number> - list all the details of a specific request 
            e - open the requests config file in your editor
            d - delete a url from the config file
            gl - list all the global variables
            ga <variable name> <variable value> - add a global variable
            gd <variable name> - delete a global variable
            h - show help
        "
        );
        return Ok(());
    }

    let first_arg = args.first_arg.as_ref().unwrap();
    let second_arg = args.second_arg.as_ref();
    let third_arg = args.third_arg.as_ref();

    if first_arg == "l" {
        if second_arg.is_some() && utils::arg_is_number(&second_arg.as_ref().unwrap()) {
            let index = second_arg.as_ref().unwrap().parse::<usize>().unwrap();
            utils::print_full_saved_request_from_index(&requests, index)
        } else {
            utils::print_line("Pass the number of the request you want to use as an argument.");
            utils::print_saved_requests(&requests);
        }
        return Ok(());
    } else if first_arg == "a" {
        open_requests_file_in_editor(&None);
        return Ok(());
    } else if first_arg == "d" {
        let result = utils::handle_delete(&mut requests);
        if result.is_err() {
            return utils::too_big(&requests);
        }
        return Ok(());
    } else if first_arg == "e" {
        open_requests_file_in_editor(&second_arg);
        return Ok(());
    } else if first_arg == "gl" && second_arg.is_none() {
        utils::list_global_variables();
        return Ok(());
    } else if first_arg == "ga" && second_arg.is_some() && third_arg.is_some() {
        let name = second_arg.as_ref().unwrap().to_string();
        let value = third_arg.as_ref().unwrap().to_string();
        utils::save_to_global_variables(name, value);
        utils::list_global_variables();
        return Ok(());
    } else if first_arg == "gd" && second_arg.is_some() && third_arg.is_none() {
        let index_str = second_arg.as_ref().unwrap().to_string();
        utils::delete_global_variable(index_str);
        utils::list_global_variables();
        return Ok(());
    } else if first_arg == "i" {
        utils::handle_open_api_sepc_import(&second_arg.as_ref().unwrap())
            .await
            .unwrap();
        return Ok(());
    }

    if utils::arg_is_number(&first_arg) {
        let index = utils::convert_option_to_number(&first_arg);
        if index > requests.len() {
            return utils::too_big(&requests);
        }
        let request = utils::get_request_from_saved_requests(&requests, index);

        make_request(&request).await?;
    } else {
        let full_url = utils::get_url_with_https(&first_arg);
        let res = reqwest::get(full_url).await?;

        println!("{}", res.status());

        let content_type = utils::get_content_type_from_header(res.headers());

        if content_type == "application/json" {
            let res_text = res.text().await?;
            let json: Value = serde_json::from_str(&res_text).unwrap();

            println!("{:#}", json);
        } else {
            println!("{}", res.text().await?);
        }
    }

    Ok(())
}
