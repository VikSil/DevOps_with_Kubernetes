## Assignment

> 
> In your "Log output" application create a folder for manifests and move your deployment into a declarative file.
> 
> Make sure everything still works by restarting and following logs.

## Solution

### Deployment manifest

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
          image: viksil/log_output:1.01
```


### Commands

![Deployment for Exercise 1.03](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.03/Exercise_1.03.png)
