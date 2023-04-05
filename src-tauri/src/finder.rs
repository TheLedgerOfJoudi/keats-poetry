use std::{fs::{File}, io::{self, Read}};
use json;

pub fn read_poems() -> io::Result<String>{
    let mut f = File::open("db/poems.json")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn find_poem(title: &str) -> io::Result<String>{
    let poems = read_poems()?;
    let parsed = json::parse(&poems).expect("Should have been able to read json");
    for i in 0..parsed.len(){
        if parsed[i]["title"] == title{
            return Ok(parsed[i]["lines"].to_string());
        }
    }

    Ok("Poem not found...".to_string())
}