extern mod extra;
extern mod http;

use std::os;
use std::io::Writer;
use std::io::fs::File;
use std::io;
//use extra::time;

use http::server::{Request, ResponseWriter};
use http::headers::content_type::MediaType;

use super::utils::{get_url,not_found};


pub struct StaticController;

impl StaticController {
    pub fn Get (request: &Request, response: &mut ResponseWriter) {
        let working_dir = os::getcwd();
        let url = get_url(request);
        let mut file_path: PosixPath = working_dir.join(url.slice_from(1));

        if file_path.exists() {
            if !file_path.is_file() {
                // try index.html, default.html in a directory
                file_path = file_path.join("index.html");
                if !file_path.exists() || !file_path.is_file() {
                    file_path = file_path.with_filename("default.html");
                    if !file_path.exists() || !file_path.is_file() {
                        not_found(request, response);
                        return;
                    }
                }
            }

            let f = io::result(|| File::open(&file_path));
            match f {
                Ok(mut reader) => {
                    response.headers.content_type = match file_path.extension_str() {
                        Some("css") => {
                            Some(MediaType {
                                type_: ~"text",
                                subtype: ~"css",
                                parameters: ~[]
                            })
                        }
                        Some("js") => {
                            Some(MediaType {
                                type_: ~"text",
                                subtype: ~"javascript",
                                parameters: ~[]
                            })
                        }
                        _ => None
                    };

                    //let reader = f.get_mut_ref();
                    let file_contents = reader.read_to_end();

                    response.headers.content_length = Some(file_contents.len());

                    response.write(file_contents);
                },
                _ => {
                    not_found(request, response);
                }
            }
        }
        else {
            not_found(request, response);
        }
    }
}

