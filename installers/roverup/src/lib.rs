//! Self-installation of `rover`
//!
//! This module contains one public function which will self-install the
//! currently running executable as `rover`. Our goal is to either overwrite
//! the existing Rover installation in `PATH`, or to add a new directory
//! for Rover to live in and add it to `PATH`.
//!
//! This installer is run directly (probably by clicking on it) on Windows,
//! meaning it will pop up a console (as we're a console app). Output goes to
//! the console and users interact with it through the console. On Unix this is
//! intended to be run from a shell script (docs/installer/init.sh) which is
//! downloaded via curl/sh, and then the shell script downloads this executable
//! and runs it.
//!
//! This may get more complicated over time (self upates anyone?) but for now
//! it's pretty simple! We're largely just moving over our currently running
//! executable to a different path.

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use atty::{self, Stream};

mod error;
use error::RoverupError;

pub struct Installer {
    pub force_install: bool,
    pub executable_location: PathBuf,
    pub override_install_path: Option<PathBuf>,
}

impl Installer {
    pub fn install(&self) -> Result<PathBuf, RoverupError> {
        let install_path = self.do_install()?;

        // On Windows we likely popped up a console for the installation. If we were
        // to exit here immediately then the user wouldn't see any error that
        // happened above or any successful message. Let's wait for them to say
        // they've read everything and then continue.
        if cfg!(windows) {
            tracing::info!("Press enter to close this window...");
            let mut line = String::new();
            drop(io::stdin().read_line(&mut line));
        }

        Ok(install_path)
    }

    fn do_install(&self) -> Result<PathBuf, RoverupError> {
        // Find `rover` in PATH, we'll be using its installation directory as
        // our installation directory.
        let installation_dir = self.get_installation_dir()?;

        let destination = installation_dir
            .join("rover")
            .with_extension(env::consts::EXE_EXTENSION);

        if !self.force_install && destination.exists() {
            if !should_overwrite(&destination)? {
                return Err(RoverupError::AbortInstall);
            }
        }

        // Our relatively simple install step!
        fs::copy(&self.executable_location, &destination)?;
        tracing::info!(
            "Successfully installed rover to `{}`",
            destination.display()
        );

        Ok(destination)
    }

    fn get_installation_dir(&self) -> Result<PathBuf, RoverupError> {
        if let Some(install_path) = self.override_install_path.clone() {
            fs::create_dir_all(&install_path)?;
            Ok(install_path)
        } else {
            // TODO: make thing to add to path and such
            Err(RoverupError::AbortInstall)
        }
    }
}

fn should_overwrite(destination: &Path) -> Result<bool, RoverupError> {
    // If we're not attached to a TTY then we can't get user input, so there's
    // nothing to do except inform the user about the `-f` flag.
    if !atty::is(Stream::Stdin) {
        return Err(io::Error::from(io::ErrorKind::AlreadyExists))?;
    }

    // It looks like we're at an interactive prompt, so ask the user if they'd
    // like to overwrite the previous installation.
    tracing::info!(
        "Existing rover installation found at `{}`",
        destination.display()
    );
    tracing::info!("Would you like to overwrite this file? [y/N]: ");
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    if line.to_lowercase().starts_with('y') {
        Ok(true)
    } else {
        Ok(false)
    }
}
