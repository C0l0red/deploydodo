-- Seed ssh_keys for tests
-- Enum values are lowercase per schema (see migration 0004)

INSERT INTO ssh_keys (name, username, password, private_key, public_key, auth_type, created_at)
VALUES
  (
    'pw-fixture',
    'ada',
    'hunter2',
    NULL,
    NULL,
    'password',
    NOW()
  ),
  (
    'kp-fixture',
    'grace',
    NULL,
    '-----BEGIN OPENSSH PRIVATE KEY-----\n-----END OPENSSH PRIVATE KEY-----',
    'ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQCs',
    'keypair',
    NOW()
  );
