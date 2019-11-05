#!/bin/sh

go build -o imgresize -ldflags "-s -w" && time ./imgresize -src /home/hacklog/Videos/test/thumb/163P -size 128

