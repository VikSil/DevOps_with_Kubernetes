# Assignment

> Make the Ping-pong application serverless.
> 
> Reading [this](https://knative.dev/docs/serving/convert-deployment-to-knative-service/) might be helpful.
> 
> TIP: Your application should listen on port 8080 or better yet have a *PORT* environment variable to configure this.

# Solution

## Binaries

### Pingpong

- Application was modified by adding introducing a `PING_PORT` environment variable to start the application on. 
Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.07/app/pingpong).
- Image was pushed to Docker Hub repo [viksil/pingpong:5.07](https://hub.docker.com/r/viksil/pingpong/tags?name=5.07).

### Log output

- Application was modified by adding introducing a `LOG_PORT` environment variable to start the application on. 
Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.07/app/log_output).
- Image was pushed to Docker Hub repo [viksil/log_output:5.07](https://hub.docker.com/r/viksil/log_output/tags?name=5.07).

### Log reader

- Application was modified by adding introducing a `READ_PORT` environment variable to start the application on. 
Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.07/app/log_reader).
- Image was pushed to Docker Hub repo [viksil/log_output_reader:5.07](https://hub.docker.com/r/viksil/log_output_reader/tags?name=5.07).

## Manifests

### Postgres

- Unchanged manifests from [Exercise 2.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07) were used, since serverless services are stateless, but postgres database must be deployed as a StatefulSet.

### [Pingpong](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.07/manifests/pingpong/service.yaml)

- Service and Deployment manifest were merged into a Knative Service manifest.
- `spec.template.spec.containers.image` was changed to `viksil/pingpong:5.07`
- environment variable `PING_PORT` with value `"8080"` was added.
- Unchanged Secret manifest from [Exercise 4.01](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.01) was used.

```
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: pingpong
spec:
  template:
    spec:
      containers:
      - image: viksil/pingpong:5.07
        imagePullPolicy: Always
        resources:
          limits:
            cpu: "30m"
            memory: "130Mi"
        ports:
        - containerPort: 8080
        env:
          - name: DB_URL
            valueFrom:
              secretKeyRef:
                name: pingpong-secret
                key: DB_URL
          - name: PING_PORT
            value: "8080"
        readinessProbe:
          initialDelaySeconds: 180 
          periodSeconds: 10
          httpGet:
              path: /healthz
              port: 8080
```

### [Log output](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.07/manifests/log_output/service.yaml)

- Service and Deployment manifest were merged into a Knative Service manifest.
- `spec.template.spec.containers.image` was changed to `viksil/log_output:5.07`.
- environment variable `LOG_PORT` with value `"8080"` was added.

```
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: log-output
spec:
  template:
    spec:
      containers:
      - image: viksil/log_output:5.07
        imagePullPolicy: Always
        resources:
          limits:
            cpu: "30m"
            memory: "130Mi"
        env:
          - name: LOG_PORT
            value: "8080"
        ports:
        - containerPort: 8080
```

### Log reader

[**ConfigMap**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.07/manifests/log_reader/configmap.yaml)

```
apiVersion: v1
kind: ConfigMap
metadata:
  name: log-output-cfgmap
data:
  message: "hello world"
  information.txt: |
    this text is from file
  pingpong_hostname: "pingpong.default.172.27.0.2.sslip.io"
  log_hostname: "log-output.default.172.27.0.2.sslip.io"
```

[**Service**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.07/manifests/log_reader/service.yaml)

- Service and Deployment manifest were merged into a Knative Service manifest.
- `spec.template.spec.containers.image` was changed to `viksil/log_output_reader:5.07`.
- environment variable `READ_PORT` with value `"8080"` was added.

```
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: log-reader
spec:
  template:
    spec:
      volumes:
        - name: configmap-volume
          configMap:
            name: log-output-cfgmap
            items:
            - key: "information.txt"
              path: "information.txt" 
      containers:
      - image: viksil/log_output_reader:5.07
        imagePullPolicy: Always
        ports:
        - containerPort: 8080
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
        - name: READ_PORT
          value: "8080"
        volumeMounts:
        - name: configmap-volume
          mountPath: /usr/local/config
          readOnly: true
        readinessProbe:
          initialDelaySeconds: 270
          periodSeconds: 10
          httpGet:
              path: /healthz
              port: 8080

```

## Commands

![Commands for Exercise 5.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.07/Exercise_5.07_commands.png)

![Commands for Exercise 5.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.07/Exercise_5.07_commands1.png)

![Commands for Exercise 5.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.07/Exercise_5.07_commands2.png)

![Commands for Exercise 5.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.07/Exercise_5.07_commands4.png)

![Commands for Exercise 5.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.07/Exercise_5.07_commands5.png)

![Commands for Exercise 5.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.07/Exercise_5.07_commands6.png)

![Commands for Exercise 5.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.07/Exercise_5.07_commands7.png)

## Frontend

![Commands for Exercise 5.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.07/Exercise_5.07_frontend.png)

![Commands for Exercise 5.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.07/Exercise_5.07_frontend2.png)

![Commands for Exercise 5.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.07/Exercise_5.07_frontend3.png)

![Commands for Exercise 5.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.07/Exercise_5.07_frontend4.png)