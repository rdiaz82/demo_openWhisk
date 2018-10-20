extern crate openssl_probe;
extern crate rustc_serialize;

use rustc_serialize::json::Json;
use std::env;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Output {
    pub message: String,
}

pub fn set_env() {
    openssl_probe::init_ssl_cert_env_vars();
    if let Some(arg1) = env::args().nth(1) {
        let params = Json::from_str(&arg1).unwrap();
        if let Some(params_obj) = params.as_object() {
            if let Some(aws_key) = params_obj.get("AWS_KEY") {
                env::set_var(
                    "AWS_ACCESS_KEY_ID",
                    aws_key.as_string().unwrap().to_string(),
                );
            }
            if let Some(aws_secret) = params_obj.get("AWS_SECRET") {
                env::set_var(
                    "AWS_SECRET_ACCESS_KEY",
                    aws_secret.as_string().unwrap().to_string(),
                );
            }
        }
    };
}
