# Roadmap

## v0.0.0

* Implementing [sqlparser](https://github.com/ballista-compute/sqlparser-rs)
* Implementing [kube-rs](https://github.com/clux/kube-rs)

* We do not care if given contexts and namespaces _really_ exist

* This should work:
```sql
SELECT namespace1, namespace2
FROM context1, context2
WHERE pod.status = 'Running'
```

## v0.0.1

* Validate given namespace(s) and context(s) are really exist

## v0.1.0

* Read SQL input from `.k8sql` file
* Implementing base CLI flags `-f` and `-q`
```bash
$ k8sql -f my_query.k8sql
$ k8sql -q "SELECT namespace FROM context WHERE pod.status = 'Runnig'"
```
