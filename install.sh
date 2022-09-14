#!/usr/bin/bash

cargo build --release
mv target/release/wallrus .

if [ -e /usr/bin/feh ]
then
	echo "Requirement satisfied: feh"
else
	echo "Installation failed! cannot find feh, is it installed? [https://archlinux.org/packages/extra/x86_64/feh/]"
	exit
fi

if [ -e /usr/bin/crontab ]
then 
	echo "Requirement satisfied: crontab"
else
	echo "Installation failed! cannot find crontab, is it installed? [https://archlinux.org/packages/core/x86_64/cronie/]"
	exit
fi

echo "Installation was successful! Try ./wallrus set --query=batman"

