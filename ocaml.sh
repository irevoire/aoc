#!/bin/bash

# This script provide two functions generate the base file needed to setup a new day
# aoc_dune_create dayx: create a new day from the day you provide in parameter
# aoc_dune_init: create a new day from the current directory

base_dune_file="(executables
	(names part1)
	(public_names part1)
	(modes exe)
)"

base_ocaml_file="open Scanf
exception Error of string

let file = Sys.argv.(1)
let scanner = Scanning.from_file file

let res = 42;;
Printf.printf \"The res is %d\\\n\" res;;"

function aoc_dune_create () {
	mkdir -p $1
	cd $1
	aoc_dune_init
}

function aoc_dune_init () {
	directory=${PWD##*/}
	echo Setting up $directory
	mkdir -p src/bin

	touch $directory.opam
	echo '(lang dune 1.2)' > dune-project
	echo $base_dune_file > src/bin/dune
	echo $base_ocaml_file > src/bin/part1.ml
}
