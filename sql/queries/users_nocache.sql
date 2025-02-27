-- name: create_user :one
INSERT into users ( username,
                    email,
                    pass_hash,
                    salt)
VALUES (?, ?, ?, ? ) RETURNING id;