use std;

use csv;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub read_a: String,
    pub length_a: u64,
    pub begin_a: u64,
    pub end_a: u64,
    pub strand: char,
    pub read_b: String,
    pub length_b: u64,
    pub begin_b: u64,
    pub end_b: u64,
    pub nb_match_base: u64,
    pub nb_base: u64,
    pub mapping_quality: u64,
    pub sam_field: Vec<String>,
}

pub struct Records<'a, R: 'a + std::io::Read> {
    inner: csv::DeserializeRecordsIter<'a, R, RecordInner>,
}

type RecordInner = (
    String,
    u64,
    u64,
    u64,
    char,
    String,
    u64,
    u64,
    u64,
    u64,
    u64,
    Vec<String>,
);

impl<'a, R: std::io::Read> Iterator for Records<'a, R> {
    type Item = csv::Result<Record>;

    fn next(&mut self) -> Option<csv::Result<Record>> {
        self.inner.next().map(|res| {
            res.map(
                |(
                    read_a,
                    length_a,
                    begin_a,
                    end_a,
                    strand,
                    read_b,
                    length_b,
                    begin_b,
                    end_b,
                    nb_match_base,
                    nb_base,
                    mapping_quality_and_sam,
                )| {
                    let mapping_quality = mapping_quality_and_sam[0].parse::<u64>().unwrap();

                    let mut sam_field = Vec::new();
                    if mapping_quality_and_sam.len() > 1 {
                        sam_field = mapping_quality_and_sam[1..].to_vec();
                    }

                    Record {
                        read_a,
                        length_a,
                        begin_a,
                        end_a,
                        strand,
                        read_b,
                        length_b,
                        begin_b,
                        end_b,
                        nb_match_base,
                        nb_base,
                        mapping_quality,
                        sam_field,
                    }
                },
            )
        })
    }
}

pub struct Reader<R: std::io::Read> {
    inner: csv::Reader<R>,
}

impl<R: std::io::Read> Reader<R> {
    pub fn new(reader: R) -> Self {
        Reader {
            inner: csv::ReaderBuilder::new()
                .delimiter(b'\t')
                .has_headers(false)
                .flexible(true)
                .from_reader(reader),
        }
    }

    /// Iterate over all records.
    pub fn records(&mut self) -> Records<R> {
        Records {
            inner: self.inner.deserialize(),
        }
    }
}



