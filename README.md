# rust_sqlc_test
A rust project designed to test rust templates for sqlc-gen-template

This is a very simple rust project which uses sqlc along with sqlc-gen-template to generate rusqlite functions. The purpose is to test sqlc-gen-template along with rust templates. 
To use this template 

- install sqlc [sqlc install instructions ](https://docs.sqlc.dev/en/stable/overview/install.html)
- install sqlc-gen-template with support for the included template
  - `git clone https://github.com/fdietze/sqlc-gen-from-template.git`
  - ensure you have the go compiler installed [go install instructions](https://go.dev/doc/install)
  - navigate to the created folder and run `go install`  
- clone the repo `git clone https://github.com/ReenigneCA/rust_sqlc_test.git`
- from the root of the created directory run `cargo run`.

  This will build everything and run the test program which should create an sqlite3 database and print some json to the terminal. From there you can edit the template and/or change your copy of sqlc-gen-template and re-run the program to confirm that everything still works. 
