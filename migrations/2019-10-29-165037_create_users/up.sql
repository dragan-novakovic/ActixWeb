CREATE TABLE users (id UUID NOT NULL PRIMARY KEY,
                                             email VARCHAR(80) NOT NULL UNIQUE,
                                                                        username VARCHAR(50) NOT NULL UNIQUE,
                                                                                                      password VARCHAR(50) NOT NULL,
                                                                                                                           created_on TIMESTAMP NOT NULL);