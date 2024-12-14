## Assignment

> 
> For the project, we'll need to do some coding to start seeing results in the next part.
> 
> 1. Add an input field. The input should not take todos that are over 140 characters long.
> 2. Add a send button. It does not have to send the todo yet.
> 3. Add a list of the existing todos with some hardcoded todos.
> 
> Maybe something similar to this:
> 
> ![Example interface](https://devopswithkubernetes.com/static/ff807ebf379aa4fd08d98d96b03d969c/8ae5a/project-ex-113.webp)

## Solution

### Binaries

- Application was built in Rust. By default the application listens to port 3030, if no `PORT` environment variable is provided. It returns an HTML age with a single random image downloaded from `https://picsum.photos/1200`. By default the image is replaced every hour, but it is possible to specify an environemnt variable `TIMEOUT` (in seconds) to specify how frequently a new picture should be returned. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.13/app/todo_app).
- Image was pushed to Docker Hub repo [viksil/todo_app:1.13](https://hub.docker.com/r/viksil/todo_app/tags?name=1.13).

### Manifests

[**Deployment**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.12/manifests/deployment.yaml)

Deployment manifest from [Exercise 1.12](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.12) was used, with only the `image` declaration changed to the latest version of todo_app:

    image: viksil/todo_app:1.13


[**Service**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.08/manifests/service.yaml)

Unchanged service manifest from [Exercise 1.08](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.08) was used.


[**Ingress**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.08/manifests/ingress.yaml)

Unchanged ingress manifest from [Exercise 1.08](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.08) was used.


[**Persistent Volume**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.12/volumes/persistentvolume.yaml)

Unchanged persistent Volume manifest from [Exercise 1.12](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.12) was used.

[**Persistent Volume Claim**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.12/volumes/persistentvolumeclaim.yaml)

Unchanged persistent Volume Claim manifest from [Exercise 1.12](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.12) was used.

### Commands

This exercise used the same cluster that was created for [exercise 1.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.07/Exercise_1.07_commands.png). Loadbalancer HTTP port `80` is exposed to outside requests on `localhost:3011`.

The same commands as for [Exercise 1.12][Exercise 1.12](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.12) were used to create all resources.

### Webpage

![Webpage for Exercise 1.13](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.13/Exercise_1.13_todo_app.png)
