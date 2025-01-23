# Assignment

> Enable the Linkerd service mesh for The Project.
> 
> Deployments are mostly trivial to move to Linkerd. Read [this](https://linkerd.io/2/tasks/adding-your-service/), and add the modified manifests (through Linkerd inject) to the repository for submission.

# Solution

## Commands

![Commands for Exercise 5.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.02/Exercise_5.02_commands.png)

![Commands for Exercise 5.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.02/Exercise_5.02_commands2.png)

![Commands for Exercise 5.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.02/Exercise_5.02_commands3.png)

![Commands for Exercise 5.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.02/Exercise_5.02_commands4.png)


## Manifests

### [Postgres](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.02/manifests/postgres.yaml)

Annotation was added into the postgres Deployment `.yaml` file manually due to Linkerd [protocol detection timeout](https://linkerd.io/2.17/features/protocol-detection/).

```
apiVersion: apps/v1
kind: StatefulSet
metadata:
  annotations:
    kubectl.kubernetes.io/last-applied-configuration: |
      {"apiVersion":"apps/v1","kind":"StatefulSet","metadata":{"annotations":{},"name":"postgres","namespace":"todo-namespace"},"spec":{"replicas":1,"selector":{"matchLabels":{"app":"postgres"}},"serviceName":"postgres","template":{"metadata":{"annotations":{"linkerd.io/inject":"enabled"},"labels":{"app":"postgres"}},"spec":{"containers":[{"env":[{"name":"POSTGRES_PASSWORD","valueFrom":{"secretKeyRef":{"key":"POSTGRES_PASSWORD","name":"postgres-secret"}}}],"envFrom":[{"configMapRef":{"name":"postgres-cfgmap"}}],"image":"postgres:10.1","imagePullPolicy":"IfNotPresent","name":"postgres","ports":[{"containerPort":5432}],"volumeMounts":[{"mountPath":"/var/lib/postgresql/data","name":"todo"}]}],"volumes":[{"name":"todo","persistentVolumeClaim":{"claimName":"todo-volume-claim"}}]}}}}
  creationTimestamp: "2025-01-17T19:39:26Z"
  generation: 1
  name: postgres
  namespace: todo-namespace
  resourceVersion: "11469"
  uid: a74ccbfe-6c97-47ce-bec8-6ddecd74431e
spec:
  persistentVolumeClaimRetentionPolicy:
    whenDeleted: Retain
    whenScaled: Retain
  podManagementPolicy: OrderedReady
  replicas: 1
  revisionHistoryLimit: 10
  selector:
    matchLabels:
      app: postgres
  serviceName: postgres
  template:
    metadata:
      annotations:
        linkerd.io/inject: enabled
      creationTimestamp: null
      labels:
        app: postgres
    spec:
      containers:
      - env:
        - name: POSTGRES_PASSWORD
          valueFrom:
            secretKeyRef:
              key: POSTGRES_PASSWORD
              name: postgres-secret
        envFrom:
        - configMapRef:
            name: postgres-cfgmap
        image: postgres:10.1
        imagePullPolicy: IfNotPresent
        name: postgres
        ports:
        - containerPort: 5432
          protocol: TCP
        resources: {}
        terminationMessagePath: /dev/termination-log
        terminationMessagePolicy: File
        volumeMounts:
        - mountPath: /var/lib/postgresql/data
          name: todo
      dnsPolicy: ClusterFirst
      restartPolicy: Always
      schedulerName: default-scheduler
      securityContext: {}
      terminationGracePeriodSeconds: 30
      volumes:
      - name: todo
        persistentVolumeClaim:
          claimName: todo-volume-claim
  updateStrategy:
    rollingUpdate:
      partition: 0
    type: RollingUpdate
status:
  availableReplicas: 1
  collisionCount: 0
  currentReplicas: 1
  currentRevision: postgres-7697bc578b
  observedGeneration: 1
  readyReplicas: 1
  replicas: 1
  updateRevision: postgres-7697bc578b
  updatedReplicas: 1
```


### [Todo backend](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.02/manifests/todo-backend-depl.yaml)



Annotation `config.linkerd.io/enable-external-profiles: "true"` was added into the postgres Deployment `.yaml` file manually due to Linkerd [protocol detection](https://linkerd.io/2.17/features/protocol-detection/).

```
apiVersion: apps/v1
kind: Deployment
metadata:
  annotations:
    deployment.kubernetes.io/revision: "2"
    kubectl.kubernetes.io/last-applied-configuration: |
      {"apiVersion":"apps/v1","kind":"Deployment","metadata":{"annotations":{"deployment.kubernetes.io/revision":"1"},"generation":1,"name":"todo-backend-depl","namespace":"todo-namespace","resourceVersion":"11735","uid":"2372cb91-0fc4-46c8-9005-3d1f6f93c63c"},"spec":{"progressDeadlineSeconds":600,"replicas":1,"revisionHistoryLimit":10,"selector":{"matchLabels":{"app":"todo-backend"}},"strategy":{"rollingUpdate":{"maxSurge":"25%","maxUnavailable":"25%"},"type":"RollingUpdate"},"template":{"metadata":{"annotations":{"config.linkerd.io/enable-external-profiles":"true","linkerd.io/inject":"enabled"},"labels":{"app":"todo-backend"}},"spec":{"containers":[{"env":[{"name":"PORT","valueFrom":{"configMapKeyRef":{"key":"port","name":"todo-cfgmap"}}},{"name":"DB_URL","valueFrom":{"secretKeyRef":{"key":"DB_URL","name":"todo-secret"}}}],"image":"viksil/todo_backend:4.05","imagePullPolicy":"Always","livenessProbe":{"exec":{"command":["sh","-c","wget --spider -S \"http://0.0.0.0:$PORT/healthz\" 2\u003e\u00261 | grep \"HTTP/\" | awk '{print $2}'"]},"failureThreshold":3,"initialDelaySeconds":300,"periodSeconds":5,"successThreshold":1,"timeoutSeconds":1},"name":"todo-backend","readinessProbe":{"exec":{"command":["sh","-c","wget --spider -S \"http://0.0.0.0:$PORT/healthz\" 2\u003e\u00261 | grep \"HTTP/\" | awk '{print $2}'"]},"failureThreshold":3,"initialDelaySeconds":180,"periodSeconds":10,"successThreshold":1,"timeoutSeconds":1},"terminationMessagePath":"/dev/termination-log","terminationMessagePolicy":"File"}],"dnsPolicy":"ClusterFirst","restartPolicy":"Always","schedulerName":"default-scheduler","securityContext":{},"terminationGracePeriodSeconds":30}}},"status":{"availableReplicas":1,"conditions":[{"message":"Deployment has minimum availability.","reason":"MinimumReplicasAvailable","status":"True","type":"Available"},{"message":"ReplicaSet \"todo-backend-depl-6c49c7d5\" has successfully progressed.","reason":"NewReplicaSetAvailable","status":"True","type":"Progressing"}],"observedGeneration":1,"readyReplicas":1,"replicas":1,"updatedReplicas":1}}
  creationTimestamp: "2025-01-17T19:39:35Z"
  generation: 2
  name: todo-backend-depl
  namespace: todo-namespace
  resourceVersion: "18103"
  uid: 2372cb91-0fc4-46c8-9005-3d1f6f93c63c
spec:
  progressDeadlineSeconds: 600
  replicas: 1
  revisionHistoryLimit: 10
  selector:
    matchLabels:
      app: todo-backend
  strategy:
    rollingUpdate:
      maxSurge: 25%
      maxUnavailable: 25%
    type: RollingUpdate
  template:
    metadata:
      annotations:
        config.linkerd.io/enable-external-profiles: "true"
        linkerd.io/inject: enabled
      creationTimestamp: null
      labels:
        app: todo-backend
    spec:
      containers:
      - env:
        - name: PORT
          valueFrom:
            configMapKeyRef:
              key: port
              name: todo-cfgmap
        - name: DB_URL
          valueFrom:
            secretKeyRef:
              key: DB_URL
              name: todo-secret
        image: viksil/todo_backend:4.05
        imagePullPolicy: Always
        livenessProbe:
          exec:
            command:
            - sh
            - -c
            - wget --spider -S "http://0.0.0.0:$PORT/healthz" 2>&1 | grep "HTTP/"
              | awk '{print $2}'
          failureThreshold: 3
          initialDelaySeconds: 300
          periodSeconds: 5
          successThreshold: 1
          timeoutSeconds: 1
        name: todo-backend
        readinessProbe:
          exec:
            command:
            - sh
            - -c
            - wget --spider -S "http://0.0.0.0:$PORT/healthz" 2>&1 | grep "HTTP/"
              | awk '{print $2}'
          failureThreshold: 3
          initialDelaySeconds: 180
          periodSeconds: 10
          successThreshold: 1
          timeoutSeconds: 1
        resources: {}
        terminationMessagePath: /dev/termination-log
        terminationMessagePolicy: File
      dnsPolicy: ClusterFirst
      restartPolicy: Always
      schedulerName: default-scheduler
      securityContext: {}
      terminationGracePeriodSeconds: 30
status:
  availableReplicas: 1
  conditions:
  - lastTransitionTime: "2025-01-17T19:42:46Z"
    lastUpdateTime: "2025-01-17T19:42:46Z"
    message: Deployment has minimum availability.
    reason: MinimumReplicasAvailable
    status: "True"
    type: Available
  - lastTransitionTime: "2025-01-17T19:39:35Z"
    lastUpdateTime: "2025-01-17T21:18:52Z"
    message: ReplicaSet "todo-backend-depl-7c5ffd6c7c" is progressing.
    reason: ReplicaSetUpdated
    status: "True"
    type: Progressing
  observedGeneration: 2
  readyReplicas: 1
  replicas: 2
  unavailableReplicas: 1
  updatedReplicas: 1
```


### [Todo app](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.02/manifests/todo-app-depl.yaml)

```
apiVersion: apps/v1
kind: Deployment
metadata:
  annotations:
    deployment.kubernetes.io/revision: "2"
    kubectl.kubernetes.io/last-applied-configuration: |
      {"apiVersion":"apps/v1","kind":"Deployment","metadata":{"annotations":{"deployment.kubernetes.io/revision":"1"},"generation":1,"name":"todo-app-depl","namespace":"todo-namespace","resourceVersion":"11541","uid":"3c030e60-39bd-4819-82bb-170869a72fce"},"spec":{"progressDeadlineSeconds":600,"replicas":1,"revisionHistoryLimit":10,"selector":{"matchLabels":{"app":"todo-app"}},"strategy":{"rollingUpdate":{"maxSurge":"25%","maxUnavailable":"25%"},"type":"RollingUpdate"},"template":{"metadata":{"annotations":{"linkerd.io/inject":"enabled"},"labels":{"app":"todo-app"}},"spec":{"containers":[{"env":[{"name":"PORT","valueFrom":{"configMapKeyRef":{"key":"port","name":"todo-cfgmap"}}},{"name":"BACKEND_HOST","valueFrom":{"configMapKeyRef":{"key":"backend_host","name":"todo-cfgmap"}}}],"image":"viksil/todo_app:4.05","imagePullPolicy":"Always","name":"todo-app","terminationMessagePath":"/dev/termination-log","terminationMessagePolicy":"File"}],"dnsPolicy":"ClusterFirst","restartPolicy":"Always","schedulerName":"default-scheduler","securityContext":{},"terminationGracePeriodSeconds":30}}},"status":{"availableReplicas":1,"conditions":[{"message":"Deployment has minimum availability.","reason":"MinimumReplicasAvailable","status":"True","type":"Available"},{"message":"ReplicaSet \"todo-app-depl-894568685\" has successfully progressed.","reason":"NewReplicaSetAvailable","status":"True","type":"Progressing"}],"observedGeneration":1,"readyReplicas":1,"replicas":1,"updatedReplicas":1}}
  creationTimestamp: "2025-01-17T19:39:43Z"
  generation: 2
  name: todo-app-depl
  namespace: todo-namespace
  resourceVersion: "18096"
  uid: 3c030e60-39bd-4819-82bb-170869a72fce
spec:
  progressDeadlineSeconds: 600
  replicas: 1
  revisionHistoryLimit: 10
  selector:
    matchLabels:
      app: todo-app
  strategy:
    rollingUpdate:
      maxSurge: 25%
      maxUnavailable: 25%
    type: RollingUpdate
  template:
    metadata:
      annotations:
        linkerd.io/inject: enabled
      creationTimestamp: null
      labels:
        app: todo-app
    spec:
      containers:
      - env:
        - name: PORT
          valueFrom:
            configMapKeyRef:
              key: port
              name: todo-cfgmap
        - name: BACKEND_HOST
          valueFrom:
            configMapKeyRef:
              key: backend_host
              name: todo-cfgmap
        image: viksil/todo_app:4.05
        imagePullPolicy: Always
        name: todo-app
        resources: {}
        terminationMessagePath: /dev/termination-log
        terminationMessagePolicy: File
      dnsPolicy: ClusterFirst
      restartPolicy: Always
      schedulerName: default-scheduler
      securityContext: {}
      terminationGracePeriodSeconds: 30
status:
  availableReplicas: 1
  conditions:
  - lastTransitionTime: "2025-01-17T19:39:45Z"
    lastUpdateTime: "2025-01-17T19:39:45Z"
    message: Deployment has minimum availability.
    reason: MinimumReplicasAvailable
    status: "True"
    type: Available
  - lastTransitionTime: "2025-01-17T19:39:43Z"
    lastUpdateTime: "2025-01-17T21:18:52Z"
    message: ReplicaSet "todo-app-depl-5999f95598" is progressing.
    reason: ReplicaSetUpdated
    status: "True"
    type: Progressing
  observedGeneration: 2
  readyReplicas: 1
  replicas: 2
  unavailableReplicas: 1
  updatedReplicas: 1
```