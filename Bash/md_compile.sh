#!/bin/bash

if [[ -f $1".md" ]]
then
	fname=$1
	# compile with pandoc
	pandoc $fname".md" -o $fname".pdf"
	open $fname".pdf"
fi
