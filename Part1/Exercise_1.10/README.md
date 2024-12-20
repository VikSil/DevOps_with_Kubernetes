## Assignment

> 
> Split the "Log output" application into two different containers within a single pod:
> 
> One generates a new timestamp every 5 seconds and saves it into a file.
> 
> The other reads that file and outputs it with a hash for the user to see.
> 
> Either application can generate the hash. The reader or the writer.
> 
> You may find [this](https://kubernetes.io/docs/reference/kubectl/generated/kubectl_logs/) helpful now since there are more than one container running inside a pod.

## Solution

### Binaries

**Log output writer**

- Application was built in Rust. It outputs a random string with timestamp to console every five seconds and appends the same string to file at `/usr/local/files/output.txt`. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.10/app/log_output_writer).
- Image was pushed to Docker Hub repo [viksil/log_output_writer:1.10](https://hub.docker.com/r/viksil/log_output_writer/tags?name=1.10).

**Log output reader**
- Application was built in Rust. It listens for a GET request on `localhost:3011`.  When a request is received it reads the content of file at `/usr/local/files/output.txt` and displayed the last line to the user. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.10/app/log_output_reader).
- Image was pushed to Docker Hub repo [viksil/log_output_reader:1.10](https://hub.docker.com/r/viksil/log_output_reader/tags?name=1.10).

### Manifests

[**Deployment**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.10/manifests/deployment.yaml)

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
      volumes:
        - name: shared-dir
          emptyDir: {}
      containers:
        - name: log-output-reader
          image: viksil/log_output_reader:1.10
          volumeMounts:
          - name: shared-dir
            mountPath: /usr/local/files/
        - name: log-output-writer
          image: viksil/log_output_writer:1.10
          volumeMounts:
          - name: shared-dir
            mountPath: /usr/local/files/
```

[**Service**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.07/manifests/service.yaml)

Unchanged service manifest from [Exercise 1.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.07) was used.


[**Ingress**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.07/manifests/Ingress.yaml)

Unchanged ingress manifest from [Exercise 1.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.07) was used. 


### Commands

This exercise used the same cluster that was created for [exercise 1.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.07/Exercise_1.07_commands.png). Loadbalancer HTTP port `80` is exposed to outside requests on `localhost:3011`.

![Commands for Exercise 1.10](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.10/Exercise_1.10_commands.png)


### GET Request

![GET Request for Exercise 1.10](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.10/Exercise_1.10_get_request.png)

