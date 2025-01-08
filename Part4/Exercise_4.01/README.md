# Assignment

> Create a ReadinessProbe for the Ping-pong application. It should be ready when it has a connection to the database.
> 
> And another ReadinessProbe for Log output application. It should be ready when it can receive data from the Ping-pong application.
> 
> Test that it works by applying everything but the database statefulset. The output of `kubectl get po` should look like this before the database is available:
> 
>     NAME                             READY   STATUS    RESTARTS   AGE
>     logoutput-dep-7f49547cf4-ttj4f   1/2     Running   0          21s
>     pingpong-dep-9b698d6fb-jdgq9     0/1     Running   0          21s
> Adding the database should automatically move the READY states to 2/2 and 1/1 for Log output and Ping-pong respectively.

# Solution

## Binaries

### Pingpong

- Application was modified by adding a `\healthz` route and a function to handle requests to this route. The function returns HTTP code `200` or HTTP code `500` depending on the availability of the database. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.01/app/pingpong).
- Image was pushed to Docker Hub repo [viksil/pingpong:4.01](https://hub.docker.com/r/viksil/pingpong/tags?name=4.01).

### Log output

The same binaries as for [Exercise 1.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.07) were used.

### Log reader

- Application was modified by adding a `\healthz` route and a function to handle requests to this route. The function makes a request to pingpong service and returns HTTP code `200` or HTTP code `500` depending on the availability of pingpong application. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.01/app/log_reader).
- Image was pushed to Docker Hub repo [viksil/log_output_reader:4.01](https://hub.docker.com/r/viksil/log_output_reader/tags?name=4.01).

## Manifests

### [Postgres](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.01/manifests/pingpong_postgres)

- Unchanged manifests from [Exercise 2.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07) were used.

### [Pingpong](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.01/manifests/pingpong_pod)

- Service and Secret manifests from [Part3](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3) were used. 

**[Deployment](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.01/manifests/pingpong_pod/deployment.yaml)**

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
          image: viksil/pingpong:4.01
          imagePullPolicy: Always
          resources:
            limits:
              cpu: "30m"
              memory: "130Mi"
          env:
            - name: DB_URL
              valueFrom:
                secretKeyRef:
                  name: pingpong-secret
                  key: DB_URL
          readinessProbe:
            initialDelaySeconds: 180 
            periodSeconds: 10
            httpGet:
               path: /healthz
               port: 3033
```

### [Log output](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.01/manifests/log_output)

- Service and deployment manifests from [Part3](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3) were used. 

### [Log reader](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.01/manifests/log_reader)

- Service manifest from [Part3](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3) was used. 

**[ConfigMap](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.01/manifests/log_reader/configmap.yaml)**

```
apiVersion: v1
kind: ConfigMap
metadata:
  name: log-output-cfgmap
  namespace: pinglog-namespace
data:
  message: "hello world"
  information.txt: |
    this text is from file
  pingpong_hostname: "pingpong-service:3032"
  log_hostname: "log-output-service:3010"
```

**[Deployment](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.01/manifests/log_reader/deployment.yaml)**

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
          image: viksil/log_output_reader:4.01
          imagePullPolicy: Always
          env:
          - name: MESSAGE
            valueFrom:
              configMapKeyRef:
                name: log-output-cfgmap
                key: message
          - name: PINGPONG_HOST
            valueFrom:
              configMapKeyRef:
                name: log-output-cfgmap
                key: pingpong_hostname
          - name: LOG_HOST
            valueFrom:
              configMapKeyRef:
                name: log-output-cfgmap
                key: log_hostname
          volumeMounts:
          - name: configmap-volume
            mountPath: /usr/local/config
            readOnly: true
          readinessProbe:
            initialDelaySeconds: 270
            periodSeconds: 10
            httpGet:
               path: /healthz
               port: 3022
```

## Deployment

Due to implementation details starting the pods without the database will cause the Pingpong pod to continously fail, enter `CrashLoopBackOff` state, terminate and start again. The Log Reader pod does not encounter an error, since it does not require a direct connection to the database. But due to the Readiness probe the Log Reader pod keeps waiting on the Pingpong pod and never becomes ready.

![Commands for Exercise 4.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.01/Exercise_4.01_commands.png)

The readiness probes for Pingpong pod and Log Reader pod are set to initial delay of three and four-and-a-half minutes respectivelly. This is done because the Deployment is also set to `imagePullPolicy` = `Always`, and pulling an image from Docker Hub can take significant time on a slower Internet connection. 
Once the database is made available, both the Pingpong pod and the Log Reader pod become available. 

![Commands for Exercise 4.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.01/Exercise_4.01_commands2.png)