## Assignment

> 
> Switch to using Ingress instead of NodePort to access the project. You can delete the ingress of the "Log output" application so they don't interfere with this exercise. We'll look more into paths and routing in the next exercise and at that point you can configure project to run with the "Log output" application side by side.

## Solution

### Binaries

- [Latest version](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.02/app) of the app developed for Exercise 1.02 was used. By default application listens to port 3030, if no `PORT` environment variable is provided.
- [Image](https://hub.docker.com/r/viksil/todo_app/tags?name=1.02) was pulled from Docker Hub.

### Manifests

[**Deployment**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.08/manifests/deployment.yaml)

Unchanged deployment manifest from [Exercise 1.04](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.04) was used. It sets `PORT` environment variable to `8088` and specifies the target Docker hub image.


[**Service**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.08/manifests/service.yaml)

```
apiVersion: v1
kind: Service
metadata:
  name: todo-app-service
spec:
  type: ClusterIP
  selector:
    app: todo-app
  ports:
    - port: 3012
      protocol: TCP
      targetPort: 8088
```

[**Ingress**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.08/manifests/Ingress.yaml)

```
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: todo-app-ingress
spec:
  rules:
  - http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: todo-app-service
            port:
              number: 3012
```

### Commands

- This exercise used the same cluster that was created for [previous exercise](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.07/Exercise_1.07_commands.png). Loadbalancer HTTP port `80` is exposed to outside requests on `localhost:3011`.

- Ingress for `log-output` app was removed to prevent a clash.

![Commands for Exercise 1.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.08/Exercise_1.08_commands.png)

### GET Request to Forwarded Port

![GET Request for Exercise 1.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.08/Exercise_1.08_get_request.png)
