-- Your SQL goes here

CREATE TABLE factories (id UUID NOT NULL PRIMARY KEY,
                                                 level integer NOT NULL DEFAULT 1,
                                                                                gold_per_day integer NOT NULL,
                                                                                                     price integer NOT NULL);


CREATE TABLE player_factories
    (user_id UUID REFERENCES users(id) ON DELETE CASCADE,
                                                 factory_id UUID REFERENCES factories(id) ON DELETE CASCADE,
                                                                                                    amount integer NOT NULL DEFAULT 0,
                                                                                                                                    PRIMARY KEY(user_id, factory_id));