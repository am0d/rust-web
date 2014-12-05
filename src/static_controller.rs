use std::os;
use std::io::Writer;
use std::io::fs::File;
use std::io::fs::PathExtensions;
//use extra::time;

use http::server::{Request, ResponseWriter};
use http::headers::content_type::MediaType;

use super::utils::{get_url,not_found};

use views::{Action};

pub struct StaticFile;

impl Action for StaticFile  {
    fn render(&self, _out: &mut Writer) {
    }
}

pub struct StaticController;

impl StaticController {
    pub fn get (request: &Request, response: &mut ResponseWriter) -> Box<Action + 'static> {
        let url = get_url(request);
        let mut file_path = if let Ok(working_dir) = os::getcwd() {
            working_dir.join(url.as_slice().slice_from(1))
        } else {
            panic!("`os::getcwd()` did not work");
        };

        if file_path.exists() {
            if !file_path.is_file() {
                // try index.html, default.html in a directory
                file_path = file_path.join("index.html");
                if !file_path.exists() || !file_path.is_file() {
                    file_path = file_path.with_filename("default.html");
                    if !file_path.exists() || !file_path.is_file() {
                        not_found(request, response);
                        return box StaticFile as Box<Action>;
                    }
                }
            }

            let f = File::open(&file_path);
            match f {
                Ok(mut reader) => {
                    response.headers.content_type = match file_path.extension_str() {
                        Some("css") => {
                            Some(MediaType {
                                type_: String::from_str("text"),
                                subtype: String::from_str("css"),
                                parameters: vec!()
                            })
                        }
                        Some("js") => {
                            Some(MediaType {
                                type_: String::from_str("text"),
                                subtype: String::from_str("javascript"),
                                parameters: vec!()
                            })
                        }
                        _ => None
                    };

                    //let reader = f.get_mut_ref();
                    let file_contents = match reader.read_to_end() {
                        Ok(contents) => contents,
                        Err(msg) => panic!(msg)
                    };

                    response.headers.content_length = Some(file_contents.len());

                    drop(response.write(file_contents.as_slice()));
                },
                _ => {
                    not_found(request, response);
                }
            }
        }
        else {
            not_found(request, response);
        }

        return box StaticFile as Box<Action>
    }
}

