# IWDB

 

This service provides checks whether a word posted to it exists in a set of words.

 

# What is a word?

A word is a valid utf-8 encoded string that is not empty, and doesn't contain internal white spaces (that would be a *sentence*) - we gracefully take care of starting and trailing white spaces for you, both at inserting & querying.

 

# Usage

You need to have [Rust](https://rust-lang.org) installed on your machine. After cloning this repository (see Branches for more information), you can do `cargo run --release` in it's root directory - this will initialize a word set, and start waiting for your queries. For querying and adding words to IWDB's set, you might consider using [Postman](https://www.postman.com/). If you intend to benchmark it, [Apache JMeter](https://jmeter.apache.org/) might be a good choice. Should you ever wish to stop this service, use Ctrl-C, or something similar.

 

# Configuration

We have an `iwdb.toml` where you can edit a few basic things, like which random word generator API should be used to initialize our word set, how many words it should contain, etc.

 

# Branches (TODO)

You can experiment with multiple backing set types. The MVP branch uses Rust's `HashSet` from the standard library as a kind of reference implementation, the kuzdu branch uses a lock-free skipmap that doesn't support removal by it's design, and the evmap branch uses an eventually consistent hashmap, where the tradeoff is eventual consistency and doubled memory usage.

 

# API details

Please note that our word set can only grow over time - removal is not supported. This enables some neat performance tricks in addition to some obvious drawbacks. Currently, this service works with unencrypted http protocol (but it can grab it's initial word set from https APIs, too). The following http requests are supported:

* ### Querying

    * GET to /query with { "word": "your_word" }

* ### Adding

    * POST to /add with { "add": "your_word"}

# Future developments

There are of course still plenty of ways to do this better. Since being able to do iteration, draining, etc. on our word set is not a requirement, storing actual values is not necessary, only hashes are needed. This would have a two-fold benefit: smaller and known size, allowing for tricks like arena allocation.

# License

The Unlicense. Leave me alone, *in the legal sense*.
