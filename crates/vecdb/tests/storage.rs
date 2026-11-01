//! Expected behaviour of the write-ahead log.
//! Run with: cargo test -p vecdb -- --ignored

use std::path::PathBuf;
use vecdb::storage::{Wal, WalRecord};

fn tmp_path(name: &str) -> PathBuf {
    let p = std::env::temp_dir().join(format!("memsearch-test-{}-{name}", std::process::id()));
    let _ = std::fs::remove_file(&p);
    p
}

#[test]
#[ignore = "implement storage::Wal"]
fn wal_round_trip() {
    let path = tmp_path("roundtrip.wal");
    let records = vec![
        WalRecord::Insert {
            id: 1,
            vector: vec![0.5, 1.5],
        },
        WalRecord::Remove { id: 1 },
        WalRecord::Insert {
            id: 2,
            vector: vec![2.0, -1.0],
        },
    ];
    {
        let mut wal = Wal::open(&path).unwrap();
        for r in &records {
            wal.append(r).unwrap();
        }
        wal.sync().unwrap();
    }
    assert_eq!(Wal::replay(&path).unwrap(), records);
}

#[test]
#[ignore = "implement storage::Wal"]
fn wal_ignores_torn_tail() {
    let path = tmp_path("torn.wal");
    {
        let mut wal = Wal::open(&path).unwrap();
        wal.append(&WalRecord::Insert {
            id: 7,
            vector: vec![1.0; 8],
        })
        .unwrap();
        wal.append(&WalRecord::Insert {
            id: 8,
            vector: vec![2.0; 8],
        })
        .unwrap();
        wal.sync().unwrap();
    }
    // Simulate a crash mid-write: chop a few bytes off the end.
    let len = std::fs::metadata(&path).unwrap().len();
    let f = std::fs::OpenOptions::new().write(true).open(&path).unwrap();
    f.set_len(len - 3).unwrap();

    let got = Wal::replay(&path).unwrap();
    assert_eq!(
        got,
        vec![WalRecord::Insert {
            id: 7,
            vector: vec![1.0; 8]
        }]
    );
}
