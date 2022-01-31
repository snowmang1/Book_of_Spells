#!/bin/bash

pandoc $1".md" -o $1".pdf"
open $1".pdf"
