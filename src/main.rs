// !#[warn(unused_must_use)]

// use std::fmt;
use std::io::{self, Write, Read};
use git2::Repository;
use git2::BranchType;

fn main() -> Result<(), Error> {
    let mut stdout = io::stdout();
    let repo = Repository::open_from_env()?;

    for branch in repo.branches(Some(BranchType::Local))? {
        let (branch, branch_type) = branch?;
        let name = branch.name_bytes()?;
        stdout.write_all(name)?;
        write!(stdout, "\n")?;

        let commit = branch.get().peel_to_commit()?;
        let hash = commit.id();
        println!("The hash is => {}", hash);

        write!(stdout, "\n")?;
    }

    Ok(())
}


// fn main() -> Result<(), Error> {

    // crossterm::terminal::enable_raw_mode()?;
   
    // let mut stdout = io::stdout();
    // let mut stdin =  io::stdin().bytes();
    // loop {
    //     write!(stdout, "Type something >")?;
    //     stdout.flush()?;

    //         let byte = match stdin.next() {
    //             Some(byte) => byte?,
    //             None => break,
    //         };
    //         let c = char::from(byte);

    //         if c == 'q' {
    //             break;
    //         }

    //         // stdout.write_all(&[byte]).unwrap();

    //         write!(stdout, "You typed: {}\n\r",c )?;

    //         stdout.flush()?;
    // }
    // crossterm::terminal::disable_raw_mode()?;

//     Ok(())
    
// }

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    CrosstermErro(#[from] crossterm::ErrorKind),
   
    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    GitError(#[from] git2::Error),
}

// impl From<crossterm::ErrorKind> for Error {
//     fn from(error: crossterm::ErrorKind) -> Error {
//         Error::CrosstermErro(error)
//     }
// }

// impl From<io::Error> for Error {
//     fn from(error: io::Error) -> Self {
//         Error::IoError(error)
//     }
// }

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Error::CrosstermErro(inner) => write!(f, "{}", inner),
//             Error::IoError(inner) => write!(f, "{}", inner),
//         }
//     }
// }

// impl std::error::Error for Error {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match self {
//             Error::CrosstermErro(inner) => Some(inner),
//             Error::IoError(inner) => Some(inner),
//         }
//     }
// }