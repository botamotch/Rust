extern crate csv;
extern crate gnuplot;

// use gnuplot::{AxesCommon, Color, Figure, Fix};
use gnuplot::{Color, Figure};

use std::error::Error;
use std::io;
use std::process;

// レア度,カード,勇者,属性,覚醒,Lv,COST,SP,ATK,技倍率
type Record = (String, String, String, String, u8, u8, u8, u32, u32);

fn read_csv_data(f: String) -> Vec<Record> {
    let mut data_in: Vec<Record> = vec![];
    let mut rdr = csv::Reader::from_path("data.csv").unwrap();
    {
        for result in rdr.deserialize() {
            let record: Record = result.unwrap();
            data_in.push((record.0, record.1, record.2, record.3, record.4, record.5, record.6, record.7, record.8))
        }
    }
    data_in
}

fn get_atk(data: Vec<Record>) -> Vec<u32> {
    let mut val: Vec<u32> = vec![];
    for record in data {
        val.push(record.7);
    }
    val
}

fn get_card(data: Vec<Record>) -> Vec<String> {
    let mut val: Vec<String> = vec![];
    for record in data {
        val.push(format!("{} {} {}", record.0, record.1, record.2));
    }
    val
}

fn get_dps(data: Vec<Record>) -> Vec<f64> {
    let mut val: Vec<f64> = vec![];
    for record in data {
        match &record.2[..] { // &*record.2 / &record.2[..] / record.2.as_str()
            "三好夏凛"     => val.push((record.8 as f64) / 0.5), // 200%
            "古波蔵棗"     => val.push((record.8 as f64) / 0.5),
            "伊予島杏"     => val.push((record.8 as f64) / 0.6), // 166%
            "山伏しずく"   => val.push((record.8 as f64) / 0.6),
            "郡千景"       => val.push((record.8 as f64) / 0.7), // 142%
            "楠芽吹"       => val.push((record.8 as f64) / 0.7),
            "三ノ輪銀"     => val.push((record.8 as f64) / 0.8), // 125%
            "乃木若葉"     => val.push((record.8 as f64) / 0.8),
            "弥勒夕海子"   => val.push((record.8 as f64) / 0.8),
            "犬吠埼樹"     => val.push((record.8 as f64) / 0.9), // 111%
            "結城友奈"     => val.push((record.8 as f64) / 1.0), // 100%
            "高嶋友奈"     => val.push((record.8 as f64) / 1.0),
            "乃木園子"     => val.push((record.8 as f64) / 1.0),
            "乃木園子(中)" => val.push((record.8 as f64) / 1.0),
            "東郷美森(後)" => val.push((record.8 as f64) / 1.3), //  76%
            "鷲尾須美"     => val.push((record.8 as f64) / 1.5), //  66%
            "秋原雪花"     => val.push((record.8 as f64) / 1.7), //  58%
            "東郷美森"     => val.push((record.8 as f64) / 2.0), //  50%
            "白鳥歌野"     => val.push((record.8 as f64) / 2.0),
            "犬吠埼風"     => val.push((record.8 as f64) / 2.4), //  41%
            "土居珠子"     => val.push((record.8 as f64) / 3.0), //  33%
            "加賀城雀"     => val.push((record.8 as f64) / 3.0),
            _ => println!("> Undifined Character : {} {} {}", record.1, record.1, record.2),
        }
    }
    val
}

fn read_csv() -> Result<(), Box<Error>> {
    let mut x_data = vec![];
    let mut y_data = vec![];
    let mut x_ticks = vec![];

    let mut fg = Figure::new();

    let mut rdr = csv::Reader::from_reader(io::stdin());
    {
        for result in rdr.deserialize() {
            let record: Record = result?;
            match &*record.2 {
                "三好夏凛" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "古波蔵棗" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "三ノ輪銀" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "伊予島杏" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "郡千景" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "結城友奈" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "高嶋友奈" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "乃木若葉" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "犬吠埼樹" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "乃木園子" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "鷲尾須美" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "秋原雪花" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "東郷美森" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "白鳥歌野" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "犬吠埼風" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "土居珠子" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "乃木園子(中)" => {
                    y_data.push((record.8 as f64) / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                _ => println!("Undifined Character : {}", record.2),
            }
        }
        let len = y_data.len();
        for i in 0..len {
            x_data.push(i + 1);
        }
        /*
         * "x_ticks"に文字列を追加するところまでは完了（たぶん）
         * プロットのX軸に設定するところができない
         */
        let axes = fg.axes2d();
        axes.boxes(x_data.iter(), y_data.iter(), &[Color("blue")]);
    }
    fg.show();
    Ok(())
}

fn main() {
    if let Err(err) = read_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
