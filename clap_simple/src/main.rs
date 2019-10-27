use clap::{App, Arg};
use cmd_lib::run_cmd;

fn main() {
    let matches = App::new("MyApp")
        .version("1.0")
        .author("Mark V")
        .arg(
            Arg::with_name("first")
                .short("a")
                .index(1)
                .required(true)
                .value_name("a")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("second")
                .short("b")
                .index(2)
                .required(true)
                .value_name("b")
                .takes_value(true)
        )
        .get_matches();

    let a = matches.value_of("a").unwrap()
        .parse::<u32>().unwrap();
    let b = matches.value_of("b").unwrap()
        .parse::<u32>().unwrap();

    run_cmd!("ps aux");

    println!("Hello, world! {}", a * a + b * b);
}
