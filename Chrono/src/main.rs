use chrono::{ DateTime, Duration, Local, NaiveDateTime, ParseError, Utc};

fn parse_date_time_from_str(date_str: &str, fmt: &str) -> Result<DateTime<Utc>, ParseError> {
    let naive_date_time = NaiveDateTime::parse_from_str(date_str, fmt)?;
    Ok(DateTime::from_utc(naive_date_time, Utc))
}





fn main() {
    let now_utc: DateTime<Utc> = Utc::now();
    let now_local: DateTime<Local> = Local::now();

    println!("当前 UTC 时间： {}", now_utc);
    println!("当前 本地 时间： {}", now_local);


    let date_time_result = parse_date_time_from_str("2020-09-05 23:10:20", "%Y-%m-%d %H:%M:%S");
    match date_time_result {
        Ok(date_time) => println!("解析出的时间为：{}", date_time),
        Err(e) => eprint!("解析错误：{}", e),
    }

    let now: DateTime<Utc> = Utc::now();
    let after_ten_days = now.checked_add_signed(Duration::days(10)).unwrap();
    let before_ten_days = now.checked_sub_signed(Duration::days(10)).unwrap();

    println!("十天后的时间：{}", after_ten_days);
    println!("十天前的时间：{}", before_ten_days);

    
    let earlier = now - chrono::Duration::seconds(5);

    println!("现在是：{}", now);
    println!("五秒前是：{}", earlier);

    // Chrono 支持使用 >、< 和 == 操作符来比较时间：
    assert!(now > earlier, "现在的时间应该比五秒前晚");

    // 格式化时间日期
    println!("现在的时间是：{}", now.format("%Y年%m月%d日 %H:%M:%S"));

    // Chrono 支持时区转换。你可以使用 with_timezone 方法将 UTC 时间转换为任何指定的时区时间：
    let utc_now: DateTime<Utc> = Utc::now();
    let shanghai = chrono::FixedOffset::east(8 * 3600);
    let shanghai_now = utc_now.with_timezone(&shanghai);

    println!("上海时间：{}", shanghai_now);
}
