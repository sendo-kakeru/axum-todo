#!/bin/bash
set -e

echo "⚠️ Dropping all tables in 'dev' database..."

# PostgreSQL のメタ情報から全テーブルを DROP 文として生成し、実行
docker-compose exec dev-postgres psql -U dev -d dev -Atc "
DO \$\$
DECLARE
    r RECORD;
BEGIN
    FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP
        EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
    END LOOP;
END
\$\$;
"

echo "✅ Tables dropped."

echo "🔄 Running migrations..."
sqlx migrate run
