# Assignment

> Create an AnalysisTemplate for The Project that will follow the CPU usage of all containers in the namespace.
> 
> If the CPU usage **rate** sum for the namespace increases above a set value (you may choose a good hardcoded value for your project) within 10 minutes, revert the update.
> 
> Make sure that the application doesn't get updated, if the value is set too low.

# Solution

## Manifests

### Postgres

- Unchanged manifests from [Exercise 2.08](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08) were used.

### Todo backend

Consecutive upgrades were rolled out by switching the image version [`2.02`](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.02) --> [`2.08`](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08) --> [`2.10`](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.10) --> [`4.02`](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.02). Coresponding versions of Service, ConfigMap and Secret manifests were used. Deployment manifest was ommited and replaced by a Rollout manifest.

**[Rollout](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.04/manifests/todo_backend_rollout.yaml)**

```
apiVersion: argoproj.io/v1alpha1
kind: Rollout
metadata:
  namespace: todo-namespace
  name: todo-backend-depl
spec:
  replicas: 1
  selector:
    matchLabels:
      app: todo-backend
  strategy:
    canary:
      steps:
      - setWeight: 25
      - pause:
          duration: 30s
      - setWeight: 50
      - pause:
          duration: 30s
      - analysis:
          templates:
          - templateName: cpu-usage-rate
  template:
    metadata:
      labels:
        app: todo-backend
    spec:
      containers:
        - name: todo-backend
          image: viksil/todo_backend:4.02
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
          readinessProbe:
            initialDelaySeconds: 180 
            periodSeconds: 10
            exec:
              command:
              - sh
              - -c
              - wget --spider -S "http://0.0.0.0:$PORT/healthz" 2>&1 | grep "HTTP/" | awk '{print $2}'
          livenessProbe:
            initialDelaySeconds: 300
            periodSeconds: 5 
            exec:
              command:
              - sh
              - -c
              - wget --spider -S "http://0.0.0.0:$PORT/healthz" 2>&1 | grep "HTTP/" | awk '{print $2}'
```

### Todo app

Consecutive upgrades were rolled out by switching the image version [`2.02`](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.02) --> [`2.08`](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.08). Coresponding versions of Service and Ingress deployments were used. Deployment manifest was ommited and replaced by a Rollout manifest.

**[Rollout](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.04/manifests/todo_app_rollout.yaml)**

```
apiVersion: argoproj.io/v1alpha1
kind: Rollout
metadata:
  namespace: todo-namespace
  name: todo-app-depl
spec:
  replicas: 1
  selector:
    matchLabels:
      app: todo-app
  strategy:
    canary:
      steps:
      - setWeight: 25
      - pause:
          duration: 30s
      - setWeight: 50
      - pause:
          duration: 30s
      - analysis:
          templates:
          - templateName: cpu-usage-rate
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

### [AnalysisTemplate](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.04/manifests/analysistemplate.yaml)

```
apiVersion: argoproj.io/v1alpha1
kind: AnalysisTemplate
metadata:
  namespace: todo-namespace
  name: cpu-usage-rate
spec:
  metrics:
  - name: cpu-usage-rate
    initialDelay: 3m # wait for 3 minutes (Readyness Probe timeout)
    interval: 20s # then check every 20 seconds
    count: 30 # 30 times, i.e. for 10 minutes
    successCondition: result <= 50
    failureCondition: result > 50 # fail immediatelly if sum exceeds threshold
    provider:
      prometheus:
        address: http://prometheus-kube-prometheus-prometheus.prometheus-namespace.svc.cluster.local:9090 
        query: |
          scalar(sum(rate(container_cpu_usage_seconds_total{namespace="todo-namespace"}[1m])))*1000
```

## Commands

![Commands for Exercise 4.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.04/Exercise_4.04_commands.png)

### Successfull Rollout

![Commands for Exercise 4.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.04/Exercise_4.04_commands2.png)

![Commands for Exercise 4.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.04/Exercise_4.04_commands3.png)

![Argo dashboard for Exercise 4.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.04/Exercise_4.04_frontend.png)

### Failed Rollout

![Commands for Exercise 4.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.04/Exercise_4.04_commands3.png)

![Commands for Exercise 4.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.04/Exercise_4.04_commands4.png)

![Argo dashboard for Exercise 4.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.04/Exercise_4.04_frontend2.png)