-- Your SQL goes here

CREATE TABLE players_data
    ( energy integer NOT NULL DEFAULT 100,
                                      gold integer NOT NULL DEFAULT 50,
                                                                    exp integer NOT NULL DEFAULT 0,
                                                                                                 user_id UUID REFERENCES users(id) ON DELETE CASCADE,
                                                                                                                                             PRIMARY KEY(user_id) );