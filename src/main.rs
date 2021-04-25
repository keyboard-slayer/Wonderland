mod uac;

use std::io;
use std::process::exit;

use is_elevated::is_elevated;

fn main() -> io::Result<()>
{
    if !is_elevated()
    {
        uac::uac()?;
        exit(0);
    }

    Ok(())
}