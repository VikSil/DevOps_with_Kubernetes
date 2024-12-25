# Assignment

> Create a database and save the todos there. Again, the database should have its own pod.
> 
> Use Secrets and/or ConfigMaps to have the backend access the database.

# Solution

## Binaries

### Todo backend

- Application was built in Rust, using `axum` crate. The application listens to port `3040` by default, unless environment variable `PORT` is defined. It requires environment variable `DB_URL` to be set to the connection string for the database. It accepts `GET` and `POST` requests on `\todos` endpoint. Upon first initialisation the application runs migration scripts that create database structures to store todos, url to image and timestamp of when the image was last changed. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08/app/todo_backend).
- Image was pushed to Docker Hub repo [viksil/todo_backend:2.08](https://hub.docker.com/r/viksil/todo_backend/tags?name=2.08).

### Todo app

- Application was built in Rust, using `yew` crate. The application requires environment variable `PORT` to be present at **compile** time (`yew` does not support runtime environment variables). It communicates with the backend via `GET` and `POST` requests and displays the todos and an image to the user. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08/app/todo_app).
- Image was pushed to Docker Hub repo [viksil/todo_app:2.08](https://hub.docker.com/r/viksil/todo_app/tags?name=2.08).

### Log reader app

The log reader app was modified to accept connections on `/logreader` route, so to not clash with todo app while running in the same cluster.

## Manifests

In all manifests the namespace was changed to 

      namespace: todo-namespace

### Postgres pod

Postgres pod manifests from [Exercise 2.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07) were used with the following changes:

  - Volume was renamed to `todo-volume` in order to distinguish from the volume used in the previous exercise.
  - Volume was mounted on path `"/mnt/todo"`.
  - Volume claim was renamed to `todo-volume-claim`
  - A different database named `todos` was used to store the data.


**[StatefulSet](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08/manifests/postgres_pod/statefulset.yaml)**


```
...
          volumeMounts:
            - mountPath: /var/lib/postgresql/data
              name: todo
      volumes:
        - name: todo
          persistentVolumeClaim:
            claimName: todo-volume-claim
```

**[PersistentVolume](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08/manifests/postgres_pod/volume.yaml)**

```
apiVersion: v1
kind: PersistentVolume
metadata:
  namespace: todo-namespace
  name: todo-volume
  labels:
    type: local
    app: postgres
spec:
  storageClassName: manual
  capacity:
    storage: 1Gi
  accessModes:
    - ReadWriteMany
  hostPath:
    path: "/mnt/todo"
```

**[PersistentVolumeClaim](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08/manifests/postgres_pod/volumeclaim.yaml)**

```
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  namespace: todo-namespace
  name: todo-volume-claim
...
```

**[ConfigMap](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08/manifests/postgres_pod/configmap.yaml)**

```
...
data:
  POSTGRES_DB: todos
  POSTGRES_USER: postgres

```

**[Service](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07/manifests/postgres_pod/service.yaml)**

Unchanged manifest from [Exercise 2.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07) was used.


**Secret**

Secret manifest was used to pass in `POSTGRES_PASSWORD` environment variable.

### Todo backend

**[Deployment](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08/manifests/todo_backend/deployment.yaml)**

```
apiVersion: apps/v1
kind: Deployment
metadata:
  namespace: todo-namespace
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
          image: viksil/todo_backend:2.08
          imagePullPolicy: Always
          env:
            - name: PORT
              valueFrom:
                configMapKeyRef:
                  name: todo-cfgmap
                  key: port
            - name: DB_URL
              valueFrom:
                secretKeyRef:
                  name: todo-secret
                  key: DB_URL         
```

**[Service](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08/manifests/todo_backend/service.yaml)**

```
apiVersion: v1
kind: Service
metadata:
  namespace: todo-namespace
  name: todo-backend-service
spec:
  type: ClusterIP
  selector:
    app: todo-backend
  ports:
    - port: 3055
      protocol: TCP
      targetPort: 3055
```

**[ConfigMap](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08/manifests/todo_backend/configmap.yaml)**

```
apiVersion: v1
kind: ConfigMap
metadata:
  name: todo-cfgmap
  namespace: todo-namespace
data:
  port: "3055"
   
```

**Secret**

Secret manifest was used to pass in `DB_URL` environment variable, containing databse credentials.

### Todo app

**[Deployment](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08/manifests/todo_app/deployment.yaml)**

```
apiVersion: apps/v1
kind: Deployment
metadata:
  namespace: todo-namespace
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
          image: viksil/todo_app:2.08
          imagePullPolicy: Always
          env:
            - name: PORT
              valueFrom:
                configMapKeyRef:
                  name: todo-cfgmap
                  key: port
```

**[Service](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08/manifests/todo_app/service.yaml)**

```
kind: Service
metadata:
  namespace: todo-namespace
  name: todo-app-service
spec:
  type: ClusterIP
  selector:
    app: todo-app
  ports:
    - port: 3012
      protocol: TCP
      targetPort: 3055
```

**[Ingress](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08/manifests/todo_backend/shared_ingress.yaml)**

```
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  namespace: todo-namespace
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
              number: 3055
      - path: /
        pathType: Prefix
        backend:
          service:
            name: todo-app-service
            port:
              number: 3012
```

## Commands

![Commands for Exercise 2.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.08/Exercise_2.08_commands.png)

![Commands for Exercise 2.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.08/Exercise_2.08_commands1.png)

![Commands for Exercise 2.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.08/Exercise_2.08_commands2.png)

![Commands for Exercise 2.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.08/Exercise_2.08_commands3.png)

![Commands for Exercise 2.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.08/Exercise_2.08_commands4.png)

![Commands for Exercise 2.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.08/Exercise_2.08_commands5.png)

![Commands for Exercise 2.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.08/Exercise_2.08_commands6.png)

![Commands for Exercise 2.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.08/Exercise_2.08_commands7.png)
