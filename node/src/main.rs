// Copyright 2021-2022 Green Aureus GmbH

// Permission is hereby granted, free of charge, to any person obtaining a copy 
// of this software and associated documentation files (the "Software"), to read 
// the Software only. Permission is hereby NOT GRANTED to use, copy, modify, 
// merge, publish, distribute, sublicense, and/or sell copies of the Software.

//! Green Aureus CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;

fn main() -> sc_cli::Result<()> {
    command::run()
}
