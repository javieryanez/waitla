# <span style="color:grey">WAIT</span><span style="color:blue">LA</span>
Wait until the one minute load average reaches the indicated treshold.

## Usage
`waitla` can run without arguments:
```sh
waitla
```
When running without arguments, the number of CPUs in the system is used as the threshold for the average load. `waitla` will wait until load average is less than the number of CPUs.

To indicate a **threshold**, the option -t (--treshold) is specified:
```sh
waitla -t 6
```

Use the **reverse** option to wait the load average to be higher than the treshold, rather than lower. the option is -r (--reverse):
```sh
waitla -r
```

Options -m (--minimum) and -M (--maximum) allows indicate a **minimum** and **maximum** waiting time:
```sh
waitla -m 5 -M 60
```
By default the minimum time is 0 and the maximum time is infinite.

With the option -s (--sleep-millis) you can specify how often is checked the load average, in milliseconds:
```sh
waitla -s 200
```
By default the **sleep time** is 1000ms.

To run in **verbose** mode use the option -v (--verbose):
```sh
waitla -v
```

## Install

### Linux
```sh
sudo wget https://github.com/javieryanez/waitla/releases/download/v1.0.0/waitla-1.0.0-linux-x86-64 -O /usr/local/bin/waitla
sudo chmod +x /usr/local/bin/waitla
```

## Building
You could clone the project and build from source. You will need rust to do so.
```sh
git clone https://github.com/javieryanez/waitla.git
cd waitla
cargo build --release
```
You can choose the binary in *target/release/* directory.