use anyhow::Result;
use robot_panic::setup_panic;
use rover::*;
use roverup::Installer;
use sputnik::Session;
use structopt::StructOpt;

use std::env;
use std::path::PathBuf;
use std::thread;

fn main() -> Result<()> {
    setup_panic!();

    maybe_install()?;

    let app = cli::Rover::from_args();
    timber::init(app.log_level);
    tracing::trace!(command_structure = ?app);

    // attempt to create a new `Session` to capture anonymous usage data
    let result = match Session::new(&app) {
        // if successful, report the usage data in the background
        Ok(session) => {
            // kicks off the reporting on a background thread
            let report_thread = thread::spawn(move || {
                // log + ignore errors because it is not in the critical path
                let _ = session.report().map_err(|telemetry_error| {
                    tracing::debug!(?telemetry_error);
                    telemetry_error
                });
            });

            // kicks off the app on the main thread
            // don't return an error with ? quite yet
            // since we still want to report the usage data
            let app_result = app.run();

            // makes sure the reporting finishes in the background
            // before continuing.
            // ignore errors because it is not in the critical path
            let _ = report_thread.join();

            // return result of app execution
            // now that we have reported our usage data
            app_result
        }

        // otherwise just run the app without reporting
        Err(_) => app.run(),
    }?;

    result.print();
    Ok(())
}

fn maybe_install() -> Result<()> {
    if let Ok(executable_location) = env::current_exe() {
        if executable_location
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("could not parse executable name")
            .starts_with("roverup")
        {
            let force_install = env::args().any(|arg| arg == "-f" || arg == "--force");
            let override_install_path =
                env::var_os("APOLLO_HOME").map(|location| PathBuf::from(&location));
            let _ = Installer {
                force_install,
                override_install_path,
                executable_location,
            }
            .install()?;
        }
    }
    Ok(())
}
