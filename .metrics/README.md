# Metrics

This is the metrics directory. It follows the evolution of the repository separately from the
repostory. You can find the actual metrics in the
[`ci-results`](https://github.com/mozilla-spidermonkey/jsparagus/tree/ci-results) branch of the jsparagus project. This branch is automatically generated using the `create-ci-branch.sh` script found in this directory. If there are issues with your fork, you can remove the `ci-results` branch, and the ci will automatically rerun the `create-ci-branch` script to reset it. Do not push manula data to this repository, it will be lost.

This is an area that you normally shouldn't need to touch. The files here are automatically invoked
by the build system, and populates the `ci-results` branch. The idea here is that metrics will be frequently updated, and to reduce the noise in pull requests, this is kept separate from our normal work flow.

## Making your own metrics
If you need to add a new metric there are two things you need to ensure.

The first is that you follow the pattern established in this repository. You can find template files in this folder under `/templates`. There you will find a template for badges, a template for a python script, and any others that might have been added.

The second, is that you do not use data that can not be automatically recovered. We cannot rely on the `ci-results` branch always being present, therefore anything that you write must be recoverable on its own, either by relying on external APIs or through some other mechanism.

Please update this README if you make any changes.

## Types of CI Actions
These actions are all found in the `.github/workflows` directory

1) `Rust.yml` - Run on Pull Request
* runs every time there is a push to master, use for any metrics that are development related. Examples include linting, testing, etc.
2) `ci-push.yml` - Run on Push to `master`
* runs on self contained metrics. An example is the number of `NotImplemented` errors in the codebase. This does not depend on anything external
3) `ci-daily.yml` - Run Daily
* a cron task that runs daily. Useful for metrics that need daily updates
4) `ci-issue.yml` - Run on issue open
* runs each time an issue is opened. Good for tracking types of issues.


## Types of data

These are the types of data that this metrics folder tracks.

1) Rust Passing
    * Ensures our internal tests are passing
    * Updates on every pull request to master. See [this action](.github/workflows/Rust.yml)

2) NotImplemented Count
    * counts number of NotImplemented errors in the codebase. This should slowly rundown to zero
    * Updates on every push to master. See [this action](.github/workflows/ci-counter.yml)

3) Days Since last Fuzzbug
    * tracks the last fuzzbug we saw, if it does not exist, return ∞, otherwise return the last date regardless of state.
    * Updates daily, regardless of push. See [this action](.github/workflows/ci-daily.yml)

4) Fuzzbug open count
    * tracks the number of open fuzzbugs
    * Updates on issue open. See [this action](.github/workflows/ci-issue.yml)

5) Percentage of tests passing with SmooshMonkey
    * TODO: tracks the number of tests passing without fallback. We should use the try api for this.
    * Updates daily, regardless of push. See [this action](.github/workflows/ci-daily.yml)

6) Percentage of JS compilable with SmooshMonkey
    * TODO: see comment about writing bytes to a file in [this repo](https://github.com/nbp/seqrec)
    * implementation is dependant on how we get the data. We need a robust solution for importing this data.

