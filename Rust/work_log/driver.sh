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
declare ARCH_MONTH=$(date "+%m")

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

if [ "$1" == "n" ]
then
	# for version 0.2.0+
	# log archive system
	case $ARCH_MONTH in
		01)
			ARCH_MONTH="Jan"
			;;
		02)
			ARCH_MONTH="Feb"
			;;
		03)
			ARCH_MONTH="Mar"
			;;
		04)
			ARCH_MONTH="Apr"
			;;
		05)
			ARCH_MONTH="May"
			;;
		06)
			ARCH_MONTH="Jun"
			;;
		07)
			ARCH_MONTH="Jul"
			;;
		08)
			ARCH_MONTH="Aug"
			;;
		09)
			ARCH_MONTH="Sep"
			;;
		10)
			ARCH_MONTH="Oct"
			;;
		11)
			ARCH_MONTH="Nov"
			;;
		12)
			ARCH_MONTH="Dec"
			;;
	esac
	ARCH_MONTH="$ARCH_MONTH$(date "+%Y").txt"
	exit
fi

# all other cases can be sorted by program

./target/release/work_log $1 $DATE
