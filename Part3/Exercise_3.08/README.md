# Assignment

> Set sensible resource limits for the project. The exact values are not important. Test what works.

# Solution

## Manifests

### [Todo app deployment](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.08/manifests/todo_app_deployment.yaml)

Frontend is built using Rust `yew` crate. This crate requires two-stage compilation: at the point of deployment (when the docker image is built) and at the point of first run (inside the pod). This means that the pod is very resource hungry, compared to other possible solutions (notably those built in JavaScript or Python). Monitoring using Lense revealed that first-time runs can consume up to 1.9 CPUs and close to 600Mi of memory. Additionally, setting the limits too low caused the pod to enter a failed state without recovery with logs abruptly stopping mid-compilation. As a compromise, to allow the pod to start up while not draining all of the resources available on the node, the limits were set at 0.8 CPU and 500Mi of memory. Once initialised the usage of CPU fell to less than 1% of CPU and around 260Mi memory usage. 

```
...
          resources:
            limits:
              cpu: "800m"
              memory: "500Mi"
...

```

### [Todo backend deployment](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.08/manifests/todo_backend_deployment.yaml)

Unlike the frontend, the backend is built with Rust `axum` crate, which is compiled by the GitHub engine building the Docker image during deployment. The resulting executable is then run inside the GKE pod. This being a pre-compiled execuable, that has been optimised by the Rust compiler, allows for a very low resource consumption. The limits were set to 5% of CPU and 50Mi of memory, although it may be possible to set them even lower. 

```
...
          resources:
            limits:
              cpu: "50m"
              memory: "50Mi"
....

```