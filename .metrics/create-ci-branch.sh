#!/usr/bin/env bash

set -ue # its like javascript, everything is allowed unless you prevent it.
shopt -s extglob

topdir=$(git rev-parse --show-toplevel)

cd $topdir

if [ `git branch --list $ci_branch` ]
then
  echo "Branch exists"
else
  git checkout -b $ci_branch

  # clear out the repostory
  git rm -r !(.metrics|.git|tmp)

  cp .metrics/generated_README.md README.md

  cd $topdir
  git add .
  git commit -m"Initial commit for results branch"

  # scripts needed to populated. Should be self contained with cleanup of extra files
  cd .metrics
  ./populate.sh

  cd $topdir
  git add .
  git commit -m"Inital run of Populate scripts"
fi
