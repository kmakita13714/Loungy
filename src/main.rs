/*
 *
 *  This source file is part of the Loungy open source project
 *
 *  Copyright (c) 2024 Loungy, Matthias Grandl and the Loungy project contributors
 *  Licensed under MIT License
 *
 *  See https://github.com/MatthiasGrandl/Loungy/blob/main/LICENSE.md for license information
 *
 */

#![allow(dead_code)]

use app::run_app;
use gpui::App;

mod app;
mod assets;
mod commands;
mod components;
mod date;
mod db;
mod hotkey;
mod loader;
mod paths;
mod platform;
mod query;
mod state;
mod theme;
mod window;
mod workspace;

#[async_std::main]
async fn main() {
    env_logger::init();
    let app = App::new();

    run_app(app)
}
