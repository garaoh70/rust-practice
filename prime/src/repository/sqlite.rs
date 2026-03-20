use super::RepositoryGenerator;
use crate::cli::Arguments;
use crate::domain::prime::PrimeResult;
use crate::entity;

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ConnectionTrait, TransactionTrait};

pub struct SQLite {}

impl SQLite {
    pub fn new() -> Self {
        SQLite {}
    }
}

#[async_trait]
impl RepositoryGenerator for SQLite {
    async fn append(&self, args: &Arguments, result: &PrimeResult) {
        // データベースオプションがなければ終了
        let Some(path) = args.database.clone() else {
            return;
        };

        // データベース接続
        let db = sea_orm::Database::connect(format!("sqlite:{}?mode=rwc", path))
            .await
            .expect("データベース接続に失敗しました");

        // テーブルがなければテーブルを作成する
        db.execute(sea_orm::Statement::from_string(
            db.get_database_backend(),
            "
                CREATE TABLE IF NOT EXISTS primes (
                    prime_index INTEGER PRIMARY KEY,
                    value INTEGER NOT NULL UNIQUE
                );

                CREATE TABLE IF NOT EXISTS prime_history (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    caluculated_at TEXT DEFAULT (datetime('now', 'localtime')),
                    max_value INTEGER NOT NULL,
                    prime_count INTEGER NOT NULL,
                    alhotiyhm TEXT
                );
                ",
        ))
        .await
        .expect("SQLの実行に失敗しました");

        // トランザクション開始
        let txn = db
            .begin()
            .await
            .expect("トランザクション開始に失敗しました");

        // 素数の追加
        for value in result.primes.iter().map(|&x| x as i64) {
            let new_prime = entity::primes::ActiveModel {
                value: sea_orm::Set(value),
                ..Default::default()
            };
            new_prime
                .insert(&txn)
                .await
                .expect("素数の追加に失敗しました");
        }

        // 記録の追加
        let new_history = entity::prime_history::ActiveModel {
            max_value: sea_orm::Set(args.number as i64),
            prime_count: sea_orm::Set(result.primes.last().copied().unwrap_or(0usize) as i64),
            alhotiyhm: sea_orm::Set(result.sieve_name.to_string()),
            ..Default::default()
        };
        new_history
            .insert(&txn)
            .await
            .expect("記録の追加に失敗しました");

        // データベース接続
        txn.commit().await.expect("データベース更新に失敗しました");
    }

    #[allow(unused_variables)]
    async fn extract(&self, start: usize, length: usize) -> Vec<usize> {
        vec![]
    }
}
