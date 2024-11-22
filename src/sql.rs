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
                FOREIGN KEY (category_id)
                        REFERENCES category (id)
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
                FOREIGN KEY (category_id)
                        REFERENCES category (id)
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

    fn get_update_by_id(&self, budget: Budget) -> Result<()> {
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

    fn get_delete_by_id(&self, id: u32) -> Result<()> {
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
}

struct Budget {
    name: String,
    id: u32,
    amount: u64,
}
