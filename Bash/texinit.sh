#!/bin/bash

declare DATE=$(date +"%D") 1> /dev/null

touch $1".tex"

echo "\documentclass{article}

\title{$1}
\date{$DATE}
\author{Evan Drake}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

\begin{document}
	\maketitle
\end{document}
" > $1".tex"
