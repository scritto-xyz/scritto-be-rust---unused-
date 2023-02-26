CREATE TABLE railway.user
(
    id         INT AUTO_INCREMENT,
    first_name VARCHAR(300)                        NOT NULL,
    last_name  VARCHAR(300)                        NOT NULL,
    email      VARCHAR(300)                        NOT NULL,
    password   VARCHAR(1000)                       NOT NULL,
    country    VARCHAR(200)                        NOT NULL,
    state      VARCHAR(200)                        NOT NULL,
    city       VARCHAR(200)                        NOT NULL,
    user_type  ENUM('Artist', 'Client')            NOT NULL,
    created_ts TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_ts TIMESTAMP DEFAULT CURRENT_TIMESTAMP NULL,
    CONSTRAINT user_pk
        PRIMARY KEY (id)
);

CREATE UNIQUE INDEX user_email_uindex
    ON railway.user (email);

