# Assignment

> Create the required probes and endpoint for The Project to ensure that it's working and connected to a database.
> 
> Test that the probe indeed works with a version without database access, for example by supplying a wrong database URL or credentials.

# Solution

## Binaries

### Todo backend

- Application was modified by adding a `\healthz` route and a function to handle requests to this route. The function returns HTTP code `200` or HTTP code `500` depending on the availability of the database. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.02/app/todo_backend).
- Image was pushed to Docker Hub repo [viksil/todo_backend:4.02](https://hub.docker.com/r/viksil/todo_backend/tags?name=4.02).

### Todo app

The same binaries as for [Exercise 2.08](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08) were used.

## Manifests

### Postgres and Todo app

- Unchanged manifests from [Exercise 2.08](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08) were used.

### Todo backend

- Unchanged Service and ConfigMap manifests from [Exercise 2.08](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08) were used.

**[Deployment](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.02/manifests/todo_backend_deployment.yaml)**

```
apiVersion: apps/v1
kind: Deployment
metadata:
  namespace: todo-namespace
  name: todo-backend-depl
spec:
  replicas: 1
  selector:
    matchLabels:
      app: todo-backend
  template:
    metadata:
      labels:
        app: todo-backend
    spec:
      containers:
        - name: todo-backend
          image: viksil/todo_backend:4.02
          imagePullPolicy: Always
          env:
            - name: PORT
              valueFrom:
                configMapKeyRef:
                  name: todo-cfgmap
                  key: port
            - name: DB_URL
              valueFrom:
                secretKeyRef:
                  name: todo-secret
                  key: DB_URL
          readinessProbe:
            initialDelaySeconds: 180 
            periodSeconds: 10
            exec:
              command:
              - sh
              - -c
              - wget --spider -S "http://0.0.0.0:$PORT/healthz" 2>&1 | grep "HTTP/" | awk '{print $2}'
          livenessProbe:
            initialDelaySeconds: 300
            periodSeconds: 5 
            exec:
              command:
              - sh
              - -c
              - wget --spider -S "http://0.0.0.0:$PORT/healthz" 2>&1 | grep "HTTP/" | awk '{print $2}'
```

### [Ingress](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.02/manifests/shared_ingress.yaml)

```
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: log-output-pingpong-ingress
  namespace: pinglog-namespace
spec:
  rules:
  - host: localhost
  - http:
      paths:
      - path: /pingpong
        pathType: Prefix
        backend:
          service:
            name: pingpong-service
            port:
              number: 3032
      - path: /logreader
        pathType: Prefix
        backend:
          service:
            name: log-reader-service
            port:
              number: 3021
      - path: /
        pathType: Prefix
        backend:
          service:
            name: log-output-service
            port:
              number: 3010
```

## Deployment

![Commands for Exercise 4.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.02/Exercise_4.02_commands.png)

![Commands for Exercise 4.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.02/Exercise_4.02_commands2.png)

## Frontend

![Frontend for Exercise 4.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.02/Exercise_4.02_frontend.png)
