# Assignment

> Use the official Kubernetes documentation for this exercise.
> 
>   - https://kubernetes.io/docs/concepts/configuration/configmap/ and
>   - https://kubernetes.io/docs/tasks/configure-pod-container/configure-pod-configmap/
> 
> should contain everything you need.
> 
> Create a ConfigMap for the "Log output" application. The ConfigMap should define one file *information.txt* and one env variable *MESSAGE*.
> 
> The app should map the file as a volume, and set the environment variable and print the content of those besides the usual output:
> 
>       file content: this text is from file
>       env variable: MESSAGE=hello world
>       2024-03-30T12:15:17.705Z: 8523ecb1-c716-4cb6-a044-b9e83bb98e43.
>       Ping / Pongs: 3

# Solution

## Binaries

### Pingpong and Log output

Solution from [Exercise 2.01](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.01) was used.

### Log reader

- Functionality was added to the existing application to read file at `/usr/local/config/information.txt` and to retrieve the value from environment variable `MESSAGE`. Both were added to the response that the application returns to a GET request.
- Image was pushed to Docker Hub repo [viksil/log_output_reader:2.06](https://hub.docker.com/r/viksil/log_output_reader/tags?name=2.06).

## Manifests

Unchanged manifests from [Exercise 2.03](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.03) were used for all objects, except Deployment of the Log reader application.

### [Log reader Deployment](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.06/manifests/log_reader_deployment.yaml)

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
          image: viksil/log_output_reader:2.06
          env:
          - name: MESSAGE
            valueFrom:
              configMapKeyRef:
                name: log-output-cfgmap
                key: message
          volumeMounts:
          - name: configmap-volume
            mountPath: /usr/local/config
            readOnly: true
```

### [ConfigMap](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.06/manifests/configmap.yaml)

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
   
```

## Commands

![Commands for Exercise 2.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.06/Exercise_2.06_commands.png)

![Commands for Exercise 2.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.06/Exercise_2.06_commands.png)

## GET request

![Response to GET request](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.06/Exercise_2.06_log_output_reader.png)