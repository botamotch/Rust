extern crate csv;
extern crate gnuplot;

// use gnuplot::{AxesCommon, Color, Figure, Fix};
use gnuplot::{Color, Figure};

use std::error::Error;
use std::io;
use std::process;

// レア度,カード,勇者,属性,覚醒,Lv,COST,SP,ATK,技倍率
type Record = (String, String, String, String, u8, u8, u8, f32, f32);

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
                    y_data.push(record.8 / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "古波蔵棗" => {
                    y_data.push(record.8 / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "三ノ輪銀" => {
                    y_data.push(record.8 / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "伊予島杏" => {
                    y_data.push(record.8 / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "郡千景" => {
                    y_data.push(record.8 / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "結城友奈" => {
                    y_data.push(record.8 / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "高嶋友奈" => {
                    y_data.push(record.8 / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "乃木若葉" => {
                    y_data.push(record.8 / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "犬吠埼樹" => {
                    y_data.push(record.8 / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "乃木園子" => {
                    y_data.push(record.8 / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "鷲尾須美" => {
                    y_data.push(record.8 / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "秋原雪花" => {
                    y_data.push(record.8 / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "東郷美森" => {
                    y_data.push(record.8 / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "白鳥歌野" => {
                    y_data.push(record.8 / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "犬吠埼風" => {
                    y_data.push(record.8 / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "土居珠子" => {
                    y_data.push(record.8 / 0.5);
                    x_ticks.push(format!("{} {} {}", record.0, record.1, record.2))
                }
                "乃木園子(中)" => {
                    y_data.push(record.8 / 0.5);
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
