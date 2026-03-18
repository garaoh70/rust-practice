use chrono::{Local, NaiveDate};
use std::{f64::consts::PI, io};

fn get_biorhythm(date: i64) -> (f64, f64, f64) {
    // 三角関数の計算はメソッドチェーンのように書く
    let b = (2.0 * PI * ((date % 23) as f64 / 23.0)).sin();
    let e = (2.0 * PI * ((date % 28) as f64 / 28.0)).sin();
    let i = (2.0 * PI * ((date % 33) as f64 / 33.0)).sin();

    return (b, e, i);
}

fn biorhythm_to_string(biorithm: (f64, f64, f64), scale: usize) -> String {
    let mut chars = vec![' '; scale * 2 + 1];
    chars[0] = '-';
    chars[scale] = '|';
    chars[scale * 2] = '+';
    chars[((biorithm.0 + 1.0) * (scale as f64)).round() as usize] = 'P';
    chars[((biorithm.1 + 1.0) * (scale as f64)).round() as usize] = 'E';
    chars[((biorithm.2 + 1.0) * (scale as f64)).round() as usize] = 'I';

    return chars.into_iter().collect();
}

fn main() {
    // 標準出力
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力ミス");

    // 日付をパース
    let birthday =
        NaiveDate::parse_from_str(input.trim(), "%Y-%m-%d").expect("日付フォーマットミス");
    let now = Local::now().date_naive();

    // 生まれてからの日数
    let diff = (now - birthday).num_days();

    // 本日プラスマイナス５日でループ
    for i in -5i64..=5 {
        let line = biorhythm_to_string(get_biorhythm(diff + i), 60);
        println!("{:>3} : {}", i, line);
    }
}
