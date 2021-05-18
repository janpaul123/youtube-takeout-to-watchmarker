use std::fs;
use regex::Regex;
use std::fmt::Debug;
use serde::Serialize;
use base64::write::EncoderWriter;
use std::io::{BufWriter, Write};

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
struct Link<'a> {
    strIdent: &'a str,
    intTimestamp: &'static f32,
    strTitle: &'a str,
    intCount: &'static i8,
}

fn main() {
    let contents = fs::read_to_string("watch-history.html").unwrap();

    let re = Regex::new(r#"v=([A-Za-z0-9_\-]+)">([^<]*)</a>"#).unwrap();

    let links: Vec<Link> = re
        .captures_iter(&contents)
        .map(|c| Link {
            strIdent: c.get(1).unwrap().as_str(),
            intTimestamp: &1613792276507.53,
            strTitle: c.get(2).unwrap().as_str(),
            intCount: &1,
        })
        .collect();

    let mut buf_writer = BufWriter::new(fs::File::create("watchmarker-youtube.database").unwrap());
    serde_json::to_writer(EncoderWriter::new(&mut buf_writer, base64::STANDARD), &links).unwrap();
    buf_writer.flush().unwrap();
}
