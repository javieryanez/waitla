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

    setup(&mut treshold, &mut reverse);

    wait(&mut treshold, &mut reverse);
}

fn wait(treshold: &mut f32, reverse: &mut bool) {
    if !*reverse {
        println!("Waiting load average less than {}", treshold);
    } else {
        println!("Waiting load average greater than {}", treshold);
    }
    
    let sys = System::new();
    let mut la = get_load_average(&sys);
    while (la > *treshold && !*reverse) || (la < *treshold && *reverse){
        thread::sleep(Duration::from_millis(1000));
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

fn setup(treshold: &mut f32, reverse: &mut bool) {
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
        .get_matches();

    set_treshold(&matches, treshold);

    set_reverse(&matches, reverse);
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
