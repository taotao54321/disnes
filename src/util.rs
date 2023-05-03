use std::fs::File;
use std::io::{BufReader, Read as _};
use std::path::Path;

use anyhow::Context as _;

/// ファイルの指定範囲をバイト列として読み取る。
pub(crate) fn fs_read_range<P>(path: P, offset: usize, len: usize) -> anyhow::Result<Vec<u8>>
where
    P: AsRef<Path>,
{
    _fs_read_range(path.as_ref(), offset, len)
}

fn _fs_read_range(path: &Path, offset: usize, len: usize) -> anyhow::Result<Vec<u8>> {
    let rdr = File::open(path)?;
    let mut rdr = BufReader::new(rdr);

    let offset = i64::try_from(offset).context("seek offset overflow")?;
    rdr.seek_relative(offset)?;

    let mut buf = vec![0; len];
    rdr.read_exact(&mut buf)?;

    Ok(buf)
}
