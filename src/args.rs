use clap::{App, Arg};

const U64_MAX_STR: &str = "18446744073709551615";

pub fn define() -> App<'static> {
    let app = App::new("waitla")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Javier Yáñez")
        .about("Waits until the one minute load average reaches the indicated threshold.")
        .arg(
            Arg::new("treshold")
                .short('t')
                .long("load-average-treshold")
                .takes_value(true)
                .required(false)
                .about("Sets load average treshold"),
        )
        .arg(
            Arg::new("reverse")
                .short('r')
                .long("reverse")
                .takes_value(false)
                .required(false)
                .about("Waits the load average to be major than the treshold, rather than lower"),
        )
        .arg(
            Arg::new("sleep_millis")
                .short('s')
                .long("sleep-millis")
                .default_value("1000")
                .takes_value(true)
                .required(false)
                .about("Sets sleep time in milliseconds"),
        )
        .arg(
            Arg::new("max_time")
                .short('M')
                .long("max-time")
                .default_value(U64_MAX_STR)
                .takes_value(true)
                .required(false)
                .about("Sets maximum waiting time in seconds"),
        )
        .arg(
            Arg::new("min_time")
                .short('m')
                .long("min-time")
                .default_value("0")
                .takes_value(true)
                .required(false)
                .about("Sets minimum waiting time in seconds"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .takes_value(false)
                .required(false)
                .about("Makes the operation more talkative"),
        );

    return app;
}
