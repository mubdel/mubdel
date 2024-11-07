#!/bin/bash

set -e

surreal start --user root --pass root file:/root/mubdel.db &
SURREAL_PID=$!

sleep 3

for file in /root/migrate/table/*.srql; do
  surreal import -u root -p root --ns mubdel --db mubdel \
    --endpoint http://localhost:8000 $file
done

for file in /root/migrate/fn/*.srql; do
  surreal import -u root -p root --ns mubdel --db mubdel \
    --endpoint http://localhost:8000 $file
done

kill $SURREAL_PID
