#! /bin/bash
#Pulls the git develop branch of all subdirectories in the current directory

DIRS=$( ls -d */)

read -p "Branch name to pull for all subfolders: " BRANCH_NAME

for dir in $DIRS
do
git -C $dir pull origin $BRANCH_NAME
done
