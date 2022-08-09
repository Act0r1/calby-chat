-- Your SQL goes here
CREATE TABLE IF NOT EXISTS chat (
    chat_id  BIGSERIAL PRIMARY KEY, 
    -- PRIMARY KEY(chats_id) NOT NULL,
    room BIGSERIAL NOT NULL,
    creator VARCHAR(255) NOT NULL,
    displayed_name VARCHAR(255) NOT NULL,
    short_name VARCHAR(100) NOT NULL,
    chat_type VARCHAR(10) NOT NULL,
    avatar TEXT NOT NULL,
    users TEXT [] NOT NULL,
    open BOOLEAN,
    description TEXT
);

CREATE TABLE IF NOT EXISTS messages (
    msg_id BIGSERIAL PRIMARY KEY,
    chat_id BIGINT NOT NULL,
    content TEXT NOT NULL,
    author VARCHAR(255) NOT NULL,
    time BIGINT NOT NULL,
    who_received TEXT NOT NULL,
    who_read TEXT NOT NULL,
    CONSTRAINT fk_chat
        FOREIGN KEY(chat_id)
            REFERENCES chat(chat_id)
            ON DELETE CASCADE
);
