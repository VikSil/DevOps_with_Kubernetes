# Assignment

> Move the Ping-pong application to use GitOps so that when you commit to the repository, the application is automatically updated.

# Solution

## Binaries

No changes were made to the binaries.

## Repository

A [new repository](https://github.com/VikSil/HU_MOOC_pingpong_app) was set up to store Pingpong app, Log output and Log output reader apps.

## Manifests

### [Postgres](https://github.com/VikSil/HU_MOOC_pingpong_app/tree/trunk/manifests/pingpong_postgres)

- Unchanged manifests from [Exercise 2.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07) were used.

### [Pingpong](https://github.com/VikSil/HU_MOOC_pingpong_app/tree/trunk/manifests/pingpong), [Log output](https://github.com/VikSil/HU_MOOC_pingpong_app/tree/trunk/manifests/log_output) and [Log reader](https://github.com/VikSil/HU_MOOC_pingpong_app/tree/trunk/manifests/log_output_reader)

- `spec.type` was switched to `LoadBalancer` in all Service manifests.
- `spec.ports.port` was switched to the default HTTP port `80` in all Service manifests.
- Otherwise unchanged manifests from [Exercise 4.01](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.01) were used.

### [Ingress](https://github.com/VikSil/HU_MOOC_pingpong_app/blob/trunk/manifests/shared_ingress.yaml)

- All ports were switched to the default HTTP port `80`, otherwise unchanged manifest from [Exercise 4.01](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.01) was used.

### [Kustomization](https://github.com/VikSil/HU_MOOC_pingpong_app/blob/trunk/kustomization.yaml)

The `kustomization.yaml` file is dynamically amended by the GitHub action when changes are pushed to the repo. The original version of the file can be found [here](https://github.com/VikSil/HU_MOOC_pingpong_app/blob/b892402dbf19caedc895b20b3ea9e5165159f74b/kustomization.yaml).

### [GitHub action](https://github.com/VikSil/HU_MOOC_pingpong_app/blob/trunk/.github/workflows/build.yaml)

```
name: Build and publish application

on:
  push:

jobs:
  build-publish:
    name: Build, Publish
    runs-on: ubuntu-latest

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Login to Docker Hub
        uses: docker/login-action@v3
        with:
          username: ${{ secrets.DOCKERHUB_USERNAME }}
          password: ${{ secrets.DOCKERHUB_TOKEN }}

      - name: Build and publish backend
        run: |-
          docker build --tag "viksil/pingpong:$GITHUB_SHA" ./apps/pingpong
          docker push "viksil/pingpong:$GITHUB_SHA"
          docker build --tag "viksil/log_output:$GITHUB_SHA" ./apps/log_output
          docker push "viksil/log_output:$GITHUB_SHA"
          docker build --tag "viksil/log_output_reader:$GITHUB_SHA" ./apps/log_output_reader
          docker push "viksil/log_output_reader:$GITHUB_SHA"
  
      - name: Set up Kustomize
        uses: imranismail/setup-kustomize@v2

      - name: Use right image
        run: kustomize edit set image PROJECT/IMAGE=viksil/pingpong:$GITHUB_SHA
      - name: Use right image1
        run: kustomize edit set image PROJECT1/IMAGE1=viksil/log_output:$GITHUB_SHA
      - name: Use right image2
        run: kustomize edit set image PROJECT2/IMAGE2=viksil/log_output_reader:$GITHUB_SHA

      - name: commit kustomization.yaml to GitHub
        uses: EndBug/add-and-commit@v9
        with:
          add: 'kustomization.yaml'
          message: New version released ${{ github.sha }}
```

## Setup and commands

![Commands for Exercise 4.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.07/Exercise_4.07_commands.png)

![Commands for Exercise 4.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.07/Exercise_4.07_setup.png)

![Commands for Exercise 4.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.07/Exercise_4.07_setup2.png)

![Commands for Exercise 4.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.07/Exercise_4.07_frontend.png)

![Commands for Exercise 4.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.07/Exercise_4.07_commands2.png)

![Commands for Exercise 4.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.07/Exercise_4.07_frontend2.png)

![Commands for Exercise 4.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.07/Exercise_4.07_frontend3.png)

![Commands for Exercise 4.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.07/Exercise_4.07_frontend4.png)