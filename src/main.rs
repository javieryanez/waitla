use std::env;
use std::process;
use std::thread;
use std::time::{Duration, SystemTime};
use systemstat::{Platform, System};

use clap::ArgMatches;
use num_cpus;

mod args;

fn main() {
    let mut treshold: f32 = 0.0;
    let mut reverse: bool = false;
    let mut sleep_millis: u64 = 1000;
    let mut max_time: u64 = u64::MAX;

    setup(&mut treshold, &mut reverse, &mut sleep_millis, &mut max_time);

    wait(&mut treshold, &mut reverse, &mut sleep_millis, &mut max_time);
}

fn wait(treshold: &mut f32, reverse: &mut bool, sleep_millis: &mut u64, max_time: &mut u64) {
    if !*reverse {
        println!("Waiting load average less than {}", treshold);
    } else {
        println!("Waiting load average greater than {}", treshold);
    }
    let sys = System::new();
    let mut la = get_load_average(&sys);
    let begin_time = SystemTime::now();
    while (la > *treshold && !*reverse) || (la < *treshold && *reverse) &&
    (begin_time.elapsed().unwrap().as_secs() < *max_time) {
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

fn setup(treshold: &mut f32, reverse: &mut bool, sleep_millis: &mut u64, max_time: &mut u64) {
    // Prints each argument on a separate line
    for argument in env::args() {
        println!("{}", argument);
    }

    let matches = args::define().get_matches();
    set_treshold(&matches, treshold);

    set_reverse(&matches, reverse);

    set_sleep_time(&matches, sleep_millis);

    set_max_time(&matches, max_time);
}

fn set_max_time(matches: &ArgMatches, max_time: &mut u64) {
    let max_time_str = matches.value_of("max_time").unwrap();

    match max_time_str.parse::<u64>() {
        Ok(m) => *max_time = m,
        Err(_) => {
            println!("That's not a valid maximum time! {}", max_time_str);
            process::exit(0x001);
        }
    }
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
    if matches.is_present("treshold") {
        let treshold_str = matches.value_of("treshold").unwrap();

        match treshold_str.parse::<f32>() {
            Ok(t) => *treshold = t,
            Err(_) => {
                println!("That's not a valid load average! {}", treshold_str);
                process::exit(0x0100);
            }
        }
    } else {
        *treshold = num_cpus::get() as f32;
    }
}
