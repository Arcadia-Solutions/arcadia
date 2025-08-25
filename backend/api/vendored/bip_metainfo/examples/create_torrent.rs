extern crate bip_metainfo;
extern crate chrono;
extern crate pbr;

use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

use bip_metainfo::error::ParseResult;
use bip_metainfo::{Metainfo, MetainfoBuilder};
use chrono::offset::{TimeZone, Utc};
use pbr::ProgressBar;

fn main() {
    println!("\nIMPORTANT: Remember to run in release mode for real world performance...\n");

    let input = io::stdin();
    let mut input_lines = input.lock().lines();
    let mut output = io::stdout();

    output.write_all(b"Enter A Source Folder/File: ").unwrap();
    output.flush().unwrap();
    let src_path = input_lines.next().unwrap().unwrap();

    output
        .write_all(b"Enter A File Name (With Extension): ")
        .unwrap();
    output.flush().unwrap();
    let dst_path = input_lines.next().unwrap().unwrap();

    match create_torrent(&src_path) {
        Ok(bytes) => {
            let mut output_file = File::create(dst_path).unwrap();
            output_file.write_all(&bytes).unwrap();

            print_metainfo_overview(&bytes);
        }
        Err(error) => println!("Error With Input: {:?}", error),
    }
}

/// Create a torrent from the given source path.
fn create_torrent<S>(src_path: S) -> ParseResult<Vec<u8>>
where
    S: AsRef<Path>,
{
    let count = 10000;
    let mut pb = ProgressBar::new(count);
    pb.format("╢▌▌░╟");

    let builder = MetainfoBuilder::new()
        .set_created_by(Some("bip_metainfo"))
        .set_comment(Some("Just Some Comment"));

    let mut prev_progress = 0;
    builder.build(2, src_path, move |progress| {
        let whole_progress = (progress * (count as f64)) as u64;
        let delta_progress = whole_progress - prev_progress;

        if delta_progress > 0 {
            pb.add(delta_progress);
        }
        prev_progress = whole_progress;
    })
}

/// Print general information about the torrent.
fn print_metainfo_overview(bytes: &[u8]) {
    let metainfo = Metainfo::from_bytes(bytes).unwrap();
    let info = metainfo.info();
    let info_hash_hex = metainfo
        .info()
        .info_hash()
        .as_ref()
        .iter()
        .map(|b| format!("{:02X}", b))
        .fold(String::new(), |mut acc, nex| {
            acc.push_str(&nex);
            acc
        });
    let utc_creation_date = metainfo.creation_date().map(|c| Utc.timestamp(c, 0));

    println!(
        "\n\n-----------------------------Metainfo File Overview-----------------------------"
    );

    println!("InfoHash: {}", info_hash_hex);
    println!("Main Tracker: {:?}", metainfo.main_tracker());
    println!("Comment: {:?}", metainfo.comment());
    println!("Creator: {:?}", metainfo.created_by());
    println!("Creation Date: {:?}", utc_creation_date);

    println!("Directory: {:?}", info.directory());
    println!("Piece Length: {:?}", info.piece_length());
    println!("Number Of Pieces: {}", info.pieces().count());
    println!("Number Of Files: {}", info.files().count());
    println!(
        "Total File Size: {}",
        info.files().fold(0, |acc, nex| acc + nex.length())
    );
}
