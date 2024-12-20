## Assignment

> 
> Since the project looks a bit boring right now, let's add a picture!
> 
> The goal is to add an hourly image to the project.
> 
> Get a random picture from Lorem Picsum like `https://picsum.photos/1200` and display it in the project. Find a way to store the image so it stays the same for 60 minutes.
> 
> Make sure to cache the image into a volume so that the API isn't needed for new images every time we access the application or the container crashes.
> 
> The best way to test what happens when your container shuts down is likely by shutting down the container, so you can add logic for that as well, for testing purposes.

## Solution

### Binaries

- Application was built in Rust. By default the application listens to port 3030, if no `PORT` environment variable is provided. It returns an HTML age with a single random image downloaded from `https://picsum.photos/1200`. By default the image is replaced every hour, but it is possible to specify an environemnt variable `TIMEOUT` (in seconds) to specify how frequently a new picture should be returned. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.12/app/todo_app).
- Image was pushed to Docker Hub repo [viksil/todo_app:1.12](https://hub.docker.com/r/viksil/todo_app/tags?name=1.12).

### Manifests

[**Deployment**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.12/manifests/deployment.yaml)

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
      volumes:
        - name: todo-app-volume
          persistentVolumeClaim:
            claimName: todo-app-volume-claim
      containers:
        - name: todo-app
          image: viksil/todo_app:1.12
          env:
          - name: PORT
            value: "8088"
          # uncomment to set timeout between image downloads (in seconds)
          # - name: TIMEOUT
          #   value: "25"
          volumeMounts:
          - name: todo-app-volume
            mountPath: /usr/local/files
```


[**Service**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.08/manifests/service.yaml)

Unchanged service manifest from [Exercise 1.08](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.08) was used.


[**Ingress**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.08/manifests/ingress.yaml)

Unchanged ingress manifest from [Exercise 1.08](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.08) was used.

[**Persistent Volume**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.12/volumes/persistentvolume.yaml)

Persistent Volume manifest from [Exercise 1.11](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.11) was used, with only the `metadata name` and `storageClassName` changed to *todo_app*:

```
metadata:
  name: todo-app-volume
spec:
  storageClassName: todo-app-volume-pv
```

[**Persistent Volume Claim**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.12/volumes/persistentvolumeclaim.yaml)

Persistent Volume Claim manifest from [Exercise 1.11](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.11) was used, with only the `metadata name` and `storageClassName` changed to *todo_app*:

```
metadata:
  name: todo-app-volume-claim
spec:
  storageClassName: todo-app-volume-pv
```

### Commands

This exercise used the same cluster that was created for [exercise 1.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.07/Exercise_1.07_commands.png). Loadbalancer HTTP port `80` is exposed to outside requests on `localhost:3011`.

![Commands for Exercise 1.12](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.12/Exercise_1.12_commands.png)

### Webpage

![Webpage for Exercise 1.12](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.12/Exercise_1.12_todo_app.png)
