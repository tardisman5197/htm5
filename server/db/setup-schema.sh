#!/bin/sh
psql -U admin -c "\
CREATE DATABASE chat; "
psql -U admin -c "\
GRANT ALL PRIVILEGES ON DATABASE chat TO admin;"
psql -U admin -c "\
CREATE TABLE messages ( \
    id SERIAL NOT NULL PRIMARY KEY, \
    sender varchar(80), \
    receiver varchar(80), \
    timesent timestamp, \
    timeread timestamp, \
    message varchar(200), \
    artist varchar(80), \
    song varchar(80), \
    link varchar(80) \
);"
psql -U admin -c "\
    CREATE INDEX sender_index ON messages (sender);
    CREATE INDEX receiver_index ON messages (receiver);
";