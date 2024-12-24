CREATE TABLE image (
    id integer PRIMARY KEY,
    url varchar(140),
	time bigint
);

INSERT INTO image (id, url, time)
VALUES (1, 'https://fastly.picsum.photos/id/633/1200/1200.jpg?hmac=w3wSzGHuyT-aMKInisjPvciLC7gIgyXaBAeU7nzo-c4', 0);
