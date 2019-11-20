-- Your SQL goes here

DROP TABLE player_factories;


CREATE TABLE player_factories
    (id UUID NOT NULL PRIMARY KEY,
                              user_id UUID REFERENCES users(id) ON DELETE CASCADE,
                                                                          factory_id UUID REFERENCES factories(id) ON DELETE CASCADE,
                                                                                                                             amount integer NOT NULL DEFAULT 0 );