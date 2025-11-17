use crate::constants::FIGLET_FONT;
use figlet_rs::FIGfont;

pub fn banner_print() {
    let font = FIGfont::from_content(FIGLET_FONT).expect("Failed to load embedded FIGlet font!");

    let banner = font.convert("PSWDGEN").unwrap();
    println!("{}", banner);
}

pub fn print_help() {
    println!(
        r#"Usage:

    pswdgen <length> <count> [options]

Arguments:

    <length>               Password length (number of characters)
    <count>                How many passwords to generate

Options:

    -h, --help             Show this help menu
    -v, --version          Display version information

Examples:

    pswdgen 16 5
    pswdgen 24 10 > passwords.txt

Description:

    Generates random passwords using cryptographic RNG (OsRng).
    Each password may optionally be labeled interactively.

"#
    );
}
