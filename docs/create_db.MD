# Create DB and Tables

```sh
createdb slac_app --template=template0

~/D/C/g/f/slac (main)> sqlx migrate add initial
Creating migrations/20250410001152_initial.sql
~/D/C/g/f/slac (main)> sqlx migrate run
Applied 20250408135615/migrate initial (40.972292ms)

```

```sh
~/D/C/g/f/slac (main)> psql -U db_manager -d slac_app
psql (14.17 (Homebrew))
Type "help" for help.

slac_app=> \dt
Did not find any relations.
slac_app=> \conninfo
You are connected to database "slac_app" as user "db_manager" via socket in "/tmp" at port "5432".
slac_app=> \dt
               List of relations
 Schema |       Name       | Type  |   Owner
--------+------------------+-------+------------
 public | _sqlx_migrations | table | db_manager
 public | channels         | table | db_manager
 public | messages         | table | db_manager
 public | threads          | table | db_manager
 public | users            | table | db_manager
(5 rows)

slac_app=> select * from users;
 id | username | password | created_at | updated_at
----+----------+----------+------------+------------
(0 rows)

slac_app=> \d users;
                                       Table "public.users"
   Column   |           Type           | Collation | Nullable |              Default
------------+--------------------------+-----------+----------+-----------------------------------
 id         | bigint                   |           | not null | nextval('users_id_seq'::regclass)
 username   | character varying(128)   |           | not null |
 password   | character varying(64)    |           | not null |
 created_at | timestamp with time zone |           |          | CURRENT_TIMESTAMP
 updated_at | timestamp with time zone |           |          | CURRENT_TIMESTAMP
Indexes:
    "users_pkey" PRIMARY KEY, btree (id)
    "users_username_key" UNIQUE CONSTRAINT, btree (username)

```
