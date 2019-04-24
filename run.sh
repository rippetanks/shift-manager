#!/bin/bash

FILE="ShiftManager"
EXT=".log"
FXT="$FILE$EXT"

process=$(pgrep turni_manager)
if [ ! -z $process ]; then
	echo "Killing process ${process}"
	kill -9 $process
fi

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

if [ $# == 1 ] && [ $1 == "CONSOLE_LOG" ]; then
	echo "Console Log Enabled!"
	exec ./target/release/turni_manager > console.log &
else
	echo "Console Log Disabled!"
	exec ./target/release/turni_manager
fi

echo "Ok!"
