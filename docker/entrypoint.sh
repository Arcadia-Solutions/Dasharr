#!/bin/bash

PGDATA="/usr/local/pgsql/data"
if [ ! -d "$PGDATA" ] || [ -z "$(ls -A "$PGDATA")" ]; then
    /etc/init.d/postgresql start
    SU_COMMAND="psql -c 'CREATE DATABASE $POSTGRES_DB;' && \
        psql -c \"CREATE USER $POSTGRES_USER WITH ENCRYPTED PASSWORD '$POSTGRES_PASSWORD';\" && \
        psql -d $POSTGRES_DB -c 'GRANT ALL PRIVILEGES ON DATABASE $POSTGRES_DB TO $POSTGRES_USER;' && \
        psql -d $POSTGRES_DB -f /tmp/initdb.sql && \
        psql -d $POSTGRES_DB -c 'GRANT ALL ON ALL TABLES IN SCHEMA public TO $POSTGRES_USER;' && \
        psql -d $POSTGRES_DB -c 'GRANT ALL ON ALL SEQUENCES IN SCHEMA public TO $POSTGRES_USER;' && \
        psql -d $POSTGRES_DB -c 'GRANT ALL ON ALL FUNCTIONS IN SCHEMA public TO $POSTGRES_USER;' && \
        psql -d $POSTGRES_DB -c 'ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO $POSTGRES_USER;' && \
        psql -d $POSTGRES_DB -c 'ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO $POSTGRES_USER;' && \
        psql -c \"ALTER SYSTEM SET listen_addresses = '*';\""
    su postgres -c "$SU_COMMAND"
    # /etc/init.d/postgresql stop
fi
