## Assignment

> 
> "Log output" application currently outputs a timestamp and a random string to the logs.
> 
> Add an endpoint to request the current status (timestamp and string) and an ingress so that you can access it with a browser.
> 
> You can just store the string and timestamp to the memory.

## Solution

### Binaries

- Application was built in Rust.  It listens for a GET request on `localhost:3011`. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.07/app).
- Image was pushed to Docker Hub repo [viksil/log_output:1.07](https://hub.docker.com/r/viksil/log_output/tags?name=1.07).

### Manifests

Port `3011` was mapped all the way through for simplicity.

[**Deployment**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.07/manifests/deployment.yaml)

```
apiVersion: apps/v1
kind: Deployment
metadata:
  name: log-output-depl
spec:
  replicas: 1
  selector:
    matchLabels:
      app: log-output
  template:
    metadata:
      labels:
        app: log-output
    spec:
      containers:
        - name: log-output
          image: viksil/log_output:1.07
```


[**Service**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.07/manifests/service.yaml)

```
apiVersion: v1
kind: Service
metadata:
  name: log-output-service
spec:
  type: ClusterIP
  selector:
    app: log-output
  ports:
    - port: 3011
      protocol: TCP
      targetPort: 3011
```

[**Ingress**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.07/manifests/Ingress.yaml)

```
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: log-output-ingress
spec:
  rules:
  - http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: log-output-service
            port:
              number: 3011
```


### Commands

![Commands for Exercise 1.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.07/Exercise_1.07_commands.png)

### GET Request to Forwarded Port

![GET Request for Exercise 1.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.07/Exercise_1.07_get_request.png)
