ALTER TABLE verifications DROP COLUMN deleted_at;

ALTER TABLE verifications DROP CONSTRAINT verifications_user_id_fkey;
ALTER TABLE verifications ADD CONSTRAINT verifications_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

DO $$
    DECLARE
        u record;
    BEGIN
        FOR u IN
            SELECT id FROM users WHERE verified_with_id IS NOT NULL
        LOOP
            DELETE FROM verifications WHERE user_id = u.id AND activated_at IS NULL;
        END LOOP;
    END;
$$;
