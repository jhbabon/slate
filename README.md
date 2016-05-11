# Slate

Slate is a snippets manager for your command line.

It helps you to have all those small pieces of code, text, and so on in one place and in your favorite environment: the shell.

## Installation

Slate is made with `rust`, so you will need the [latest stable version](https://www.rust-lang.org/downloads.html) of it to compile and run the program.

Clone the repository and run `cargo install`. You can also run `cargo build` if you want only to play with it:

```
$ git clone slate.git slate
$ cd slate
$ cargo install
```

## Usage

You can always check all the commands available with the `--help` option:

```
$ slate --help
Slate: Manage your snippets from your command line.

Note that Slate will use the file ~/.slate to save
its contents.

Usage:
  slate <command> [<args>...]
  slate [options]

Options:
  -h --help      Show this screen.
  -v --version   Show version.

Commands:
   set     Write a new key and value.
   get     Read a key.
   list    List all keys.
   rename  Rename a key.
   remove  Delete a key.
   exec    Run a key value as a command.
   snippet Get a key and replace all placeholders with new data.
```

As you can see, Slate is basically a `HashMap`, so playing with it is straight forward:

```
# Basic operations
$ slate set foo bar
$ slate get foo
bar
$ slate list
foo
$ slate remove foo
The key has been removed
$ slate list

# Renaming
$ slate set bar baz
$ slate list
bar

$ slate rename bar foo
The key has been renamed
$ slate list
foo

# Cleaning up
$ slate remove --all
All keys have been removed
```

It also plays nice with other UNIX tools:

```
$ cat redis.conf | slate set redis
$ slate get redis
daemonize yes
dbfilename dump.rdb
dir /tmp

$ cd other/project
$ slate get redis > redis.conf
$ cd other/project/v2
$ slate get redis > redis.conf
```

### The `~/.slate` file

By default Slate will save all its contents in the file `~/.slate`. It is just a plain JSON file for the moment. It is better if you don't touch it and let the program handle it.

## Experimental

Slate also offers some extra commands to make it a little bit more useful, or not, maybe. But these commands are nice to play with until you find how useful they can be.

### exec

If you don't want to clutter your aliases, or you want to save a temporal command, you can use the `exec` subcommand to run your keys' values as shell programs:

```
$ slate set echo "echo 'hello world!'"
$ slate exec echo
hello world!
```

### snippet

You can also save a template and reuse it with different values. A template, or snippet, is composed by text and some _placeholders_. These placeholders is what you want to substitue with fresh values. A placeholder has the form `:placeholder:`, where `placeholder` can be whatever text you want.

You can see a minimum example of how to use here:

```
$ slate set tepl ':c: :m:'
$ slate snippet tepl -r c echo m 'Hello World!'
echo Hello World!
```
