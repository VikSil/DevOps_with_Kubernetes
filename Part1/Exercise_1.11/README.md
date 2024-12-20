## Assignment

> 
> Let's share data between "Ping-pong" and "Log output" applications using persistent volumes. Create both a *PersistentVolume* and *PersistentVolumeClaim* and alter the *Deployment* to utilize it. As PersistentVolumes are often maintained by cluster administrators rather than developers and those are not application specific you should keep the definition for those separated, perhaps in own folder.
> 
> Save the number of requests to "Ping-pong" application into a file in the volume and output it with the timestamp and hash when sending a request to our "Log output" application. In the end, the two pods should share a persistent volume between the two applications. So the browser should display the following when accessing the "Log output" application:
> 
>     2020-03-30T12:15:17.705Z: 8523ecb1-c716-4cb6-a044-b9e83bb98e43.
>     Ping / Pongs: 3

## Solution

### Binaries

**Pingpong**

- Application was built in Rust.  It listens for a GET request on `localhost:3033/pingpong`. It utilises a file at `/usr/local/files/pongs.txt` to store the number of requests received to date. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.11/app/pingpong).
- Image was pushed to Docker Hub repo [viksil/pingpong:1.11](https://hub.docker.com/r/viksil/pingpong/tags?name=1.11).

**Log output writer**

Application was unchanged from [Exercise 1.10](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.10).

**Log output reader**
- Application was built in Rust. It listens for a GET request on `localhost:3011`. When a request is received it reads the content of file at `/usr/local/files/pongs.txt` and the last line of file at `/usr/local/files/output.txt`. The two pieces of data are concatenated and displayed to the user Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.11/app/log_output_reader).
- Image was pushed to Docker Hub repo [viksil/log_output_reader:1.11](https://hub.docker.com/r/viksil/log_output_reader/tags?name=1.11).

### Manifests

[**Deployment**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.11/manifests/deployment.yaml)

```
apiVersion: apps/v1
kind: Deployment
metadata:
  name: log-output-pingpong-depl
spec:
  replicas: 1
  selector:
    matchLabels:
      app: log-output-pingpong
  template:
    metadata:
      labels:
        app: log-output-pingpong
    spec:
      volumes:
        - name: shared-volume
          persistentVolumeClaim:
            claimName: shared-volume-claim
      containers:
        - name: pingpong
          image: viksil/pingpong:1.11
          volumeMounts:
          - name: shared-volume
            mountPath: /usr/local/files
        - name: log-reader
          image: viksil/log_output_reader:1.11
          volumeMounts:
          - name: shared-volume
            mountPath: /usr/local/files
        - name: log-writer
          image: viksil/log_output_writer:1.10
          volumeMounts:
          - name: shared-volume
            mountPath: /usr/local/files
```


[**Service**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.11/manifests/service.yaml)

```
apiVersion: v1
kind: Service
metadata:
  name: log-output-pingpong-service
spec:
  type: ClusterIP
  selector:
    app: log-output-pingpong
  ports:
    - name: get-pingpong
      port: 3032
      protocol: TCP
      targetPort: 3033
    - name: get-log-output
      port: 3011
      protocol: TCP
      targetPort: 3011
```



[**Ingress**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.11/manifests/Ingress.yaml)

```
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: log-output-pingpong-ingress
spec:
  rules:
  - host: localhost
  - http:
      paths:
      - path: /pingpong
        pathType: Prefix
        backend:
          service:
            name: log-output-pingpong-service
            port:
              number: 3032
      - path: /
        pathType: Prefix
        backend:
          service:
            name: log-output-pingpong-service
            port:
              number: 3011
```

[**Persistent Volume**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.11/volumes/persistentvolume.yaml)

```
apiVersion: v1
kind: PersistentVolume
metadata:
  name: shared-volume
spec:
  storageClassName: shared-volume-pv
  capacity:
    storage: 1Gi
  volumeMode: Filesystem
  accessModes:
  - ReadWriteOnce
  local:
    path: /tmp
  nodeAffinity:
    required:
      nodeSelectorTerms:
      - matchExpressions:
        - key: kubernetes.io/hostname
          operator: In
          values:
          - k3d-k3s-default-agent-0
```

[**Persistent Volume Claim**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.11/volumes/persistentvolumeclaim.yaml)

```
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: shared-volume-claim
spec:
  storageClassName: shared-volume-pv
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 1Gi
```

### Commands

This exercise used the same cluster that was created for [exercise 1.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.07/Exercise_1.07_commands.png). Loadbalancer HTTP port `80` is exposed to outside requests on `localhost:3011`.

![Commands for Exercise 1.11](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.11/Exercise_1.11_commands1.png)

![Commands for Exercise 1.11](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.11/Exercise_1.11_commands2.png)

### GET Requests

**First request to /pingpong route**

![GET Request for Exercise 1.11](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.11/Exercise_1.11_get_pingpong.png)

**First request to / route**

![GET Request for Exercise 1.11](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.11/Exercise_1.11_get_log_output.png)

**Additional requests to /pingpong route**

![GET Request for Exercise 1.11](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.11/Exercise_1.11_get_pingpong_more.png)


### Verifying persistance

![Redeployment for Exercise 1.11](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.11/Exercise_1.11_commands3.png)

**Another request to /pingpong route** - *count adds to the previous persisted state*

![GET Request for Exercise 1.11](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.11/Exercise_1.11_get_pingpong_more_more.png)


**Another request to / route** - *count is read from the persisted file*

![GET Request for Exercise 1.11](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.11/Exercise_1.11_get_log_output_later.png)