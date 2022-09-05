# human-adviser-study

code for the human-adviser study

## study_backend

the backend server accepts http requests from the frontend and forwards them as inserts into a mySQL database. make sure that a .env file exists in the workspace directory that looks like this:

```
DATABASE_URL=...
DATABASE_PORT=...
DATABASE_USER=...
DATABASE_PASS=...
DATABASE_NAME=...
```

## study_frontend

a WASM app that runs in the browser. this is the interactive study itself.
