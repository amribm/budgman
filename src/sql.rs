use chrono::{DateTime, Local};
use rusqlite::{Connection, Result};

struct DB {
    conn: Connection,
}

impl DB {
    fn new(file: String) -> Result<DB> {
        let conn = Connection::open(file)?;
        Ok(DB { conn })
    }

    fn init(&self) -> Result<()> {
        let create_budget_table = "
            CREATE TABLE budget (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                amount INTEGER NOT NULL DEFAULT 0,
            )";

        self.conn.execute(&create_budget_table, [])?;

        let create_expense_table = "
            CREATE TABLE expense (
                id INTEGER PRIMARY KEY,
                reason TEXT NOT NULL,
                time TEXT NOT NULL,
                amount INTEGER NOT NULL DEFAULT 0,
                budget_id INTEGER NOT NULL,
                category_id INTEGER NOT NULL,
                FOREIGN KEY (budget_id)
                        REFERENCES budget (id)
                        ON DELETE  CASCADE,
            )";

        self.conn.execute(&create_expense_table, [])?;

        let create_income_table = "
            CREATE TABLE income (
                id INTEGER PRIMARY KEY,
                reason TEXT NOT NULL,
                time TEXT NOT NULL,
                amount INTEGER NOT NULL DEFAULT 0,
                budget_id INTEGER NOT NULL,
                category_id INTEGER NOT NULL,
                FOREIGN KEY (budget_id)
                        REFERENCES budget (id)
                        ON DELETE  CASCADE,
            )";

        self.conn.execute(&create_income_table, [])?;
        Ok(())
    }

    fn get_all_budgets(&self) -> Result<Vec<Budget>> {
        let mut query = self.conn.prepare(
            "
            select
                name, id, amount
            from
                budget",
        )?;

        let budget_iter = query.query_map([], |row| {
            Ok(Budget {
                name: row.get(0)?,
                id: row.get(1)?,
                amount: row.get(2)?,
            })
        })?;

        let mut budgets = Vec::new();

        for budget in budget_iter {
            budgets.push(budget?);
        }

        Ok(budgets)
    }

    fn create_budget(&self, budget: Budget) -> Result<()> {
        let mut query = self.conn.prepare(
            "
            insert
            into
                budget(name,amount)
            values
                (?,?)",
        )?;
        let _ = query.execute((budget.name, budget.amount))?;
        Ok(())
    }

    fn get_budget_by_id(&self, id: u32) -> Result<Budget> {
        let mut query = self.conn.prepare(
            "
            select
                name, id, amount
            from
                budget
            where
                id = ?",
        )?;

        let budget = query.query_row([id], |row| {
            Ok(Budget {
                name: row.get(0)?,
                id: row.get(1)?,
                amount: row.get(2)?,
            })
        })?;

        Ok(budget)
    }

    fn update_budget_by_id(&self, budget: Budget) -> Result<()> {
        let mut query = self.conn.prepare(
            "
            update
                budget
            set
                name = ?,
                amount = ?
            where
                id = ?",
        )?;

        let _ = query.execute((budget.name, budget.amount, budget.id))?;

        Ok(())
    }

    fn delete_budget_by_id(&self, id: u32) -> Result<()> {
        let mut query = self.conn.prepare(
            "
            delete
            from
                budget
            where
                id = ?",
        )?;

        let _ = query.execute([id])?;

        Ok(())
    }

    fn add_expense_for_budget(&self, expense: Expense) -> Result<()> {
        let mut query = self.conn.prepare(
            "
            insert into
                expense(name,time,amount,budget_id)
            values (?,?,?,?)",
        )?;

        let _ = query.execute((
            expense.name,
            expense.time,
            expense.amount,
            expense.budget_id,
        ))?;

        Ok(())
    }

    fn get_expenses_for_budget(&self, budget_id: u32) -> Result<Vec<Expense>> {
        let mut query = self.conn.prepare(
            "
            select
                name, id, amount, time
            from
                expense
            where
                id = ?",
        )?;

        let expense_iter = query.query_map([budget_id], |row| {
            Ok(Expense {
                name: row.get(0)?,
                id: row.get(1)?,
                amount: row.get(2)?,
                time: row.get(3)?,
                budget_id,
            })
        })?;

        let mut expenses = Vec::new();
        for expense in expense_iter {
            expenses.push(expense?);
        }

        Ok(expenses)
    }

    fn get_expense_by_id(&self, id: u32) -> Result<Expense> {
        let mut query = self.conn.prepare(
            "
            select
                name, budget_id, amount, time
            from
                expense
            where
                id = ?",
        )?;

        let expense = query.query_row([id], |row| {
            Ok(Expense {
                name: row.get(0)?,
                budget_id: row.get(1)?,
                amount: row.get(2)?,
                time: row.get(3)?,
                id,
            })
        })?;

        Ok(expense)
    }

    fn update_expense_by_id(&self, expense: Expense) -> Result<()> {
        let mut query = self.conn.prepare(
            "
            update
                budget
            set
                name = ?,
                amount = ?,
                time = ?,
                budget_id = ?
            where
                id = ?",
        )?;

        let _ = query.execute((
            expense.name,
            expense.amount,
            expense.time,
            expense.budget_id,
            expense.id,
        ))?;

        Ok(())
    }

    fn delete_expense_by_id(&self, id: u32) -> Result<()> {
        let mut query = self.conn.prepare(
            "
            delete
            from
                expense
            where
                id = ?",
        )?;

        let _ = query.execute([id])?;

        Ok(())
    }

    fn add_income_for_budget(&self, income: Income) -> Result<()> {
        let mut query = self.conn.prepare(
            "
            insert into
                income(name,time,amount,budget_id)
            values (?,?,?,?)",
        )?;

        let _ = query.execute((income.name, income.time, income.amount, income.budget_id))?;

        Ok(())
    }

    fn get_incomes_for_budget(&self, budget_id: u32) -> Result<Vec<Income>> {
        let mut query = self.conn.prepare(
            "
            select
                name, id, amount, time
            from
                income
            where
                id = ?",
        )?;

        let income_iter = query.query_map([budget_id], |row| {
            Ok(Income {
                name: row.get(0)?,
                id: row.get(1)?,
                amount: row.get(2)?,
                time: row.get(3)?,
                budget_id,
            })
        })?;

        let mut incomes = Vec::new();
        for income in income_iter {
            incomes.push(income?);
        }

        Ok(incomes)
    }

    fn get_income_by_id(&self, id: u32) -> Result<Income> {
        let mut query = self.conn.prepare(
            "
            select
                name, budget_id, amount, time
            from
                income
            where
                id = ?",
        )?;

        let income = query.query_row([id], |row| {
            Ok(Income {
                name: row.get(0)?,
                budget_id: row.get(1)?,
                amount: row.get(2)?,
                time: row.get(3)?,
                id,
            })
        })?;

        Ok(income)
    }

    fn update_income_by_id(&self, income: Income) -> Result<()> {
        let mut query = self.conn.prepare(
            "
            update
                income
            set
                name = ?,
                amount = ?,
                time = ?,
                budget_id = ?
            where
                id = ?",
        )?;

        let _ = query.execute((
            income.name,
            income.amount,
            income.time,
            income.budget_id,
            income.id,
        ))?;

        Ok(())
    }

    fn delete_income_by_id(&self, id: u32) -> Result<()> {
        let mut query = self.conn.prepare(
            "
            delete
            from
                income
            where
                id = ?",
        )?;

        let _ = query.execute([id])?;

        Ok(())
    }
}

struct Budget {
    name: String,
    id: u32,
    amount: u64,
}

struct Expense {
    id: u32,
    name: String,
    time: DateTime<Local>,
    budget_id: u32,
    amount: u64,
}

struct Income {
    id: u32,
    name: String,
    time: DateTime<Local>,
    budget_id: u32,
    amount: u64,
}
