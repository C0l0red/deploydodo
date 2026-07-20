-- Seed users for tests
-- Note: account_type enum values must be lowercase per schema

INSERT INTO users (name, email, password_hash, account_type, created_at)
VALUES
  (
    'Ada Lovelace',
    'ada@example.com',
    -- syntactically valid Argon2id hash used in tests
    '$argon2id$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$c29tZWNoZWNrc3Vt',
    'admin',
    NOW()
  ),
  (
    'Grace Hopper',
    'grace@example.com',
    '$argon2id$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$c29tZWNoZWNrc3Vt',
    'member',
    NOW()
  );
