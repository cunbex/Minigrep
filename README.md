# Minigrep
Simple version of the classic command line search tool grep (globally search a regular expression and print) for UTF-8 using RUST.
The project is proposed in the Rust programming book [Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) and its for educational purposes.

This grep project will combine a number of concepts:

- Structs & Enums (in chapter 5 & 6)
- Organizing code (modules in Chapter 7)
- Using vectors and strings (collections, Chapter 8)
- Handling errors (Chapter 9)
- Using traits where appropriate (Chapter 10)
- Writing tests (Chapter 11)
- Use Iterators & closures over Loops ( Chapter 13 )

## How to use

To run this command first clone the repo in your home directory:

```bash
cd $home
git clone https://github.com/cunbex/Minigrep.git
```

OPTIONAL: If needed add permissions for executing the binary (Depending on executables default permissions of your operating system)
```bash
cd Minigrep
chmod +x minigrep
```

move to the desired file that you wanna grep from, or move where you want to create a file and write stuff in it:

```bash
cd Desktop
touch test.txt
```

write these lines in the text file

```text
123
helLo us
HELLO me
hello you
```

## command usage

```bash
Minigrep/minigrep Query Filename [IGNORE_CASE:0|1]
```
executing the command, using the past file created :

- For case insensitive
```bash
~/Minigrep/minigrep hello ~/Desktop/test.txt 1
```
output
```text
#########################################

"helLo us"
"HELLO me"
"hello you"

#########################################
```

- For case sensitive
```bash
~/Minigrep/minigrep hello ~/Desktop/test.txt 0
```
output
```text
#########################################

"hello you"

#########################################
```

- Note: 3rd argument is optional and if not provided the program will execute in case sensitive mode.
```bash
~/Minigrep/minigrep hello ~/Desktop/test.txt
```
output
```text
IGNORE_CASE environment variable: environment variable not found, Applying default search case: 'Sensitive'.
Looking for 'hello' with case:'Sensitive' in '/home/null/Desktop/test.txt'.

#########################################

"hello you"

#########################################
```

## Using environment variables
we can use environment variables instead of arguments for case sensitivity, the program will look for an environment variable with 
`KEY = IGNORE_CASE` & `VALUE = 0 | 1` (1 Meaning case insensitive).

```bash
export IGNORE_CASE=1
~/Minigrep/minigrep hello ~/Desktop/test.txt
```

If we already have an environment variable + we provide the case sensitivity argument, the argument will overlap the environment variable.
for example this execution will occure in insensitive mode, & the environment variable `IGNORE_CASE` will be overlapped to `1`.

```bash
export IGNORE_CASE=0
~/Minigrep/minigrep hello ~/Desktop/test.txt 1
```
