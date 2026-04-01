use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};

use super::RepositoryGenerator;
use crate::cli::Arguments;
use crate::domain::prime::PrimeResult;

use async_trait::async_trait;

pub struct BinaryU64 {
    block_size: usize,
    buffer_size: usize,
}

impl BinaryU64 {
    pub fn new() -> Self {
        BinaryU64 {
            block_size: std::mem::size_of::<usize>(),
            buffer_size: 1048576usize,
        }
    }
}

#[async_trait]
impl RepositoryGenerator for BinaryU64 {
    async fn append(&self, args: &Arguments, result: &PrimeResult) {
        // ファイルオプションがなければ終了
        let Some(path) = args.binary_file.clone() else {
            return;
        };

        // ファイルオープン
        let file = File::create(&path)
            .await
            .expect("ファイルのオープンに失敗しました");
        let mut writer = BufWriter::new(file);

        // ファイル書き込み()
        for prime in &result.primes {
            let buffer = usize::to_le_bytes(*prime);
            writer
                .write_all(&buffer)
                .await
                .expect("ファイルへの書き込みに失敗しました");
        }
        writer
            .flush()
            .await
            .expect("ファイルへの書き込みに失敗しました");
    }

    async fn extract(&self, args: &Arguments) -> Vec<usize> {
        // ファイルオプションがなければ終了
        let Some(path) = args.binary_file.clone() else {
            return vec![];
        };

        // ファイルオープン
        let mut file = File::open(&path)
            .await
            .expect("ファイルのオープンに失敗しました");
        let metadata = file
            .metadata()
            .await
            .expect("ファイルのメタデータの取得に失敗しました");

        // バイト列をusizeのベクタに変換
        let primes_count = metadata.len() as usize / self.block_size;
        let mut remain_size = primes_count * self.block_size;
        let mut primes: Vec<usize> = Vec::with_capacity(primes_count);

        // バイト列をベクタに変換
        let buffer_size = self.buffer_size;
        loop {
            if remain_size == 0 {
                break;
            }
            // ファイルリード
            let read_size = remain_size.min(buffer_size);
            let mut buffer = vec![0u8; read_size];
            let read_bytes = file
                .read_exact(&mut buffer)
                .await
                .expect("ファイルのリードに失敗しました");

            // バイト列をusizeのベクタに変換
            for index in 0..read_bytes / self.block_size {
                let prime = usize::from_le_bytes(
                    buffer[index * self.block_size..(index + 1) * self.block_size]
                        .try_into()
                        .expect("コード変換に失敗しました"),
                );
                primes.push(prime);
            }

            remain_size -= read_bytes;
        }

        primes
    }
}
