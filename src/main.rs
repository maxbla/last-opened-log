use std::env::current_dir;
use std::error::Error;
use std::fs::read_dir;
use std::os::unix::process::CommandExt;
use std::process::Command;

use clap::{App, Arg};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Lol - Last opened log viewer")
        .version("0.1")
        .author("Max B. <blachmanmax@gmail.com>")
        .about("view last modified file using a pager")
        .arg(
            Arg::with_name("pager")
                .short("p")
                .long("pager")
                .value_name("PAGER")
                .help("Prints name of last edited file, or use -p to select which pager program to use.")
                .takes_value(true),
        )
        .get_matches();

    let mut use_pager = true;
    let pager = matches.value_of("pager").unwrap_or_else(||{
    	use_pager = false;
	return "no_pager_str";
    });

    let work_dir = current_dir()?;
    let dir_iter = read_dir(work_dir.clone())?;
    let recent = dir_iter
        .filter_map(|file| file.ok()) // filter out permission issues
        .filter_map(|f| f.metadata().map(|m| (f, m)).ok())
        .filter(|(_, meta)| meta.is_file()) // only files
        .max_by(|(_, meta1), (_, meta2)| {
            let err_str = "Could not find modified time";
            meta1
                .modified()
                .expect(err_str)
                .cmp(&meta2.modified().expect(err_str))
        });
    let (recent_file, _) = match recent {
        None => {
            let s = work_dir.as_os_str().to_str().unwrap();
            eprintln!("No files found in working directory: {}", s);
            return Ok(());
        }
        Some(file) => file,
    };

    if use_pager {
       Command::new(pager).arg(recent_file.path()).exec();
    } else {
       println!("{}", recent_file.path().into_os_string().into_string().unwrap());
    }

    Ok(())
}
