#! /bin/bash

run_silent()
{
	${1} > /dev/null 2>&1
}

echo -ne '#		[0%]\r'
run_silent 'brew update'
echo -ne '####		[33%]\r'
run_silent 'brew upgrade'
echo -ne '########	[66%]\r'
run_silent 'brew cleanup --prune=20'
echo -ne '				\r'
echo -ne '##############[100%] \n'
