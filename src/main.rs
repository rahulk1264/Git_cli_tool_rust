// !#[warn(unused_must_use)]

// use std::fmt;
use std::io::{self, Write, Read};


fn main() -> Result<(), Error> {

    crossterm::terminal::enable_raw_mode()?;
   
    let mut stdout = io::stdout();
    let mut stdin =  io::stdin().bytes();
    loop {
        write!(stdout, "Type something >")?;
        stdout.flush()?;

            let byte = match stdin.next() {
                Some(byte) => byte?,
                None => break,
            };
            let c = char::from(byte);

            if c == 'q' {
                break;
            }

            // stdout.write_all(&[byte]).unwrap();

            write!(stdout, "You typed: {}\n\r",c )?;

            stdout.flush()?;
    }

    crossterm::terminal::disable_raw_mode()?;

    Ok(())
    
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    CrosstermErro(#[from] crossterm::ErrorKind),
   
    #[error(transparent)]
    IoError(#[from] io::Error),
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