# Preparation
* Install PostgreSQL database
* Create a database `blog`
* Create user `bloguser` with password `bloguser` 
* Grant all privileges on `blog` database to `bloguser`


## Apply migrations
```bash
diesel migration run --database-url='postgres://bloguser:bloguser@localhost/blog'
```

## Update all migrations
```bash
diesel migration redo --database-url='postgres://bloguser:bloguser@localhost/blog'
```
