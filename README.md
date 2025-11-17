### Passwords generator CLI utility for command prompt generating passwords and saving it to the .txt file.

## Step 1: Downloading.

```bash
git clone https://github.com/ArutyunyanA/PSWDGEN.git
cd PSWDGEN
```
## Step 2: Installation.

```bash
cargo install --path .
```

## Step 3: Usage.


```bash

MacBook-Air-Witcher:~ macintosh$ pswdgen -h
Usage:

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


MacBook-Air-Witcher:~ macintosh$ pswdgen 16 2

██████╗ ███████╗██╗    ██╗██████╗  ██████╗ ███████╗███╗   ██╗
██╔══██╗██╔════╝██║    ██║██╔══██╗██╔════╝ ██╔════╝████╗  ██║
██████╔╝███████╗██║ █╗ ██║██║  ██║██║  ███╗█████╗  ██╔██╗ ██║
██╔═══╝ ╚════██║██║███╗██║██║  ██║██║   ██║██╔══╝  ██║╚██╗██║
██║     ███████║╚███╔███╔╝██████╔╝╚██████╔╝███████╗██║ ╚████║
╚═╝     ╚══════╝ ╚══╝╚══╝ ╚═════╝  ╚═════╝ ╚══════╝╚═╝  ╚═══╝
                                                             

Would you like to specify the passwords name?
Answers: 'yes' or 'no'
yes
Enter the name of resource or website: 
HackerOne
Would you like to specify the passwords name?
Answers: 'yes' or 'no'
yes   
Enter the name of resource or website: 
Google

Passwords saved to passwords.txt

MacBook-Air-Witcher:~ macintosh$ cat passwords.txt
 
Google: Gl!Y4=h%Bp14V>37
HackerOne: f91qhQxt=$jSC6zR
```




