// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

pub mod cli_printers;

use std::path::PathBuf;

use clap::Parser;

use core_needle::io::{load_graph_from_file, load_policy_from_file};
use core_needle::policy::evaluator::evaluate;
use core_needle::policy::model::Policies;
use core_needle::representation::graph::Graph;
use core_needle::representation::Issue;

use crate::cli_printers::*;

/// Validate Sphinx-Needs JSON with policy rules
#[derive(Parser, Debug)]
#[command(name = "rusty-needle", version)]
#[command(about = "Validate Sphinx-Needs JSON with policy rules", long_about = None)]
struct Args {
    /// Path to needs file (Sphinx-Needs JSON)
    #[arg(long)]
    needs: PathBuf,

    /// Path to policies file (JSON)
    #[arg(long)]
    policies: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("Needs file: {}", args.needs.display());
    println!("Policies file: {}", args.policies.display());

    let loaded_graph =
        load_graph_from_file(args.needs.as_path()).expect("Can't load graph from file");

    let loaded_policy =
        load_policy_from_file(args.policies.as_path()).expect("Can't load policy from file");

    println!("Summary of loaded graph");
    println!("{}", cli_printers::issues(&loaded_graph, ColorMode::Always));

    println!("Running evaluation of the policy!");
    let issues: Vec<Issue> = evaluate(&loaded_graph, &loaded_policy);
    println!("{}", cli_printers::issues_from(&issues, ColorMode::Always));
}
