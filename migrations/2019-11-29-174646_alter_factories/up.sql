-- Your SQL goes here

ALTER table factories ADD COLUMN product VARCHAR(50) NOT NULL DEFAULT 'food',
                                                                      ADD COLUMN product_amount INTEGER NOT NULL DEFAULT 10;