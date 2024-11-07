#!/bin/bash

set -e

surreal start --user root --pass root --bind 0.0.0.0:8100 memory &

sleep 3

for file in /root/migrate/table/*.srql; do
  surreal import -u root -p root --ns mubdel --db mubdel \
    --endpoint http://localhost:8100 $file
done

for file in /root/migrate/fn/*.srql; do
  surreal import -u root -p root --ns mubdel --db mubdel \
    --endpoint http://localhost:8100 $file
done

wait -n
