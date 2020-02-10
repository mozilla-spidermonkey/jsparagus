# Metrics

[![Rust](https://github.com/mozilla-spidermonkey/jsparagus/workflows/Rust/badge.svg)](https://github.com/mozilla-spidermonkey/jsparagus/actions?query=branch%3Amaster)

[![NotImplemented](https://github.com/mozilla-spidermonkey/jsparagus/workflows/Rust/badge.svg)](https://github.com/mozilla-spidermonkey/jsparagus/actions?query=branch%3Amaster)

[![Fuzzbugs days](https://github.com/mozilla-spidermonkey/jsparagus/workflows/Rust/badge.svg)](https://github.com/mozilla-spidermonkey/jsparagus/actions?query=branch%3Amaster)

[![Fuzzbugs count](https://github.com/mozilla-spidermonkey/jsparagus/workflows/Rust/badge.svg)](https://github.com/mozilla-spidermonkey/jsparagus/actions?query=branch%3Amaster)

Unlike other branches in this project, this branch is for collecting metrics from the CI. you will
find these files in the `.results` folder. If this branch gets deleted, don't worry. This branch can be auto-generated from the `.metrics`
folder in the main repository.

## Types of data

These are the types of data that this metrics folder tracks.

1) NotImplemented Count
    * counts number of NotImplemented errors in the codebase. This should slowly rundown to zero
    * Updates on every push to master. See [this action](.github/workflows/ci-counter.yml)

2) Days Since last Fuzzbug
    * tracks the last fuzzbug we saw, if it does not exist, return ∞, otherwise return the last date regardless of state.
    * Updates daily, regardless of push. See [this action](.github/workflows/ci-daily.yml)

3) Fuzzbug open count
    * tracks the number of open fuzzbugs
    * Updates daily, regardless of push. See [this action](.github/workflows/ci-daily.yml)

4) Percentage of tests passing with SmooshMonkey
    * TODO: tracks the number of tests passing without fallback. We should use the try api for this.
    * Updates daily, regardless of push. See [this action](.github/workflows/ci-daily.yml)


5) Percentage of JS compilable with SmooshMonkey
    * TODO: see comment about writing bytes to a file in [this repo](https://github.com/nbp/seqrec)
    * implementation is dependant on how we get the data. We need a robust solution for importing this data.


