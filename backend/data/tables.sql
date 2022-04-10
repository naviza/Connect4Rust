DROP TABLE IF EXISTS Games;
DROP TABLE IF Exists Players;

CREATE TABLE Games (
    gameID INTEGER NOT NULL PRIMARY KEY,
    player1ID INTEGER NOT NULL,
    player2ID INTEGER NOT NULL,
    gameType TEXT NOT NULL,
    won INTEGER NOT NULL CHECK(won == player1ID or won == player2ID),
    gameDateTime INTEGER NOT NULL,
    FOREIGN KEY(player1ID) REFERENCES Players(playerID),
    FOREIGN KEY(player2ID) REFERENCES Players(playerID)
    -- FOREIGN KEY(won) REFERENCES Players(playerID)
);

CREATE TABLE Players (
    playerID INTEGER NOT NULL PRIMARY KEY,
    playerName TEXT NOT NULL UNIQUE,
    pswd TEXT NOT NULL
);

INSERT INTO Players (playerName, pswd)
VALUES ("Anuj","Anuj"), ("Richmond","Richmond"), ("Ralph","Ralph"), ("Taranjot", "Taranjot");

INSERT INTO Games (player1ID, player2ID, gameType, won, gameDateTime)
VALUES (1, 4, "C4", 1, CURRENT_TIMESTAMP),
(2, 4, "C4", 2, CURRENT_TIMESTAMP),
(1, 2, "C4", 1, CURRENT_TIMESTAMP);



