# Metrics

This is the metrics directory. It follows the evolution of the repository separately from the
repostory. You can find the actual metrics in the
[`ci-results`](https://github.com/mozilla-spidermonkey/jsparagus/tree/ci-results) branch of the jsparagus project.

This is an area that you normally shouldn't need to touch. The files here are automatically invoked
by the build system, and the `ci-results` branch is populated via the workflow action. The idea here
is that metrics will be frequently updated, and to reduce the noise in pull requests, this is kept
separate from our normal work flow.

If you need to add a new metric, you can do so by writing a python script which manipulates the data
that you want so that it outputs as a JSON file. Then, to have this added to the webpage, include
the directory in the list of ... . To test it locally, run `make count`.
