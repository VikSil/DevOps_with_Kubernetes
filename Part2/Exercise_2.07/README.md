# Assignment

> Run a Postgres database and save the Ping-pong application counter into the database.
> 
> The Postgres database and Ping-pong application should **not be** in the same pod. A single Postgres database is enough and it may disappear with the cluster but it should survive even if all pods are taken down.
> 
> **Hint:** it might be a good idea to ensure that the database is operational and available for connections before you try connecting it from the Ping-pong app. For that purpose, you might just start a stand-alone pod that runs a Postgres image:
> 
>     kubectl run -it --rm --restart=Never --image postgres psql-for-debugging sh
>     $ psql postgres://yourpostgresurlhere
>     psql (16.2 (Debian 16.2-1.pgdg120+2))
>     Type "help" for help.
>     postgres=# \d
>     Did not find any relations.

# Solution

## Binaries

### Log output and Log reader

The same binaries as for [Exercise 2.06](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.06) were used.

### Pingpong

- Application was built in Rust using `sqlx` crate. It connects to a database path defined in `DB_URL` environment variable. Upon initialisation the application runs a migration script, if not run previously. The script creates `pings` table and inserts a row with initial number of `pongs`. The application listens for a GET request on `localhost:3033/pingpong`. Upon receiving requests, the number of `pongs` in the database is updated. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07/app/pingpong).
- Image was pushed to Docker Hub repo [viksil/pingpong:2.07](https://hub.docker.com/r/viksil/pingpong/tags?name=2.07).

## Manifests

### Log output and Log reader pods

Unchanged manifests from [Exercise 2.06](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.06) were used.

### Pingpong pod

Unchanged manifests from [Exercise 2.06](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.06) were used for `pingpong-service`.

**[Deployment](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07/manifests/pingpong_pod/deployment.yaml)**

```
apiVersion: apps/v1
kind: Deployment
metadata:
  namespace: pinglog-namespace
  name: pingpong-depl
spec:
  replicas: 1
  selector:
    matchLabels:
      app: pingpong
  template:
    metadata:
      labels:
        app: pingpong
    spec:
      containers:
        - name: pingpong
          image: viksil/pingpong:2.07
          imagePullPolicy: Always          
          env:
            - name: DB_URL
              valueFrom:
                secretKeyRef:
                  name: pingpong-secret
                  key: DB_URL

```

**Secret**

Secret manifest was used to pass in `DB_URL` environment variable, containing databse credentials.

### Postgres pod

**[StatefulSet](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07/manifests/postgres_pod/statefulset.yaml)**

For simplicity only one replica of a StatefulSet was used. Otherwise, the reads and writes are not always routed to the same instance of the database, without setting up a more complicated master-slave relationship between the replicas.

```
apiVersion: apps/v1
kind: StatefulSet
metadata:
  namespace: pinglog-namespace
  name: postgres
spec:
  serviceName: postgres
  replicas: 1
  selector:
    matchLabels:
      app: postgres
  template:
    metadata:
      labels:
        app: postgres
    spec:
      containers:
        - name: postgres
          image: postgres:10.1
          imagePullPolicy: "IfNotPresent"
          ports:
            - containerPort: 5432
          envFrom:
            - configMapRef:
                name: postgres-cfgmap
          env:
            - name: POSTGRES_PASSWORD
              valueFrom:
                secretKeyRef:
                  name: postgres-secret
                  key: POSTGRES_PASSWORD
          volumeMounts:
            - mountPath: /var/lib/postgresql/data
              name: pingpong
      volumes:
        - name: pingpong
          persistentVolumeClaim:
            claimName: postgres-volume-claim
```

**[Service](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07/manifests/postgres_pod/service.yaml)**

```
apiVersion: v1
kind: Service
metadata:
  namespace: pinglog-namespace
  name: postgres-service
  labels:
    app: postgres
spec:
  ports:
    - port: 5432
      name: postgres-port
      targetPort: 5432
  clusterIP: None
  selector:
    app: postgres

```

**[PersistentVolume](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07/manifests/postgres_pod/volume.yaml)**

```
apiVersion: v1
kind: PersistentVolume
metadata:
  namespace: pinglog-namespace
  name: postgres-volume
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
    path: "/mnt/data"
```

**[PersistentVolumeClaim](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07/manifests/postgres_pod/volumeclaim.yaml)**

```
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  namespace: pinglog-namespace
  name: postgres-volume-claim
  labels:
    app: postgres
spec:
  storageClassName: manual
  accessModes:
    - ReadWriteMany
  resources:
    requests:
      storage: 1Gi
```

**[ConfigMap](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07/manifests/postgres_pod/configmap.yaml)**

```
apiVersion: v1
kind: ConfigMap
metadata:
  name: postgres-cfgmap
  namespace: pinglog-namespace
  labels:
    app: postgres
data:
  POSTGRES_DB: pingpong
  POSTGRES_USER: postgres

```

**Secret**

Secret manifest was used to pass in `POSTGRES_PASSWORD` environment variable.

## Commands

![Commands for Exercise 2.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.07/Exercise_2.07_commands.png)

![Commands for Exercise 2.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.07/Exercise_2.07_commands1.png)

![Commands for Exercise 2.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.07/Exercise_2.07_commands2.png)

![Commands for Exercise 2.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.07/Exercise_2.07_commands3.png)

![Commands for Exercise 2.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.07/Exercise_2.07_commands4.png)