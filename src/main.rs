// SPDX-License-Identifier: GPL-3.0-or-latere
// Copyright (C) 2024 Tobias Hunger <tobias.hunger@gmail.com>

pub mod trace;

pub use eyre::Result;
pub use tracing::{debug, error, instrument, trace, warn};

#[instrument(name = "main")]
fn main_loop() -> Result<()> {
    trace!("Trace: Main loop started");
    debug!("Debug: Main loop started");
    warn!("Warn: Main loop started");
    error!("Error: Main loop started");

    Ok(())
}

fn main() -> Result<()> {
    trace::setup_tracing()?;

    main_loop()
}
