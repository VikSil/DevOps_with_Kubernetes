# Assignment

> Set sensible resource limits for the Ping-pong and Log output applications. The exact values are not important. Test what works.

# Solution

## Binaries

## Log output and pingpong

The same binaries as for [Exercise 3.02](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.02) were used.

## Log reader

- Application was modified to use `BACKEND_HOST` environment variable as connection string to pingpong pod and log output pod. The value for `BACKEND_HOST` was set to the external IP of the ingress and passed to the pod via a ConfigMap. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.09/app/log_reader).
- Image was pushed to Docker Hub repo [viksil/log_output_reader:3.09](https://hub.docker.com/r/viksil/log_output_reader/tags?name=3.09).

## Manifests

Resource limits for Pingpong pod, log output pod and log reader pod were set to 3% CPU and 130Mi of memory by adding the following directives to container each manifest:

```
...
          resources:
            limits:
              cpu: "30m"
              memory: "130Mi"
...
```

### Postgres

- Unchanged manifests from [Exercise 3.02](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.02) were used.

### Pingpong

- Manifests from [Exercise 3.02](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.02) were used with the resource limits added to the Deployment manifest.

### Log output

- Manifests from [Exercise 3.02](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.02) were used with the resource limits added to the Deployment manifest.

### Log input

- Unchanged service manifest for service from [Exercise 3.02](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.02) was used.

**[ConfigMap](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.09/manifests/log_reader/configmap.yaml)**

```
apiVersion: v1
kind: ConfigMap
metadata:
  name: log-output-cfgmap
  namespace: pinglog-namespace
data:
  message: "hello world"
  information.txt: |
    file.content=this text is from file
  backend-host: "localhost"
```


**[Deployment](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.09/manifests/log_reader/deployment.yaml)**

```
apiVersion: apps/v1
kind: Deployment
metadata:
  namespace: pinglog-namespace
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
      volumes:
        - name: configmap-volume
          configMap:
            name: log-output-cfgmap
            items:
            - key: "information.txt"
              path: "information.txt" 
      containers:
        - name: log-reader
          image: viksil/log_output_reader:3.09
          resources:
            limits:
              cpu: "30m"
              memory: "130Mi"
          env:
          - name: MESSAGE
            valueFrom:
              configMapKeyRef:
                name: log-output-cfgmap
                key: message
          - name: BACKEND_HOST
            valueFrom:
              configMapKeyRef:
                name: log-output-cfgmap
                key: backend-host
          volumeMounts:
          - name: configmap-volume
            mountPath: /usr/local/config
            readOnly: true
```

### Namespace

- Manifest from [Exercise 2.03](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_2.03) was used.

### Shared ingress

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
      - path: /logreader
        pathType: Prefix
        backend:
          service:
            name: log-reader-service
            port:
              number: 3022
      - path: /
        pathType: Prefix
        backend:
          service:
            name: log-output-service
            port:
              number: 3011
```

## Deployment

![Deployment for Exercise 3.09](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.09/Exercise_3.09_deployment.png)