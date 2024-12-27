# Assignment

> Deploy Ping-pong application into GKE.
> 
> In this exercise use a LoadBalancer service to expose the service.
> 
> If your Postgres logs say
> 
>     initdb: error: directory "/var/lib/postgresql/data" exists but is not empty
>     It contains a lost+found directory, perhaps due to it being a mount point.
>     Using a mount point directly as the data directory is not recommended.
>     Create a subdirectory under the mount point.
> 
> you can add *subPath* configuration:
> 
> **statefulset.yaml**
> 
>     # ...
>     volumeMounts:
>     - name: data
>       mountPath: /var/lib/postgresql/data
>       subPath: postgres
>     # ...
>
> This will create a Postgres directory where the data will reside. subPaths also make it possible to use single volume for multiple purposes.

# Solution

## Manifests

Namespace declaration was added at the top of the `metadata` section of all `.yaml` file.

```
metadata:
  namespace: pinglog-namespace
```

### Postgres

- Unchanged manifests for `ConfigMap`, `Service` and `Secret` from [Exercise 2.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07) were used.
- Manifest for `PersistentVolume` was ommited, since GKE automatically provisions volumes based on the `PersistentVolumeClaim`.
- Manifest for `StatefulSet` was changed by adding `spec.template.spec.containers.volumeMounts.subPath`.
- Manifest for `PersistentVolumeClaim` was changed by removing `spec.storageClassName`.

**[StatefulSet](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.01/manifests/postgres_pod/statefulset.yaml)**

```
...
          volumeMounts:
            - mountPath: /var/lib/postgresql/data
              name: pingpong
              subPath: postgres
      volumes:
        - name: pingpong
          persistentVolumeClaim:
            claimName: postgres-volume-claim
```

**[PersistentVolumeClaim](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.01/manifests/postgres_pod/volumeclaim.yaml)**

```
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  namespace: pinglog-namespace
  name: postgres-volume-claim
  labels:
    app: postgres
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 1Gi
```


### Pingpong

- Unchanged manifests for `Deployment` and `Secret` from [Exercise 2.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07) were used.
- Manifest for `Service` was changed by switching it to `spec.type:LoadBalancer` and switching port to 80, in order to senf HTTP requests.

**[Service](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.01/manifests/pingpong_pod/service.yaml)**

```
apiVersion: v1
kind: Service
metadata:
  name: pingpong-service
spec:
  type: LoadBalancer
  selector:
    app: pingpong
  ports:
    - port: 80
      protocol: TCP
      targetPort: 3033
```


## Commands

![Commands for Exercise 3.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.01/Exercise_3.01_commands.png)

![Commands for Exercise 3.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.01/Exercise_3.01_commands2.png)

![Commands for Exercise 3.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.01/Exercise_3.01_commands3.png)

![Commands for Exercise 3.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.01/Exercise_3.01_commands4.png)
