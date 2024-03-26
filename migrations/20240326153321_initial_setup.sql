--
-- File generated with SQLiteStudio v3.4.4 on Tue Mar 26 11:35:39 2024
--
-- Text encoding used: US-ASCII
--
PRAGMA foreign_keys = off;

-- Table: musics
CREATE TABLE IF NOT EXISTS musics (id varchar (36) PRIMARY KEY, name varchar (255));
PRAGMA foreign_keys = on;

-- Table: files
CREATE TABLE IF NOT EXISTS files (id VARCHAR (36) PRIMARY KEY, location TEXT, size INTEGER, mime varchar (16), name varchar (255));

-- Table: images
CREATE TABLE IF NOT EXISTS images (id varchar (36) PRIMARY KEY, file varchar (36) REFERENCES files (id));

-- Table: album_artists
CREATE TABLE IF NOT EXISTS album_artists (id varchar(36) PRIMARY KEY, album varchar(36) REFERENCES albums (id), artist varchar(36) REFERENCES artists (id));

-- Table: audios
CREATE TABLE IF NOT EXISTS audios (id varchar (36) PRIMARY KEY, file varchar (36) REFERENCES files (id));

-- Table: albums
CREATE TABLE IF NOT EXISTS albums (id varchar (36) PRIMARY KEY, name TEXT, released date, cover varchar(36) REFERENCES images (id));
INSERT INTO albums (id, name, released, cover) VALUES ('c73267c2-f8e5-4280-af56-9cf0e62f3397', 'Key Ingredient', NULL, NULL);
INSERT INTO albums (id, name, released, cover) VALUES ('c0818a03-67b5-44ad-a379-729e8ecafaff', 'Library Of Ruina', NULL, NULL);
INSERT INTO albums (id, name, released, cover) VALUES ('9a6d78b2-9568-428e-8fce-cbf9e06a4b71', 'Library of Ruina (Original Soundtrack)', NULL, NULL);

-- Table: artists
CREATE TABLE IF NOT EXISTS artists (id varchar (36) PRIMARY KEY, name varchar (255), avatar varchar(36) REFERENCES images (id));
INSERT INTO artists (id, name, avatar) VALUES ('9b19c059-de3b-429b-a63e-2f667c5ce3c7', 'Mili', NULL);
INSERT INTO artists (id, name, avatar) VALUES ('635800cf-6e6c-44e1-a17f-5b45938ab235', 'Gawr Gura', NULL);
INSERT INTO artists (id, name, avatar) VALUES ('50ea5eba-df3f-4033-9933-2d4599fe8a9e', 'Mori Calliope', NULL);
INSERT INTO artists (id, name, avatar) VALUES ('61272cf0-a867-4fd2-afdd-41766079b44f', 'KAF', NULL);
INSERT INTO artists (id, name, avatar) VALUES ('961cc408-9dc9-4c39-902a-f60b513041df', 'KIHOW', NULL);

-- Table: music_albums
CREATE TABLE IF NOT EXISTS music_albums (id varchar(36) PRIMARY KEY, music varchar(36) REFERENCES musics (id), album varchar(36) REFERENCES albums (id));

-- Table: music_artists
CREATE TABLE IF NOT EXISTS music_artists (id varchar (36) PRIMARY KEY, music varchar (36) REFERENCES musics (id), artist varchar (36) REFERENCES artists (id), featured Boolean DEFAULT (0));

-- Table: music_audios
CREATE TABLE IF NOT EXISTS music_audios (id varchar (36) PRIMARY KEY, music varchar (36) REFERENCES musics (id), audio varchar (36) REFERENCES audios (id));

-- Table: music_images
CREATE TABLE IF NOT EXISTS music_images (id varchar(36) PRIMARY KEY, image varchar(36) REFERENCES images (id), music varchar(36) REFERENCES musics (id));

