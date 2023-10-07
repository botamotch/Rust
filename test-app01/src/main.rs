use anyhow::{anyhow, Result};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::env;
use std::io::Read;

#[derive(Deserialize, Clone)]
pub struct Sheet {
    frames: HashMap<String, Cell>,
}

#[derive(Deserialize, Clone)]
struct Cell {
    frame: RectSheet,
}

impl Cell {
    fn copy(&self) -> Cell {
        Cell {
            frame: RectSheet {
                x: self.frame.x,
                y: self.frame.y,
                w: self.frame.w,
                h: self.frame.h,
            },
        }
    }
}

#[derive(Deserialize, Clone)]
struct RectSheet {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let framename: &str = &args[2];

    let cell: Cell = foo(filename, framename).expect(&format!(
        "file read error !, file : {}, frame : {}",
        filename, framename
    ));

    println!("> file read success : {}", filename);
    println!("> frame data '{}' is", framename);
    println!("> x : {:?}", cell.frame.x);
    println!("> y : {:?}", cell.frame.y);
    println!("> w : {:?}", cell.frame.w);
    println!("> h : {:?}", cell.frame.h);
}

fn foo(filename: &str, framename: &str) -> Result<Cell> {
    let mut file = std::fs::File::open(filename).map_err(|e| anyhow!(e))?;
    let mut s = String::new();
    file.read_to_string(&mut s).map_err(|e| anyhow!(e))?;
    let sheet: Sheet = serde_json::from_str(&s).map_err(|e| anyhow!(e))?;

    Ok(sheet
        .frames
        .get(framename)
        .ok_or_else(|| anyhow!("frame not found"))?
        .copy())
}
