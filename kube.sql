SELECT kube-system, default
FROM minikube, kind-kind
WHERE pod.status.phase = 'Running' OR deployment.metadata.name = 'vault-agent-injector'