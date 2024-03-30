# Portfolio webpage with an HTMX frontend and Rocket on the backend
A little side project to practice my `rust` + `htmx` tool combo.\
\
To access the admin page, add `/admin` at the end of the URL.
# How to run
It uses `diesel` and `Postgresql` to store posts, so you need it [installed](https://www.postgresql.org/download/) if, for some reason, you want to try my project out.\
Configure the connection string in `.env` file. You may leave everything as it is, but idealy you need a separate postgresql user and a database.\
Create a database:
```
psql -U username -c "CREATE DATABASE dbname;"
```

Then install Diesel CLI tools:
```
cargo install diesel_cli --no-default-features --features postgres
```

And finally run a migration:

```
diesel migration run
```

Now it can be compiled with `cargo run --release`

## Error note
For some reason, windows postgresql installer might not add necessary directories to PATH, so you can get errors like:\
`LINK: fatal error LNK1181: cannot open input file 'libpq.lib'`\
So remember to check if `C:\Program Files\PostgreSQL\<version>\bin` and `C:\Program Files\PostgreSQL\<version>\lib` are added to avoid errors.
