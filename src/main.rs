use std::{str::FromStr, io::{Read, Write}, iter::zip};
use encoding_rs::*;

fn main() {
    if std::env::args().len() != 2 {
        eprintln!("{}", usage());
        std::process::exit(1);
    }
    let path = std::env::args().nth(1).unwrap();
    let mut f = std::fs::File::open(&path).unwrap();
    let meta = std::fs::metadata(&path).unwrap();
    let mut bytes = vec![0u8; meta.len() as usize];
    f.read(&mut bytes).unwrap();
    let (s, _encoding_used, had_errors) = SHIFT_JIS.decode(&bytes);
    if had_errors {
        eprintln!("Failed to decode SJIS.");
        std::process::exit(1);
    }
    let mut file_names: Vec<String> = Vec::new();
    let mut rezepts: Vec<Vec<&str>> = Vec::new();
    let mut rows: Vec<&str> = Vec::new();
    for line in s.split("\r\n") {
        if line.starts_with("RE") {
            let values: Vec<&str> = line.split(",").into_iter().collect();
            let ym = values[3];
            let name = String::from_str(values[4]).unwrap();
            let name = name.replace(&[' ', '　'], "");
            let patient_id = values[13];
            let file = format!("henrei-{}-{}-{}.csv", ym, name, patient_id);
            file_names.push(file);
            if rows.len() > 0 {
                let rs = rows.clone();
                rezepts.push(rs);
            }
            rows = vec![line];
        } else if line.starts_with("HG") {
            if rows.len() > 0 {
                let rs = rows.clone();
                rezepts.push(rs);
            }
        } else if line.starts_with("HI" ){
            continue;
        } else {
            rows.push(line);
        }
    }
    for (f, rs) in zip(file_names, rezepts) {
        let mut out = std::fs::File::create(&f).unwrap();
        for r in rs {
            let line = format!("{}\r\n", r);
            out.write(line.as_bytes()).unwrap();
        }
    }
}

fn usage() -> String {
    String::from_str("Usage: henrei-split HENREI-FILE").unwrap()
}
