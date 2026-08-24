-- Changing the is_group to chat_type

CREATE TYPE chat_type_enum AS ENUM ('private', 'group', 'channel');

ALTER TABLE chats ADD COLUMN chat_type chat_type_enum NOT NULL DEFAULT 'private';

UPDATE chats
SET chat_type = CASE
    WHEN is_group = true THEN 'group'::chat_type_enum
    ELSE 'private'::chat_type_enum
END;

ALTER TABLE chats DROP COLUMN is_group;

-- Changing the role type

CREATE TYPE member_role_enum AS ENUM ('admin', 'moderator', 'member');

ALTER TABLE chat_members ALTER COLUMN role DROP DEFAULT;

ALTER TABLE chat_members ALTER COLUMN role TYPE member_role_enum USING role::member_role_enum;

ALTER TABLE chat_members ALTER COLUMN role SET DEFAULT 'member'::member_role_enum;