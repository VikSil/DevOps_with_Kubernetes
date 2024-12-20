## Assignment

> 
> Connect the "Log output" application and "Ping-pong" application. Instead of sharing data via files use HTTP endpoints to respond with the number of pongs. Deprecate all the volume between the two applications for the time being.
> 
> The output will stay the same:
> 
>     2020-03-30T12:15:17.705Z: 8523ecb1-c716-4cb6-a044-b9e83bb98e43.
>     Ping / Pongs: 3

## Solution

### Binaries

**Pingpong**

 - Solution from [Exercise 1.09](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.09) was used with only the output line changed from `pong {count}` to `Ping / Pongs: {count}`.
 - Image was pushed to Docker Hub repo [viksil/pingpong:2.01](https://hub.docker.com/r/viksil/pingpong/tags?name=2.01).

**Log output**

Solution from [Exercise 1.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.07) was used.

**Log reader**

- Application was built in Rust. Application listens to port 3022. When a get request is received, it sends requests to endpoints `http://pingpong-service:3033/pingpong` and `http://log-output-service:3011`. It concatenates the responses and returns the result to the user. For the purposes of the exercise the endpoints are hard-coded, but for a more general usecase, environment variables could be used. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.01/app/log_output_reader).
- Image was pushed to Docker Hub repo [viksil/log_output_reader:2.01](https://hub.docker.com/r/viksil/log_output_reader/tags?name=2.01).

### Manifests

**Pingpong**

[*Deployment*](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.01/manifests/pingpong_pod/deployment.yaml)

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
          image: viksil/pingpong:2.01
```

[*Service*](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.01/manifests/pingpong_pod/service.yaml)

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
    - port: 3033
      protocol: TCP
      targetPort: 3033
```

**Log output**

Unchanged manifests from [Exercise 1.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.07) were used for [Deployment](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.07/manifests/deployment.yaml) and [Service](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.07/manifests/service.yaml), Ingress was ommited.

**Log reader**

[*Deployment*](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.01/manifests/log_reader_pod/deployment.yaml)

```
apiVersion: apps/v1
kind: Deployment
metadata:
  name: log-reader-depl
spec:
  replicas: 1
  selector:
    matchLabels:
      app: log-reader
  template:
    metadata:
      labels:
        app: log-reader
    spec:
      containers:
        - name: log-reader
          image: viksil/log_output_reader:2.01
```


[*Service*](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.01/manifests/log_reader_pod/service.yaml)

```
apiVersion: v1
kind: Service
metadata:
  name: log-reader-service
spec:
  type: ClusterIP
  selector:
    app: log-reader
  ports:
    - port: 3022
      protocol: TCP
      targetPort: 3022
```


[*Ingress*](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.01/manifests/log_reader_pod/ingress.yaml)

```
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: log-reader-ingress
spec:
  rules:
  - http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: log-reader-service
            port:
              number: 3022
```

### Commands

![Commands for Exercise 2.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.01/Exercise_2.01_commands.png)

### GET Requests

**Request from outside the cluster**

![GET Request for Exercise 2.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.01/Exercise_2.01_get_request.png)

**Requests to services by name**

![GET Request for Exercise 2.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.01/Exercise_2.01_commands2.png)

**Requests to services by IP**

![GET Request for Exercise 2.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.01/Exercise_2.01_commands3.png)

**Request to log-reader pod**

![GET Request for Exercise 2.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.01/Exercise_2.01_commands4.png)



