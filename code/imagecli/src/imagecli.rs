use imagix::resize::SizeOption;
use imagix::resize::Mode;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "resize",
    about = "A simple image resizing CLI tool",
    help = "Specify subcommand resize or stats. For help, type imagicli resize --help or imagicli stats --help"
)]
enum CommandLine {
    #[structopt(
        help = "Specify size (small/medium/large), mode (single/all) and path to image file or folder"
    )]
    Resize {
        #[structopt(long)]
        size: SizeOption,
        #[structopt(long, help = "Specify mode: single or all")]
        mode: Mode,
        #[structopt(long, parse(from_os_str), help = "Path to image file or folder")]
        path: PathBuf,
    },
    Stats {
        #[structopt(long, parse(from_os_str), help = "Path to folder with images")]
        path: PathBuf,
    },
}

fn main() {
    let args: CommandLine = CommandLine::from_args();

    match args {
        CommandLine::Resize { size, mode, path } => {
            match imagix::resize::process_resize_request(size, mode, path) {
                Ok(_) => println!("Resize operation completed successfully."),
                Err(e) => eprintln!("Error during resize operation: {}", e),
            }
        }
        CommandLine::Stats { path } => match imagix::stats::get_stats(path) {
            Ok((count, total_size)) => {
                println!("Total images: {}, Total size: {:.2} MB", count, total_size);
            }
            Err(e) => eprintln!("Error retrieving stats: {}", e),
        },
    }
}
