## Assignment

> 
> Create a deployment.yaml for the project.
> 
> You won't have access to the port yet but that'll come soon.

## Solution

### Deployment manifest

```
apiVersion: apps/v1
kind: Deployment
metadata:
  name: todo-app-depl
spec:
  replicas: 1
  selector:
    matchLabels:
      app: todo-app
  template:
    metadata:
      labels:
        app: todo-app
    spec:
      containers:
        - name: todo-app
          image: viksil/todo_app:1.02
          env:
          - name: PORT
            value: "8088"
```


### Commands

![Deployment for Exercise 1.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.04/Exercise_1.04.png)
