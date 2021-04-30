use std::env;
use std::process;
use std::thread;
use std::time::Duration;
use systemstat::{Platform, System};

use clap::ArgMatches;
use clap::{App, Arg};
use num_cpus;

fn main() {
    let mut treshold: f32 = 0.0;
    let mut reverse: bool = false;
    let mut sleep_millis: u64 = 1000;

    setup(&mut treshold, &mut reverse, &mut sleep_millis);

    wait(&mut treshold, &mut reverse, &mut sleep_millis);
}

fn wait(treshold: &mut f32, reverse: &mut bool, sleep_millis: &mut u64) {
    if !*reverse {
        println!("Waiting load average less than {}", treshold);
    } else {
        println!("Waiting load average greater than {}", treshold);
    }
    
    let sys = System::new();
    let mut la = get_load_average(&sys);
    while (la > *treshold && !*reverse) || (la < *treshold && *reverse){
        thread::sleep(Duration::from_millis(*sleep_millis));
        la = get_load_average(&sys);
    }
}

fn get_load_average(sys: &System) -> f32 {
    let mut result: f32 = 0.0;

    match sys.load_average() {
        Ok(loadavg) => result = loadavg.one,
        Err(x) => println!("\nLoad average: error: {}", x),
    }

    return result;
}

fn setup(treshold: &mut f32, reverse: &mut bool, sleep_millis: &mut u64) {
    // Prints each argument on a separate line
    for argument in env::args() {
        println!("{}", argument);
    }

    let matches = App::new("waitla")
        .version("0.1.0")
        .author("Javier Yáñez")
        .about("Wait for Load Average")
        .arg(
            Arg::new("treshold")
                .short('t')
                .long("load-average-treshold")
                .default_value(&*num_cpus::get().to_string())
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
                .takes_value(true)
                .required(false)
                .about("Set sleep time in milliseconds"),
        )
        .get_matches();

    set_treshold(&matches, treshold);

    set_reverse(&matches, reverse);

    set_sleep_time(&matches, sleep_millis);
}

fn set_sleep_time(matches: &ArgMatches, sleep_millis: &mut u64) {
    let sleep_time_str = matches.value_of("sleep_millis").unwrap();

    match sleep_time_str.parse::<u64>() {
        Ok(s) => *sleep_millis = s,
        Err(_) => {
            println!("That's not a valid sleep time! {}", sleep_time_str);
            process::exit(0x001);
        }
    }
}

fn set_reverse(matches: &ArgMatches, reverse: &mut bool) {
    if matches.is_present("reverse") {
        *reverse = true;
    }
}

fn set_treshold(matches: &ArgMatches, treshold: &mut f32) {
    let treshold_str = matches.value_of("treshold").unwrap();

    match treshold_str.parse::<f32>() {
        Ok(t) => *treshold = t,
        Err(_) => {
            println!("That's not a valid load average! {}", treshold_str);
            process::exit(0x0100);
        }
    }
}
