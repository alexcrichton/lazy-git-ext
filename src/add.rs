extern crate lazy_git_ext;

use lazy_git_ext::LazyGit;

const USAGE: &'static str = "usage: add-all [options]

Add all changes to index.

Options:
-h/--help    - Prints help.
";

fn main() {
    for arg in std::env::args().skip(1) {
        match arg.as_ref() {
            "-h" | "--help" => {
                println!("{}", USAGE);
                return;
            }
            _ => {
                println!("Incorrect usage\n{}", USAGE);
                return;
            }
        }
    }

    lazy_git_ext::open_repo(".").add_all().expect("Cannot add to repository");
}
