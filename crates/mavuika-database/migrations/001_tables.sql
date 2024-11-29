CREATE TABLE t_sdk_account (
    uid int primary key generated always as identity,
    token varchar(64) NOT NULL,
    username varchar(40) NOT NULL,
    password varchar(256) NOT NULL,
    UNIQUE(username)
);

CREATE TABLE t_combo_token (
    account_uid varchar(32) primary key,
    token varchar(64) NOT NULL,
    device_id varchar(128) NOT NULL,
    UNIQUE(account_uid, device_id)
);

CREATE TABLE t_user_uid (
	account_uid varchar(32) primary key,
	uid int generated always as identity
);

CREATE TABLE t_player_data (
	uid int primary key,
	data jsonb NOT NULL
);
