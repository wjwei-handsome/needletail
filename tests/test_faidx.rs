use std::fs::File;
use std::io::{Seek, SeekFrom};
use needletail::parser::FastaReader;
// use needletail::parser::fasta::Reader;

use std::io::Cursor;
use needletail::FastxReader;


fn seq(s: &[u8]) -> Cursor<&[u8]> {
    Cursor::new(&s[..])
}

#[test]
fn test_faidx() {
    let path = "tests/data/test.fa";
    let mut reader = FastaReader::from_path(path).unwrap();
    reader.seek(SeekFrom::Start(0)).unwrap();
    while let Some(record) = reader.next() {
        let record = record.unwrap();
        println!("{:?}", record);
        let seq = record.seq();
        println!("{:?}", seq);
    }
}
