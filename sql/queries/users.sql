-- name: get_user :one
SELECT *
FROM users
where email = ?;


-- name: get_users :many
SELECT *
from users;

-- name: get_users_like :many
SELECT * from users where username LIKE ?;


-- name: testy :exec
UPDATE users
set username=?
where username = ?
  and username LIKE ?;
-- sqlc.arg(new_username, search_username)

-- name: reset :exec
DELETE from users;
