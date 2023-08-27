use crate::utils::read_http_request_file;
use clap::Parser;
use serde_json::Value;
mod openapi_structs;
mod structs;
mod utils;

fn open_requests_file_in_editor() {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let path = utils::get_http_requests_file_path();
    let mut child = std::process::Command::new(editor)
        .arg(path)
        .spawn()
        .expect("Failed to open file in editor");
    let ecode = child.wait().expect("Failed to wait on child");
    assert!(ecode.success());
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
            import/i <path to openapi spec> - import an openapi spec and save the requests in the config file 
            list/l - list all the urls in the config file
            list/l <request number> - list all the details of a specific request 
            edit/e - open the requests config file in your editor
            delete/d - delete a url from the config file
            global/g - manage all the global variables
            help/h - show help
        "
        );
        return Ok(());
    }

    let first_arg = args.first_arg.as_ref().unwrap();
    let second_arg = args.second_arg.as_ref();

    if first_arg == "list" || first_arg == "l" {
        if second_arg.is_some() && utils::arg_is_number(&second_arg.as_ref().unwrap()) {
            let index = second_arg.as_ref().unwrap().parse::<usize>().unwrap();
            utils::print_full_saved_request_from_index(&requests, index)
        } else {
            utils::print_line("Pass the number of the request you want to use as an argument.");
            utils::print_saved_requests(&requests);
        }
        return Ok(());
    } else if first_arg == "add" || first_arg == "a" {
        //utils::handle_add(&mut requests);
        open_requests_file_in_editor();
        return Ok(());
    } else if first_arg == "delete" || first_arg == "d" {
        let result = utils::handle_delete(&mut requests);
        if result.is_err() {
            return utils::too_big(&requests);
        }
        return Ok(());
    } else if first_arg == "edit" || first_arg == "e" {
        open_requests_file_in_editor();
        return Ok(());
    } else if first_arg == "global" || first_arg == "g" {
        return utils::handle_global_variables();
    } else if first_arg == "import" || first_arg == "i" {
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
