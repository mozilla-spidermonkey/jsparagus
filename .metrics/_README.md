# Metrics

[![Rust](https://github.com/mozilla-spidermonkey/jsparagus/workflows/Rust/badge.svg)](https://github.com/mozilla-spidermonkey/jsparagus/actions?query=branch%3Amaster)

Unlike other branches in this project, this branch is for collecting metrics from the CI. you will
find these files in the `.results` folder. If this branch gets deleted, don't worry. This branch can be auto-generated from the `.metrics`
folder in the main repository.

The `ci-counter` workflow in the main repository relies on there being a `ci-results` branch. It
will create it if it doesn't exist
