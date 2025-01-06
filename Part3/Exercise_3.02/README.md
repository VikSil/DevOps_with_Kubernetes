# Assignment

> Deploy the "Log output" and "Ping-pong" applications into GKE and expose it with Ingress.
> 
> "Ping-pong" will have to respond from /pingpong path. This may require you to rewrite parts of the code.
> 
> Note that Ingress expects a service to give a successful response in the path / even if the service is mapped to some other path!
> 


# Solution

## Binaries

### Log output

The same binaries as for [Exercise 1.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.07) were used.


### Pingpong

- Application was modified by adding a `/` route that responds to `GET` requests with HTML response code `200`:`OK` and outputs a log line to `stdout`. The number of pings/pongs are still returned on `/pingpong` endpoint. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.02/app/pingpong).
- Image was pushed to Docker Hub repo [viksil/pingpong:3.02](https://hub.docker.com/r/viksil/pingpong/tags?name=3.02).


## Manifests

Namespace declaration was added at the top of the `metadata` section of all `.yaml` files.

```
metadata:
  namespace: pinglog-namespace
```

### Postgres

- Unchanged manifests from [Exercise 3.01](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.01) were used.


### Pingpong

- Unchanged manifest for `Secret` from [Exercise 3.01](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.01) was used.

**[Deployment](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.02/manifests/pingpong_pod/manifest.yaml)**

Manifest from [Exercise 3.01](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.01) was used with only the `spec.template.spec.containers.image` tag changed to the latest version `viksil/pingpong:3:02`.

```
...

    spec:
      containers:
        - name: pingpong
          image: viksil/pingpong:3.02
...

```

**[Service](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.02/manifests/pingpong_pod/service.yaml)**

- Manifest for `Service` was changed by switching it to `spec.type:NodePort` and switching back to port `3033`.

```
apiVersion: v1
kind: Service
metadata:
  namespace: pinglog-namespace
  name: pingpong-service
spec:
  type: NodePort
  selector:
    app: pingpong
  ports:
    - port: 3033
      protocol: TCP
      targetPort: 3033
```

### Log output

- `Deployment` manifest from [Exercise 2.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07) was used.

- `Service` manifest from [Exercise 2.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07) was used with `spec.type` changed to `NodePort`, same as for the Pingpong service.

### [Shared Ingress](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.02/manifests/shared_ingress.yaml)

```
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  namespace: pinglog-namespace
  name: shared-ingress
spec:
  rules:
  - http:
      paths:
      - path: /pingpong
        pathType: Prefix
        backend:
          service:
            name: pingpong-service
            port:
              number: 3033
      - path: /
        pathType: Prefix
        backend:
          service:
            name: log-output-service
            port:
              number: 3011
```

## Commands

![Commands for Exercise 3.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.02/Exercise_3.02_commands.png)

![Commands for Exercise 3.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.02/Exercise_3.02_commands2.png)

![Commands for Exercise 3.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.02/Exercise_3.02_commands3.png)

## Frontend

![Log output for Exercise 3.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.02/Exercise_3.02_frontend.png)

![Pingpong output for Exercise 3.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.02/Exercise_3.02_frontend1.png)