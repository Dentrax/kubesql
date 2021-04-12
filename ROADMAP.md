# Roadmap

## v0.1.0

* Implement [sqlparser](https://github.com/ballista-compute/sqlparser-rs)
* Implement [kube-rs](https://github.com/clux/kube-rs)
* Implement [prettytable-rs](https://github.com/phsym/prettytable-rs)
* Validate if given namespace(s) and context(s) are really exist
* Implement base CLI flags `-f, --file` and `-q, --query`
```bash
$ k8sql --file kube.sql
$ k8sql --query "SELECT namespace FROM context WHERE pod.status.phase = 'Running'"
```

* This should work:
```sql
SELECT namespace1, namespace2
FROM context1, context2
WHERE pod.status.phase = 'Running' AND deployment.metadata.name = 'my-awesome-deployment'
```

* It should print:
```bash
+----------------+---------------------------------+---------------------------------+
| KIND / CONTEXT | context1                        | context2                        |
+----------------+---------------------------------+---------------------------------+
| pod            | +-------+-------+               | +-------+-------+               |
|                | | ns1   | ns2   |               | | ns1   | ns2   |               |
|                | +-------+-------+               | +-------+-------+               |
|                | | pod-1 | pod-1 |               | | pod-2 | pod-2 |               |
|                | +-------+-------+               | +-------+-------+               |
+----------------+---------------------------------+---------------------------------+
| deployment     | +--------------+--------------+ | +--------------+--------------+ |
|                | | ns1          | ns2          | | | ns1          | ns2          | |
|                | +--------------+--------------+ | +--------------+--------------+ |
|                | | deployment-1 | deployment-1 | | | deployment-2 | deployment-2 | |
|                | +--------------+--------------+ | +--------------+--------------+ |
+----------------+---------------------------------+---------------------------------+
```

* Print conditions:
1. Printer should not insert `pod` row if does not given in `WHERE` statement
2. Printer should not insert `deployment` row if does not given in `WHERE` statement

## v0.2.0?

* Why even are the Humankind on Earth needs to make API calls to K8s using SQL instead of official [kubectl](https://github.com/kubernetes/kubectl) tool?
* Let's try not to create noisy To-Do list on README to do not those in the near future
* I don't even think there will just another version. Are you?