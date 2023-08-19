extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use serde_json::Error;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
struct Url {
    raw: String,
    host: Vec<String>,
    path: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Header {
    key: String,
    value: String,
    disabled: Option<bool>,
    #[serde(rename = "type")]
    _type: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Auth {
    #[serde(rename = "type")]
    x: String,
    bearer: Option<Vec<Header>>,
}

#[derive(Serialize, Deserialize)]
struct Request {
    method: String,
    header: Vec<Header>,
    url: Url,
}

#[derive(Serialize, Deserialize)]
struct Api {
    request: Request,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Info {
    _postman_id: String,
    name: String,
    schema: String,
    _exporter_id: String,
}

#[derive(Serialize, Deserialize)]
struct Postman {
    info: Info,
    item: Vec<Api>,
    variable: Vec<Header>,
    auth: Option<Auth>,
}

fn main() -> Result<(), Error> {
    let mut file_content = String::new();
    let mut data_file = File::open("test.postman.json").unwrap();
    data_file.read_to_string(&mut file_content).unwrap();
    let r: Postman = serde_json::from_str(&file_content).expect("JSON was not well-formatted");

    let mut write_file = File::create("api.http").expect("create failed");
    let mut write_content = String::new();
    write_content.push_str(&format!("### {}\n", r.info.name));
    for variable in r.variable.iter() {
        if variable.disabled.is_none() {
            write_content.push_str(&format!("@{} = {}\n", variable.key, variable.value));
        }
    }
    write_content.push_str("\n\n");
    for x in r.item.iter() {
        write_content.push_str(&format!("### {}\n", x.name));
        write_content.push_str(&format!("{} {}\n", x.request.method, x.request.url.raw));
        for header in x.request.header.iter() {
            if header.disabled.is_none() {
                write_content.push_str(&format!("{}: {}\n", header.key, header.value));
            }
        }
        if r.auth.is_some() {
            let auth = r.auth.as_ref().unwrap();
            if auth.x == "bearer" {
                for header in auth.bearer.as_ref().unwrap().iter() {
                    if header.disabled.is_none() {
                        write_content
                            .push_str(&format!("{}: Bearer {}\n", "Authorization", header.value));
                    }
                }
            }
        }
        write_content.push('\n');
    }

    write_file
        .write_all(write_content.as_bytes())
        .expect("write failed");

    Ok(())
}
