use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Pipe Viewer")]
pub struct Args {
    /// Read from a file instead of stdin
    #[structopt()]
    pub infile: Option<String>,

    /// Write output to a file instead of stdout
    #[structopt(short, long)]
    pub outfile: Option<String>,

    /// Display total bytes
    #[structopt(short, long)]
    pub silent: bool,
}

impl Args {
    pub fn get_args() -> Self {
        Args::from_args()
    }
}
