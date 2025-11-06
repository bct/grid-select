use argh::FromArgs;
use grid_select::{config, window};
use std::io::{self, BufRead};
use std::path::PathBuf;

/// Graphical picker like XMonad's GridSelect.
#[derive(FromArgs)]
struct Args {
    /// path to config file
    #[argh(option, short = 'c')]
    config_file: Option<PathBuf>,

    /// string delimiter. If provided, each line provided on stdin should be in the
    /// form "value<delimiter>display". "display" will be presented to the user;
    /// "value" will be written to stdout when an item is chosen.
    #[argh(option, short = 'd')]
    delimiter: Option<String>,
}

fn read_options(delimiter: Option<&str>) -> Vec<(String, Option<String>)> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    lines
        .map(|r| {
            let line = r.expect("failed to read line");
            match delimiter.and_then(|d| line.split_once(d)) {
                Some((value, display)) => (value.to_string(), Some(display.to_string())),
                None => (line, None),
            }
        })
        .collect()
}

fn main() {
    env_logger::init();

    let args: Args = argh::from_env();
    let config = config::Config::load(args.config_file).unwrap();

    let options = read_options(args.delimiter.as_deref()).to_vec();

    if options.is_empty() {
        println!("no options were provided on stdin, exiting.");
        return;
    }

    let (mut window, mut event_loop) = window::Window::new(config, &options).unwrap();

    // We don't draw immediately, the configure will notify us when to first draw.
    loop {
        event_loop.dispatch(None, &mut window).unwrap();

        if window.should_exit() {
            break;
        }
    }
}
