use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};

use super::RepositoryGenerator;
use crate::cli::Arguments;
use crate::domain::prime::PrimeResult;

use async_trait::async_trait;
use bincode::config;

pub struct BinaryVector;

impl BinaryVector {
    #[allow(dead_code)]
    pub fn new() -> Self {
        BinaryVector {}
    }
}

#[async_trait]
impl RepositoryGenerator for BinaryVector {
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
        let config = config::standard().with_little_endian();
        let encoded = bincode::encode_to_vec(&result.primes, config).expect("変換に失敗しました");
        writer
            .write_all(&encoded)
            .await
            .expect("ファイルへの書き込みに失敗しました");
        writer
            .flush()
            .await
            .expect("ファイルへの書き込みに失敗しました");
    }

    #[allow(unused_variables)]
    async fn extract(&self, args: &Arguments) -> Vec<usize> {
        // ファイルオプションがなければ終了
        let Some(path) = args.binary_file.clone() else {
            return vec![];
        };

        // ファイルリード
        let bytes = tokio::fs::read(&path)
            .await
            .expect("ファイルのリードに失敗しました");

        let config = config::standard().with_little_endian();
        let (decoded, _): (Vec<usize>, usize) =
            bincode::decode_from_slice(&bytes, config).expect("ファイルのリードに失敗しました");

        decoded
    }
}
