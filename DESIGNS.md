# Design Templates
(*): Currently implemented
## 1
```sql
SELECT namespace.context
FROM pods p
WHERE p.status = 'Running'
```
## 2
```sql
SELECT namespace1, namespace2
FROM context1, context2, context3
WHERE pod.status = 'Running'
```
### 3 (*)
```sql
SELECT namespace1, namespace2
FROM context1, context2, context3
WHERE pod.status.phase = 'Running' AND deployment.metadata.name = 'my-awesome-deployment'
```
## 4
```sql
SELECT pods, deployments
FROM namespace1, namespace2
CLUSTER BY context1, context2, context3
WHERE pod.status = 'Running'
```
## 5
```sql
SELECT *
FROM pods
WHERE pod.status = 'Running' IN (context1, context2)
```