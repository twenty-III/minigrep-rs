# minigrep-rs 👋

<div align="center">

**A cool little command-line tool I built with Rust. Think of it like a simple, super-fast version of `grep`.**

</div>

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.78.0-orange.svg" alt="Rust Version">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/platform-cross--platform-lightgrey.svg" alt="Platform">
</p>

This whole project is basically my way of getting better at Rust by making something useful and fun! With `minigrep-rs`, you can easily search for text inside your files right from the terminal. It's all about clean code and doing one thing really well!

---

## 📚 Table of Contents
* [What it can do!](#what-it-can-do-)
* [How to Use It](#how-to-use-it-️)
* [Let's See Some Examples](#lets-see-some-examples-)
* [My Learning Adventure & What's Next](#my-learning-adventure--whats-next-)

---

## What it can do! ✨

* **Find what you're looking for:** 🔍 Just give it a word or phrase, and it'll search for it in any file.
* **Case doesn't matter:** 🔠 Use a simple flag, and it'll find your text whether it's uppercase, lowercase, or mixed!
* **Know where you are:** 📍 It can show you the exact line number for every match it finds. Super handy!
* **Just the facts:** 📊 If you only need to know how many times something appears, it can just give you the total count.
* **Handles errors gracefully:** 👍 If you forget a file or type something wrong, it'll give you a helpful message instead of just crashing.
* **It's fast!** ⚡️ Thanks to Rust, this thing is speedy and safe with your computer's memory.

---

## How to Use It 🛠️

Getting started is a piece of cake. First, you'll want to build the project to get the best performance.

```sh
cargo build --release
```

### The Basic Command

Once that's done, you can run it like this from your terminal:

```sh
./target/release/minigrep-rs <what to search for> <what file to search in> [flags]
```

* `<what to search for>`: The text you're trying to find.
* `<what file to search in>`: The path to the file you want to search.
* `[flags]`: These are optional little extras to change how it searches.

### All the Awesome Flags 🚩

Here are the flags you can use to spice up your searches!

| Short Flag | Long Flag         | What it Does                                                 |
| :--------- | :---------------- | :----------------------------------------------------------- |
| `-ic`      | `--ignore-case`   | Makes your search case-insensitive.                          |
| `-ln`      | `--line-number`   | Shows the line number for every match.                       |
| `-co`      | `--count-only`    | Just shows you how many lines matched, nothing else.         |

> **Just a heads-up:** You can't use the line number (`-ln`) and count (`-co`) flags at the same time. It's one or the other!

---

## Let's See Some Examples 💡

Imagine you have a file called `poem.txt` that says:

> I'm nobody! Who are you?
> Are you nobody, too?
> Then there's a pair of us - don't tell!
> They'd banish us, you know.

#### A Simple Search

```sh
./target/release/minigrep-rs "nobody" poem.txt
```

**It'll show you:**

```
I'm nobody! Who are you?
Are you nobody, too?
```

#### A Fancy Search (Case-Insensitive with Line Numbers!)

```sh
./target/release/minigrep-rs "Nobody" poem.txt -ic -ln
```

**And you'll get:**

```
I'm nobody! Who are you? (line=1)
Are you nobody, too? (line=2)
```

#### Just Counting

```sh
./target/release/minigrep-rs "us" poem.txt -co
```

**The output is simple:**

```
Count: 2
```

---

## My Learning Adventure & What's Next 🚀

Honestly, I started this project to get a good handle on some key Rust concepts, like:

* How to use structs and `impl` blocks
* Dealing with errors using `Result`
* Reading files from the computer
* Organizing code into modules and writing tests
* Figuring out how to handle command-line arguments from scratch

I'm planning to keep adding to this project as I learn new things! Here's what I'm thinking about for the future:

* **Make argument parsing better:** I'll probably switch to the `clap` library to make adding new features easier.
* **Search more file types:** Wouldn't it be cool if it could search inside PDFs? 📄
* **Search the web:** Maybe add a feature to pull down a webpage and search its text. 🌐
* **Get crazy with AI/ML:** It would be awesome to use some machine learning to search for text inside of images! 🤖

Thanks for checking out my project! 😊
