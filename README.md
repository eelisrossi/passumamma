# A password generator for old ladies

This is partly a joke and partly for work.

Created in Rust because I want it to be blazingly fast.

### How to Use

Clone the repo and build it with cargo. 

``
    $cargo build
``

You can run it with any wordlist .csv, but there is also one included.

(it's a list of around 10 000 Finnish words)

At the moment it will output ten different passphrases.

``
    $cargo run ./nykysuomensanalista2022.csv 
``


### Main functions

1. Create a secure but human-readable passphrase

2. Use any .csv of words with the words at [0] and a '\t' delimiter

### TODO (if I don't get sidetracked to a new project)

1. Add ability to determine how many words there are
 
2. Add a more comprehensive parser for differently formatted .csv-files
