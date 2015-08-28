#! /bin/bash

echo "Git repository downloader"
read -p "Please enter repo name: " REPO_NAME
echo "Creating directory: " $REPO_NAME

mkdir $REPO_NAME
cd $REPO_NAME

read -p "Please enter remote url: " REMOTE_URL
read -p "Please enter branch name: " BRANCH

echo "Initialising git, adding remote, and pulling.."
git init
git remote add origin $REMOTE_URL
git pull origin $BRANCH

cd ..
echo "--Done! Listing created directory-- "
ls -a $REPO_NAME
