# Add Avatar

```sh
~/D/C/g/f/slac (main)> sqlx migrate add add_avatar_to_users
Creating migrations/20250414001737_add_avatar_to_users.sql
```

```sql
-- Add migration script here
ALTER TABLE
    users
ADD
    COLUMN avatar_url VARCHAR(255) NOT NULL DEFAULT '';
```
