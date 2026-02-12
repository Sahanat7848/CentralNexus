ALTER TABLE brawlers ADD COLUMN tag VARCHAR(4) NOT NULL DEFAULT '0000';

-- Set random tags for existing users
UPDATE brawlers SET tag = LPAD(FLOOR(RANDOM() * 10000)::text, 4, '0');

-- Create friendships table
CREATE TABLE friendships (
    brawler_id INT NOT NULL REFERENCES brawlers(id) ON DELETE CASCADE,
    friend_id INT NOT NULL REFERENCES brawlers(id) ON DELETE CASCADE,
    status VARCHAR(20) NOT NULL DEFAULT 'accepted', -- starting with 'accepted' for simplicity if the user just wants "add friend" or maybe 'pending' for request. User said "add friend", usually implies request system or direct add. Let's go with 'accepted' if they want it simple, or 'pending' for a proper system. I'll use 'pending' as it's more standard.
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (brawler_id, friend_id)
);
