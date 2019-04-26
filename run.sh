#!/bin/bash

echo "Hello, do you want enable console log on file? (y,N)?"
read consoleLog

echo "Git branch? (Master, ...)?"
read branch

FILE="ShiftManager"
EXT=".log"
FXT="$FILE$EXT"

process=$(pgrep turni_manager)
if [ ! -z $process ]; then
	echo "Killing process ${process}"
	kill -9 $process
fi

echo "Cloning..."

if [ -z $branch ]; then
	$branch = "master"
fi
git fetch
git reset --hard origin/$branch
git checkout $branch
git pull
chmod +x run.sh

echo "Preparing..."

if [ ! -d "log" ];  then
	mkdir log
fi

if [ -f $FXT ]; then
	mv ShiftManager.log ./log/ShiftManager-$(date +%F-%T).log
fi

echo "Compiling..."

cargo build --release

echo "Running..."

if [ ! -z $consoleLog ] && [ $consoleLog == 'y' ]; then
	echo "Console Log Enabled!"
	exec ./target/release/turni_manager &> console.log &
else
	echo "Console Log Disabled!"
	exec ./target/release/turni_manager &> /dev/null &
fi

echo "Ok!"
