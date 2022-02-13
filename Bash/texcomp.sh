#!/bin/bash

# if I entered the name of a file
if [[ -f $1".tex" ]]
then
	fname=$1
	# compile to pdf and open it
	pdflatex $fname".tex"
	open $fname".pdf"
fi

# if there are no .tex dont enter the loop
shopt -s nullglob
for i in *.tex; do
	pdflatex $i
done
