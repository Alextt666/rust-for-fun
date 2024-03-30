use std::collections::HashMap;
use std::io::{Result,Write};

#[derive(Debug,PartialEq,Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code:&'a str,
    status_text:&'a str,
    headers:Options<HashMap<&'a str,&'a str>>,
    body:Option<String>
}