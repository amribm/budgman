use clap::{builder::ValueParser, value_parser, Arg, Command};

pub fn command() -> Command {
    Command::new("budgman")
        .about("budget manager for u :)")
        .subcommand(
            Command::new("budget")
                .subcommand(
                    Command::new("ls")
                        .about("lists  budgets")
                        .arg(Arg::new("name"))
                        .arg(Arg::new("amount").value_parser(value_parser!(u64)))
                        .arg(Arg::new("time").help("date in dd/mm/yyyy")),
                )
                .subcommand(Command::new("new").about("create new budget"))
                .subcommand(Command::new("edit").about("edit budget"))
                .subcommand(Command::new("rm").about("delete budget")),
        )
        .subcommand(
            Command::new("expence")
                .subcommand(Command::new("ls").about("lists expense for budget"))
                .subcommand(Command::new("new").about("adds expense for budget"))
                .subcommand(Command::new("edit").about("edits expense"))
                .subcommand(Command::new("rm").about("removes expense")),
        )
        .subcommand(
            Command::new("income")
                .subcommand(Command::new("ls").about("lists incomes for budget"))
                .subcommand(Command::new("new").about("adds income for budget"))
                .subcommand(Command::new("edit").about("edit existing income"))
                .subcommand(Command::new("rm").about("removes income")),
        )
}
