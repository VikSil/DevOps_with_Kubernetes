# Assignment

> This exercise doesn't rely on previous exercises. You may again choose whichever technologies you want for the implementation.
> 
> <span style="color:red">This exercise is difficult!</span>
> 
> We need a *DummySite* resource that can be used to create an HTML page from any URL.
> 
>   1. Create a "DummySite" resource that has a string property called "website_url".
>   1. Create a controller that receives a created "DummySite" object from the API
>   1. Have the controller create all of the resources that are required for the functionality.
>
> Refer to https://kubernetes.io/docs/reference/kubernetes-api/ and https://kubernetes.io/docs/reference/using-api/api-concepts/ for more information on Kubernetes API, and https://kubernetes.io/docs/reference/using-api/client-libraries/ for information about client libraries.
> 
> You may also take inspiration from the material example apps: [js](https://github.com/kubernetes-hy/material-example/tree/master/app10), [go](https://github.com/kubernetes-hy/material-example/tree/master/app10-go). Note that the JavaScript app does not quite utilize the features of [Kubernetes Client](https://github.com/kubernetes-client/javascript), but it calls the REST API directly.
> 
> Test that creating a DummySite resource with website_url "https://example.com/" creates a copy of the website. > With a more complex website your "copy" does not need to be a complete one. Eg. in https://en.wikipedia.org/wiki/Kubernetes the CSS styles can be broken:
> 
> ![wikipedia](https://devopswithkubernetes.com/static/609f88728f7f5dea2774c347ef555dde/966d8/wikipedia.webp)
>
> The controller doesn't have to work perfectly in all circumstances. The following workflow should succeed:
> 
> 1. apply role, account and binding.
> 1. apply deployment.
> 1. apply DummySite

# Solution

## Binaries

### Dummysite

- The Dummysite app is a Dockerised shell script that curls a URL that is stored in a `WEBSITE_URL` environment variable. The container is built from nginx base image and serves the website on port `80`. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.01/app/dummysite).
- Image was pushed to Docker Hub repo [viksil/dummysite:latest](https://hub.docker.com/r/viksil/dummysite/tags?name=latest).

### Controller

- The Controller app was written in JavaScript. It takes in `WEBSITE_URL` environment variable from a DummySite resource manifest and creates deployment, service and ingress for it. The website is forwarded on port `80` and accessible on loadbalancer port of the cluster (localhost port `3055` in this case). Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.01/app/controller).
- Image was pushed to Docker Hub repo [viksil/dummysite_controller:latest](https://hub.docker.com/r/viksil/dummysite_controller/tags?name=latest).
- Deleting the controller or an existing DummySite does not remove Service and Ingress respources - this could be improved in later versions of the app.

## Manifests

### [CustomResourceDefinition](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.01/manifests/customresource.yaml)

```
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: dummysites.stable.dwk
spec:
  group: stable.dwk
  scope: Namespaced
  names:
    kind: DummySite
    plural: dummysites
    singular: dummysite
    shortNames:
    - dmms
  versions:
    - name: v1
      served: true
      storage: true
      schema:
        openAPIV3Schema:
          type: object
          properties:
            spec:
              type: object
              properties:
                website_url:
                  type: string
      additionalPrinterColumns:
        - name: WEBSITE_URL
          type: string
          description: The URL of the web page
          jsonPath: .spec.website_url
```

### [ServiceAccount](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.01/manifests/serviceaccount.yaml)

```
apiVersion: v1
kind: ServiceAccount
metadata:
  name: dummysite-controller-account
```

### [Deployment](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.01/manifests/deployment.yaml)

```
apiVersion: apps/v1
kind: Deployment
metadata:
  name: dummysite-controller-dep
spec:
  replicas: 1
  selector:
    matchLabels:
      app: dummysite-controller
  template:
    metadata:
      labels:
        app: dummysite-controller
    spec:
      serviceAccountName: dummysite-controller-account
      containers:
        - name: dummysite-controller
          image: viksil/dummysite_controller:latest
          imagePullPolicy: Always
```

### [Clusterrole](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.01/manifests/clusterrole.yaml)

```
kind: ClusterRole
apiVersion: rbac.authorization.k8s.io/v1
metadata:
  name: dummysite-controller-role
rules:
#  probably can be pruned down, not all of the below are strictly necessary
- apiGroups: [""]
  resources: ["pods"]
  verbs: ["get", "list", "watch", "create", "delete"]
- apiGroups: [""]
  resources: ["services"]
  verbs: ["get", "list", "watch", "create", "delete"]
- apiGroups: ["apps"]
  resources: ["deployments"]
  verbs: ["get", "list", "watch", "create", "delete"]
- apiGroups: ["networking.k8s.io"]
  resources: ["ingresses"]
  verbs: ["get", "list", "watch", "create", "delete"]
- apiGroups: ["stable.dwk"]
  resources: ["dummysites"]
  verbs: ["get", "list", "watch", "create", "delete"]
```

### [ClusterRoleBinding](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.01/manifests/clusterrolebinding.yaml)

```
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: dummysite-rolebinding
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: dummysite-controller-role
subjects:
- kind: ServiceAccount
  name: dummysite-controller-account
  namespace: default
```


### [Example DummySite](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.01/manifests/dummysite-example.yaml)

```
apiVersion: stable.dwk/v1
kind: DummySite
metadata:
  name: example-dummysite
spec:
  website_url: https://example.com/
```

### [Kubernetes DummySite](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.01/manifests/dummysite-kubernetes-wiki.yaml)

```
apiVersion: "stable.dwk/v1"
kind: DummySite
metadata:
  name: kubernetes-dummysite
spec:
  website_url: https://en.wikipedia.org/wiki/Kubernetes
```

## Commands

![Commands for Exercise 5.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.01/Exercise_5.01_commands.png)

![Commands for Exercise 5.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.01/Exercise_5.01_commands1.png)

![Commands for Exercise 5.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.01/Exercise_5.01_commands2.png)


## Frontend

![Commands for Exercise 5.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.01/Exercise_5.01_frontend.png)

![Commands for Exercise 5.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.01/Exercise_5.01_frontend2.png)