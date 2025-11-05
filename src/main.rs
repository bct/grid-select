use argh::FromArgs;
use grid_select::{config, window};
use std::path::PathBuf;

/// Graphical picker like XMonad's GridSelect
#[derive(FromArgs)]
struct Args {
    /// path to config file
    #[argh(option, short = 'c')]
    config_file: Option<PathBuf>,
}

fn read_options() -> Vec<String> {
    std::io::stdin()
        .lines()
        .collect::<Result<_, _>>()
        .expect("failed to read stdin")
}

fn main() {
    env_logger::init();

    let args: Args = argh::from_env();
    let config = config::Config::load(args.config_file).unwrap();

    let options = read_options().to_vec();

    if options.is_empty() {
        println!("no options were provided on stdin, exiting.");
        return
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
