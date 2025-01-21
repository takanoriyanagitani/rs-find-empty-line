use std::io;

use std::io::BufWriter;
use std::io::Write;

use std::io::BufRead;
use std::io::BufReader;

use std::fs::File;

pub fn args_exclude_self() -> Result<impl Iterator<Item = String>, io::Error> {
    let mut i = std::env::args();
    i.next();
    Ok(i)
}

pub fn find_empty_line<I>(lines: I) -> Result<bool, io::Error>
where
    I: Iterator<Item = Result<Vec<u8>, io::Error>>,
{
    for rline in lines {
        let line: Vec<u8> = rline?;
        if line.is_empty() {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn filenames2filenames_with_empty_line<I>(
    filenames_all: I,
) -> Result<impl Iterator<Item = String>, io::Error>
where
    I: Iterator<Item = String>,
{
    Ok(filenames_all.filter_map(|filename: String| {
        File::open(&filename)
            .ok()
            .and_then(|f| {
                let br = BufReader::new(f);
                let lines = br.split(b'\n');
                find_empty_line(lines).ok()
            })
            .and_then(|found: bool| match found {
                true => Some(filename),
                false => None,
            })
    }))
}

pub fn writer_new<W>(mut wtr: W) -> impl FnMut(String) -> Result<(), io::Error>
where
    W: Write,
{
    move |filename: String| {
        write!(&mut wtr, "{filename}")?;
        writeln!(&mut wtr)?;
        wtr.flush()
    }
}

pub fn stdout_writer() -> impl FnMut(String) -> Result<(), io::Error> {
    let o = io::stdout();
    let ol = o.lock();
    let bw = BufWriter::new(ol);
    writer_new(bw)
}

pub fn filenames2writer<I, W>(filenames: I, mut writer: W) -> Result<(), io::Error>
where
    I: Iterator<Item = String>,
    W: FnMut(String) -> Result<(), io::Error>,
{
    for name in filenames {
        writer(name)?;
    }
    Ok(())
}

pub fn args2filtered_names2writer<W>(writer: W) -> Result<(), io::Error>
where
    W: FnMut(String) -> Result<(), io::Error>,
{
    let args_without_1st = args_exclude_self()?;
    let filtered = filenames2filenames_with_empty_line(args_without_1st)?;
    filenames2writer(filtered, writer)
}

pub fn args2filtered_names2stdout() -> Result<(), io::Error> {
    args2filtered_names2writer(stdout_writer())
}
