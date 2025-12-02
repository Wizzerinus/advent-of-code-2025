#!/usr/bin/env bash

cat input2.txt | awk -M -v 'RS=,' -F '-' -f task.awk