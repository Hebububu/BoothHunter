use std::path::PathBuf;
use std::sync::Mutex;

use rusqlite::{params, Connection};

use crate::error::{AppError, AppResult};

pub struct AppDatabase {
    conn: Mutex<Connection>,
}

impl AppDatabase {
    pub fn initialize(app_data_dir: PathBuf) -> Result<Self, AppError> {
        std::fs::create_dir_all(&app_data_dir)
            .map_err(|e| AppError::Database(format!("Failed to create data dir: {}", e)))?;

        let db_path = app_data_dir.join("boothhunter.db");
        let conn = Connection::open(&db_path)?;

        conn.execute_batch("PRAGMA journal_mode=WAL;")?;

        // Idempotent migrations
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS cached_items (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                price INTEGER NOT NULL,
                category_name TEXT,
                shop_name TEXT,
                url TEXT NOT NULL,
                images_json TEXT,
                tags_json TEXT,
                cached_at TEXT DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS favorites (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                item_id INTEGER NOT NULL UNIQUE,
                name TEXT NOT NULL,
                price INTEGER NOT NULL,
                thumbnail_url TEXT,
                category_name TEXT,
                shop_name TEXT,
                added_at TEXT DEFAULT (datetime('now')),
                note TEXT
            );

            CREATE TABLE IF NOT EXISTS search_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                keyword TEXT NOT NULL,
                searched_at TEXT DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS popular_avatars (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name_ja TEXT NOT NULL UNIQUE,
                name_ko TEXT NOT NULL,
                item_count INTEGER DEFAULT 0,
                thumbnail_url TEXT,
                updated_at TEXT DEFAULT (datetime('now')),
                is_default INTEGER DEFAULT 0
            );",
        )?;

        // Seed default popular avatars (INSERT OR IGNORE is idempotent)
        Self::seed_default_avatars(&conn)?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn conn(&self) -> AppResult<std::sync::MutexGuard<'_, Connection>> {
        self.conn.lock().map_err(|e| {
            log::error!("Database mutex poisoned: {}", e);
            AppError::Database(format!("Lock poisoned: {}", e))
        })
    }

    fn seed_default_avatars(conn: &Connection) -> AppResult<()> {
        let defaults: &[(&str, &str)] = &[
            ("しなの", "시나노"),
            ("マヌカ", "마누카"),
            ("セレスティア", "세레스티아"),
            ("リリエル", "릴리엘"),
            ("キュピッド", "큐피드"),
            ("カリン", "카린"),
            ("ルシュカ", "루슈카"),
            ("舞", "마이"),
            ("桜", "사쿠라"),
            ("ここあ", "코코아"),
            ("イメリス", "이메리스"),
            ("ミラン", "미란"),
            ("うるる", "우루루"),
            ("イチゴ", "이치고"),
            ("桔梗", "키쿄"),
        ];
        for (ja, ko) in defaults {
            conn.execute(
                "INSERT OR IGNORE INTO popular_avatars (name_ja, name_ko, is_default) VALUES (?1, ?2, 1)",
                params![ja, ko],
            )?;
        }
        Ok(())
    }
}
