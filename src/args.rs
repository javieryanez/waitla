use clap::{App, Arg};

pub fn define() -> App<'static> {
    let app = App::new("waitla")
        .version("0.1.0")
        .author("Javier Yáñez")
        .about("Wait for Load Average")
        .arg(
            Arg::new("treshold")
                .short('t')
                .long("load-average-treshold")
                .takes_value(true)
                .required(false)
                .about("Set load average treshold"),
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
                .about("Set sleep time in milliseconds"),
        );

    return app;
}
