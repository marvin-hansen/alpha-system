

## Test DB Postgres connection:

```shell
kubectl port-forward svc/postgres-rw 5432:5432
```
Connect to postgres cluster:

When enableSuperuserAccess is set to true:
User: postgres
Password: postgres

Otherwise, a custom user and password is required.

```shell
psql -h 127.0.0.1 -p 5432 -U postgres -d postgres
```