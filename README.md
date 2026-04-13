# kip_count - a skip-gram counter written in Rust

This repository contains the source code for *kip_count*, an efficient skip-gram counter written in Rust. An n-gram is any sequence of n consecutive words in a text corpus. Similarly, a skip-gram is an n-gram with k words skipped within the sequence (this is also called a k-skip-n-gram). 

The program is already quite fast and efficient, though there are many features that remain to be added.

## To be added in the future

- more efficient data reading
- the option to ignore certain words
- stopping at segment breaks (such as periods and/or commas and other punctuation)
- the option to use different word separation strategies (this includes support for other, commonly-used formats for linguistically structured data, e.g. CoNLL-U)
- more efficient sorting (sorting during the counting process as opposed to the currently used sorting phase at the end)
- etc.

## CLI usage

`./kip_count k n /path/to/input/directory path/to/output/file`

where k is the number of intervening words to be skipped within the n-gram and n is the number of words to be considered in the main sequence

