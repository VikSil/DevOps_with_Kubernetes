# Assignment

> The project could really use logging.
> 
> Add request logging so that you can monitor every *todo* that is sent to the backend.
> 
> Set the limit of 140 characters for todos into the backend as well. Use Postman or curl to test that too long todos are blocked by the backend and you can see the non-allowed messages in your Grafana.

# Solution

## Binaries

### Todo backend

- Application was built in Rust, using `axum` crate. Additional logging was introduced for this exercise. A check was implemented for the length of the received TODO item on the `POST` endpoint. If the TODO is longer than 140 characters, the backend responds with HTTP code `400`:`Invalid request data`. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.10/app/todo_backend).
- Image was pushed to Docker Hub repo [viksil/todo_backend:2.10](https://hub.docker.com/r/viksil/todo_backend/tags?name=2.10).

## Manifests

### Todo backend

Manifest from [Exercise 2.08](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08) was used with only the image version changed:

```
    spec:
      containers:
        - name: todo-backend
          image: viksil/todo_backend:2.10
```

### Prometheus and Grafana

```
prometheus:
  prometheusSpec:
    serviceMonitorSelectorNilUsesHelmValues: false
    serviceMonitorSelector: {}
    serviceMonitorNamespaceSelector: {}

grafana:
  sidecar:
    datasources:
      defaultDatasourceEnabled: true
  additionalDataSources:
    - name: Loki
      type: loki
      url: http://loki-loki-distributed-query-frontend.loki-namespace:3100
```

### Promtail

```
config:
  serverPort: 8080
  clients:
    - url: http://loki-loki-distributed-gateway.loki-namespace/loki/api/v1/push
```

## Commands

![Commands for Exercise 2.10](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.10/Exercise_2.10_commands.png)

![Commands for Exercise 2.10](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.10/Exercise_2.10_commands1.png)

![Commands for Exercise 2.10](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.10/Exercise_2.10_commands2.png)

![Commands for Exercise 2.10](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.10/Exercise_2.10_commands3.png)

![Commands for Exercise 2.10](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.10/Exercise_2.10_commands4.png)

## Loki logs

![Loki log output to Grafana](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.10/Exercise_2.10_grafana.png)

