create table if not exists languages (
    code VARCHAR(8) PRIMARY KEY,
    isDefaultLanguage BIT DEFAULT 0
);

create table if not exists pages (
    id VARCHAR(64) PRIMARY KEY
);

-- create table if not exists literals (
--     pageId VARCHAR(64) NOT NULL,
--     `key` VARCHAR(64) NOT NULL,
--     `text` TEXT,
--     PRIMARY KEY (pageId, `key`),
--     FOREIGN KEY (pageId) REFERENCES pages(id)
-- );

create table if not exists literals (
    pageId VARCHAR(64),
    `key` VARCHAR(64),
    `language` VARCHAR(8),
    `text` TEXT,
    reviewed BIT,
    PRIMARY KEY (pageId, `key`, `language`),
    FOREIGN KEY (pageId) REFERENCES pages(id),
    FOREIGN KEY (`language`) REFERENCES languages(code)
);