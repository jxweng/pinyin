extern crate clap;
use clap::{App, AppSettings, Arg};
use pinyin::ToPinyin;
fn main() {
    let matches = App::new("pinyin")
        .version("1.0")
        .author("jxw3ng")
        .about("汉字转抖音")
        .arg(
            Arg::with_name("source")
                .short("s")
                .index(1)
                .takes_value(true)
                .help("输入汉字"),
        )
        .arg(Arg::with_name("tone").short("t").help("显示声调"))
        .setting(AppSettings::ArgRequiredElseHelp) //没有参数时显示帮助
        .get_matches();

    if let Some(s) = matches.value_of("source") {
        if matches.is_present("tone") {
            for pinyin in s.to_pinyin() {
                if let Some(pinyin) = pinyin {
                    print!("{} ", pinyin.with_tone());
                }
            }
        } else {
            for pinyin in s.to_pinyin() {
                if let Some(pinyin) = pinyin {
                    print!("{} ", pinyin.plain());
                }
            }
        }
    }
    println!();
}
