// SPDX-License-Identifier: GPL-3.0-or-latere
// Copyright (C) 2024 Tobias Hunger <tobias.hunger@gmail.com>
//
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn setup_tracing() -> crate::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("MULTI_LS_LOG"))
        .init();

    Ok(())
}
