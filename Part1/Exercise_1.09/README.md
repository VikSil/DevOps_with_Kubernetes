## Assignment

> 
> Develop a second application that simply responds with "pong 0" to a GET request and increases a counter (the 0) so that you can see how many requests have been sent. The counter should be in memory so it may reset at some point. Create a new deployment for it and have it share ingress with "Log output" application. Route requests directed '/pingpong' to it.
> 
> In future exercises, this second application will be referred to as "ping-pong application". It will be used with "Log output" application.
> 
> The ping-pong application will need to listen requests on '/pingpong', so you may have to make changes to its code. This can be avoided by configuring the ingress to rewrite the path, but we will leave that as an optional exercise. You can check out https://kubernetes.io/docs/concepts/services-networking/ingress/#the-ingress-resource

## Solution

### Binaries

- Application was built in Rust.  It listens for a GET request on `localhost:3033/pingpong`. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.09/app).
- Image was pushed to Docker Hub repo [viksil/pingpong:1.09](https://hub.docker.com/r/viksil/pingpong/tags?name=1.09).

### Manifests

[**Deployment**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.09/manifests/deployment.yaml)

```
apiVersion: apps/v1
kind: Deployment
metadata:
  name: pingpong-depl
spec:
  replicas: 1
  selector:
    matchLabels:
      app: pingpong
  template:
    metadata:
      labels:
        app: pingpong
    spec:
      containers:
        - name: pingpong
          image: viksil/pingpong:1.09
```


[**Service**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.09/manifests/service.yaml)

```
apiVersion: v1
kind: Service
metadata:
  name: pingpong-service
spec:
  type: ClusterIP
  selector:
    app: pingpong
  ports:
    - port: 3032
      protocol: TCP
      targetPort: 3033
```

[**Ingress**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.09/manifests/Ingress.yaml)

```
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: shared-ingress
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
      - path: /
        pathType: Prefix
        backend:
          service:
            name: log-output-service
            port:
              number: 3011
```

### Commands

- This exercise used the same cluster that was created for [exercise 1.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.07/Exercise_1.07_commands.png). Loadbalancer HTTP port `80` is exposed to outside requests on `localhost:3011`.

- Ingress to `log-output` service was removed to prevent a clash.

![Commands for Exercise 1.09](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.09/Exercise_1.09_commands.png)

![Shared Ingress for Exercise 1.09](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.09/Exercise_1.09_ingress.png)

### GET Request to root

![GET Request to root](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.09/Exercise_1.09_log_output.png)


### GET Request to /pingpong

![GET Request to /pingpong](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.09/Exercise_1.09_pingpong.png)