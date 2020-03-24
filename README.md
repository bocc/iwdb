# IWDB

 

This service checks whether a word posted to it exists in a set of words.

 

# What is a word?

A word is a valid utf-8 encoded string that is not empty, and doesn't contain internal white spaces (that would be a *sentence*) - we gracefully take care of leading and trailing white spaces for you, both at inserting & querying through the web API.

 

# Usage

You need to have [Rust](https://rust-lang.org) installed on your machine. After cloning this repository (see Branches for more information), you can do `cargo run --release` in it's root directory - this will initialize a word set, and start waiting for your queries. For querying and adding words to IWDB's set, you might consider using [Postman](https://www.postman.com/). If you intend to benchmark it, [Apache JMeter](https://jmeter.apache.org/) might be a good choice. Should you ever wish to stop this service, use Ctrl-C, or something similar.

 

# Configuration

We have an `iwdb.toml` where you can edit a few basic things, like which random word generator API should be used to initialize our word set, how many words it should contain, etc.

 

# Branches

Currently, there are two branches. `master` uses Rust's `HashSet` from the standard library as a kind of reference implementation, with the necessary locking mechanisms. `skiplist` uses a lock-free skiplist ([kudzu](https://github.com/withoutboats/kudzu)), that can do insertion through a shared reference, but doesn't support removal by design. This implementation doesn't store inserted words, only their hashes (as `u64`s, calculated by Rust's default hasher) - this should be faster and require less memory. Also, a reference to the skiplist can be sent between threads without locking, which should result in better performance under high contention. The drawback of this approach that this datatype is not exactly *battle-tested*, and compiling it requires nightly Rust - so, this is mostly here due to theoretical interest.

 

# API details

Please note that our word set can only grow over time - removal is not supported. This enables some neat performance tricks in addition to some obvious drawbacks. Currently, this service works with unencrypted http protocol (but it can grab it's initial word set from https APIs, too). The following http requests are supported:

* ### Querying

    * GET to /query with { "word": "your_word" }

* ### Adding

    * POST to /add with { "add": "your_word" }

# Tests

There are some rudimentary test cases in the `assets` folder, which can be imported to Postman. (They use the default configuration.)

# Future developments

Further backing data structures could be evaluated, like [evmap](https://crates.io/crates/evmap). There are quite a lot of missed abstractions, ie. some problems could have been solved in a more idiomatic way via trait implementations for higher generality.


# License

The Unlicense.
