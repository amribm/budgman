use crate::sql::{Budget, Income, DB};
use chrono::{DateTime, Local};
use console::Style;
use std::{env, fs, io, num::ParseFloatError, os, path::PathBuf};
use thiserror::Error;

pub struct BudgMan {
    db: DB,
}

impl BudgMan {
    pub fn new() -> Result<BudgMan, BudgManError> {
        let home = env!("HOME");

        let db_path = format!("{}/.config/budgman/data.db", home);
        let mut new_file = false;

        let path = PathBuf::from(db_path);

        // creats a db file if now exisita
        if !path.exists() {
            let dir_path = PathBuf::from(format!("{}/.config/budgman", home));
            if !dir_path.exists() {
                fs::DirBuilder::new().create(dir_path)?;
            }
            let _ = fs::File::create(path.clone())?;
            new_file = true;
        }

        let db = DB::new(path)?;

        if new_file {
            db.init()?;
        }

        Ok(BudgMan { db })
    }

    pub fn budget_stats(&self) -> Result<(), BudgManError> {
        let budget = env::var("BUDG_ID").ok();
        let mut budget_id: u32;

        if let Some(b) = budget {
            budget_id = b.parse().or(Err(BudgManError::InvalidBudgetId(b)))?;
        } else {
            budget_id = self.ask_to_chose_budget()?;
            env::set_var::<&str, String>("BUDG_ID", format!("{}", budget_id));
        }
        let budget = self.db.get_budget_by_id(budget_id)?;

        let total_expense = self.db.sum_of_expense_for_budget(budget_id)?;
        let total_income = self.db.get_sum_of_income_for_budget(budget_id)?;

        println!("budget name: {}", budget.name);
        println!("budget amount: {}", budget.amount);
        println!("total expense: {}", total_expense);
        println!("total income: {}", total_income);

        Ok(())
    }

    fn ask_to_chose_budget(&self) -> Result<u32, BudgManError> {
        let budgets = self.db.get_all_budgets()?;

        if budgets.len() == 0 {
            let red = Style::new().red();
            println!("{}", red.apply_to("no budgets found"));
            let cyan = Style::new().red();
            println!(
                "create new budget using: {}",
                cyan.apply_to("budgman budget new")
            );
            std::process::exit(0);
        }

        for budget in budgets {
            println!(
                "id:{} name:{} amount:{}",
                budget.id, budget.name, budget.amount
            );
        }
        println!("select budget (enter id):");

        let budget_id: u32;
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let input = input.parse::<u32>();

            if input.is_err() {
                println!("invalid input, please enter valid id");
                continue;
            }
            budget_id = input.unwrap();

            break;
        }

        Ok(budget_id)
    }

    pub fn create_budget(&self, name: String, amount: u64) -> Result<(), BudgManError> {
        let budget = Budget {
            id: 0,
            name,
            amount,
        };
        self.db.create_budget(budget)?;
        Ok(())
    }

    pub fn create_expense(
        &self,
        budget_id: u32,
        name: String,
        amount: u64,
        time: DateTime<Local>,
    ) -> Result<(), BudgManError> {
        let expense = Expense {
            id: 0,
            name,
            amount,
            budget_id,
            time,
        };
        self.db.add_expense_for_budget(expense)?;
        Ok(())
    }

    pub fn create_income(
        &self,
        budget_id: u32,
        name: String,
        amount: u64,
        time: DateTime<Local>,
    ) -> Result<(), BudgManError> {
        let income = Income {
            id: 0,
            name,
            amount,
            budget_id,
            time,
        };
        self.db.add_income_for_budget(income)?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum BudgManError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    DBError(#[from] rusqlite::Error),

    #[error("invalid budget id {}",.0)]
    InvalidBudgetId(String),
}
