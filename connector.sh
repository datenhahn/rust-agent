#!/bin/bash

i=0

while [ 1 ]; do
   i=$((i + 1))
   echo "{ 'data': 'woohoo $i' }"
   sleep 1
done