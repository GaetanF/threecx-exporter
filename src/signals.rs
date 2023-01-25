use signal_hook::{iterator::Signals};
use signal_hook::consts::{SIGTERM, SIGQUIT, SIGINT, SIGHUP};

pub(crate) fn register_signal_handlers() -> Result<(), Box<dyn std::error::Error>>  {
    let mut signals = Signals::new(&[SIGINT, SIGTERM, SIGQUIT, SIGHUP])?;

    std::thread::spawn(move || {
        for sig in signals.forever() {
            match sig {
                SIGHUP => {
                    println!("Need reload");
                    unsafe { CONFIG += 1 };
                }
                SIGQUIT => {
                    println!("Good bye!");
                    std::process::exit(0);
                },
                SIGTERM => {
                    println!("Good bye!");
                    std::process::exit(0);
                },
                SIGTERM => {
                    println!("Good bye!");
                    std::process::exit(0);
                },
                _ => continue,
            }
        }
    });

    Ok(())
}