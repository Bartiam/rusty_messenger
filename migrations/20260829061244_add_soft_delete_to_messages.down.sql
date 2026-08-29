DROP TRIGGER IF EXISTS trigger_messages_updated_at ON messages;
DROP FUNCTION IF EXISTS update_updated_at_column();

DROP INDEX IF EXISTS idx_messages_chat_active;
DROP INDEX IF EXISTS idx_messages_chat_all;

ALTER TABLE messages DROP COLUMN IF EXISTS deleted_at;
ALTER TABLE messages DROP COLUMN IF EXISTS updated_at;