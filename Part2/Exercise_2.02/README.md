## Assignment

> 
> Let us get back to our Project. In the previous part we added a random pic and a form for creating todos to the app. The next step is to create a new container that takes care of saving the todo items.
> 
> This new service, let us call it todo-backend, should have a GET /todos endpoint for fetching the list of todos and a POST /todos endpoint for creating a new todo. The todos can be saved into memory, we'll add a database later.
> 
> Use ingress routing to enable access to the todo-backend.
> 
> After this exercise, the project should look like the following:
> 
> ![Todo application diagram](https://devopswithkubernetes.com/static/bc4bed9387ebafa11912ae48b2339d14/b3c31/p2-2.webp)
>
> The role of the service that we made in previous exercises (Todo-app in the figure) is to serve the HTML and possibly JavaScript to the browser. Also, the logic for serving random pictures and caching those remain in that service.
> 
> The new service then takes care of the todo items.
> 
> After this exercise, you should be able to create new todos using the form, and the created todos should be rendered in the browser.

## Solution

### Binaries

**Todo backend**

- Application was built in Rust, using `axum` crate. the application listens to port `3040`. It accepts `GET` and `POST` requests on `\todos` endpoint. Along with the todos, this application returns a url from image picsum to display in the frontend. The link changes every hour. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.02/app/todo_backend).
- Image was pushed to Docker Hub repo [viksil/todo_backend:2.02](https://hub.docker.com/r/viksil/todo_backend/tags?name=2.02).

**Todo app**

- Application was rebuilt from scratch using Rust `yew` crate to accomodate the need for a dynamic frontend. By default the application listens to port `3030`, unless `PORT` environment variable is provided. It communicates with the backend via `GET` and `POST` requests and displays the todos and an image to the user. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.02/app/todo_app).
- Image was pushed to Docker Hub repo [viksil/todo_app:2.02](https://hub.docker.com/r/viksil/todo_app/tags?name=2.02).

### Manifests

**Todo backend**

[*Deployment*](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.02/manifests/todo_backend/deployment.yaml)

```
apiVersion: apps/v1
kind: Deployment
metadata:
  name: todo-backend-depl
spec:
  replicas: 1
  selector:
    matchLabels:
      app: todo-backend
  template:
    metadata:
      labels:
        app: todo-backend
    spec:
      containers:
        - name: todo-backend
          image: viksil/todo_backend:2.02
```


[*Service*](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.02/manifests/todo_backend/service.yaml)

```
apiVersion: v1
kind: Service
metadata:
  name: todo-backend-service
spec:
  type: ClusterIP
  selector:
    app: todo-backend
  ports:
    - port: 3040
      protocol: TCP
      targetPort: 3040
```

**Todo app**

[*Deployment*](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.04/deployment.yaml)

Deployment manifest from [Exercise 1.04](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.04) was used, with only the `image` declaration changed to the latest version of todo_app:

    image: viksil/todo_app:2.02


[*Service*](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.08/manifests/service.yaml)

Unchanged service manifest from [Exercise 1.08](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.08) was used.


[**Shared Ingress**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.02/manifests/shared_ingress.yaml)

```
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: todo-shared-ingress
spec:
  rules:
  - http:
      paths:
      - path: /todos
        pathType: Prefix
        backend:
          service:
            name: todo-backend-service
            port:
              number: 3040
      - path: /
        pathType: Prefix
        backend:
          service:
            name: todo-app-service
            port:
              number: 3012
```

### Commands

A new cluster was created for this exercise, exposing loadbalancer on port `3040`, thus allowing the frontend to communicate via the browser to the backend that listens on the same port.

![Commands for Exercise 2.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.02/Exercise_2.02_commands.png)


### Frontend

![Frontend for Exercise 2.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.02/Exercise_2.02_todo_app.png)

### Backend

![Backend response](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.02/Exercise_2.02_todo_backend.png)

