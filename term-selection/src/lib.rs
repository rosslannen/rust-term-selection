pub use std::io::{self, Read as _, Write as _};

pub use term_selection_derive::*;

pub trait TermSelection: Sized + Copy + 'static {
    fn prompt() -> &'static str;

    fn description(self) -> &'static str;

    fn options() -> &'static [Self];

    fn get_single_selection() -> Self {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        let options = Self::options();

        println!("{}:", Self::prompt());
        for (i, option) in options.iter().enumerate() {
            println!("{:>2}) {}", i, option.description());
        }

        loop {
            print!("Enter selection: ");
            stdout.flush().expect("Could not flush stdout");

            let mut buf = String::new();
            stdin
                .read_line(&mut buf)
                .expect("Could not read from stdin");

            let input = buf.trim();

            if let Some(selection) = input
                .parse::<usize>()
                .ok()
                .and_then(|index| options.get(index))
            {
                break *selection;
            } else {
                eprintln!("'{}' is not a valid selection.", input);
                continue;
            }
        }
    }

    fn get_multi_selection() -> Vec<Self> {
        unimplemented!();
    }
}
