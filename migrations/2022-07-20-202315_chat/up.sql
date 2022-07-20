-- Your SQL goes here
CREATE TABLE IF NOT EXISTS chats (
    chat_id BIGSERIAL,
    PRIMARY KEY(chat_id),
    room INT NOT NULL,
    creator VARCHAR(255),
    displayed_name VARCHAR(255),
    short_name VARCHAR(100),
    chat_type VARCHAR(10),
    avatar TEXT,
    users TEXT,
    open BOOLEAN,
    description TEXT
);

CREATE TABLE IF NOT EXISTS messages (
    msg_id BIGSERIAL PRIMARY KEY,
    content TEXT,
    author VARCHAR(255),
    time TIMESTAMP WITH TIME ZONE,
    who_received TEXT,
    who_read TEXT,
    chat_id INT,
    CONSTRAINT fk_chat
        FOREIGN KEY(chat_id)
        REFERENCES chats(chat_id)
        ON DELETE CASCADE
);
