#! /bin/bash

# This is the bash driver for rust code that will file and archive my
# work times according to the date and time.
# possible flags: "action" = [char flag]
# login = i
# logout = o
# new_month = n
# print = p
# help = h

declare DATE=$(date +"%D %T") 1> /dev/null

if [ "$1" == "h" ] || [ -z "$1" ]
then
	# misuse / help case
	echo 	"
		login = i 	\n
		logout = o 	\n
		new_month = n 	\n
		print = p 	\n
		help = h	\n
		"
	exit
fi

if [ "$1" == "p" ]
then
	# printing log
	cat log.txt
	exit
fi

# all other cases can be sorted by program

./target/release/work_log $1 $DATE
