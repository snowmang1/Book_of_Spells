#! /bin/bash

# This is the bash driver for rust code that will file and archive my
# work times according to the date and time.
# possible flags: "action" = [char flag]
# syntax for input: [flag] [date] [time]

declare DATE=$(date +"%D %T") 1> /dev/null
declare ARCH_MONTH=$(date "+%m")

if [[ $1 =~ ^h(elp)?$ ]] || [[ -z $1 ]]
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

if [[ $1 =~ ^p(rint)?$ ]]
then
	# printing log
	cat log.txt
	exit
fi

if [[ $1 =~ ^n(ew)?*$ ]]
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
	if [[ -d log_archive ]]
	then
		# insert log into archive
		mv log.txt log_archive/"$ARCH_MONTH"
		touch log.txt
	else
		# create archive and insert log
		mkdir log_archive
		mv log.txt log_archive/"$ARCH_MONTH"
		touch log.txt
	fi
fi

# all other cases can be sorted by program

if [[ -f target/release/work_log ]]
then
	# found optimized binary
	./target/release/work_log $1 $DATE
elif [[ -f target/debug/work_log ]]
then
	# found unoptimized
	./target/debug/work_log $1 $DATE

else
	# no binary found
	echo "ERROR NO BINARY FOUND"

fi
