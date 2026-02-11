use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn expressionfile(expression: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let file = File::open(expression).expect("file not found");
    let fileread = BufReader::new(file);
    let mut filevec: Vec<i32> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("file not present");
        let linevec = line.split("\t").collect::<Vec<_>>()[0];
        filevec.push(linevec.parse::<i32>().unwrap());
    }
    Ok(filevec)
}
