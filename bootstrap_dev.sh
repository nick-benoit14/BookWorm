# TODO configure pg

cat db/setup_dev.sql | psql
cat db/seeds.sql | psql -d book_worm_dev
