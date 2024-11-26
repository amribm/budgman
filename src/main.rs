mod budgman;
mod cli;
mod sql;

use budgman::{BudgMan, BudgManError};

fn main() -> Result<(), BudgManError> {
    let bman = BudgMan::new()?;
    let matches = cli::command().get_matches();

    match matches.subcommand() {
        Some(("budget", budget_matches)) => match budget_matches.subcommand() {
            Some(("new", _)) => {}
            Some(("edit", _)) => {}
            Some(("ls", _)) => {}
            Some(("rm", _)) => {}
            _ => println!("unknown subcommand"),
        },
        Some(("expense", expense_matches)) => match expense_matches.subcommand() {
            Some(("new", _)) => {}
            Some(("edit", _)) => {}
            Some(("ls", _)) => {}
            Some(("rm", _)) => {}
            _ => println!("unknown subcommand"),
        },
        Some(("income", income_matches)) => match income_matches.subcommand() {
            Some(("new", _)) => {}
            Some(("edit", _)) => {}
            Some(("ls", _)) => {}
            Some(("rm", _)) => {}
            _ => println!("unknown subcommand"),
        },
        _ => {
            bman.budget_stats()?;
        }
    }
    Ok(())
}
