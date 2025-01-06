# Assignment

> Setup automatic deployment for the project as well.
> 
> Hints:
> 
> - If your pod uses a Persistent Volume Claim access mode [ReadWriteOnce](https://kubernetes.io/docs/concepts/storage/persistent-volumes/#access-modes), you may need to consider the deployment [strategy](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#strategy), since the default (RollingUpdate) may cause problems. Read more from the [documentation](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#strategy). The other option is to use an [access mode](https://kubernetes.io/docs/concepts/storage/persistent-volumes/#access-modes) that allows many pods to mount the volume.
> - If you are using Ingres, remember that it expects a service to give a successful response in the path / even if the service is mapped to some other path!


# Solution

A [separate repo](https://github.com/VikSil/HU_MOOC_GKE_deployment) was set up for GKE deployment. The project was deployed iterrativelly starting with v.0.4 that was deployed to the main (`trunk`) branch. Consecutive versions were first deployed in their respective branches/namespaces for [Exercise 3.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.04). Each branch was then applied to main/default namespace via a pull request and merge.

## GitHub action

The following action was placed in `.github/workflows` directory in the main branch in order to deploy first iterration of the project to GKE.

```
name: project-v.0.4

on:
  push:

env:
  PROJECT_ID: ${{ secrets.GKE_PROJECT }}
  GKE_CLUSTER: dwk-cluster
  GKE_ZONE: europe-west1-c
  IMAGE_APP: todo_app
  BRANCH: ${{ github.ref_name }}

jobs:
  build-publish-deploy:
    name: Build, Publish and Deploy
    runs-on: ubuntu-latest

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: 'Log into Google Cloud'
        uses: google-github-actions/auth@v2
        with:
          credentials_json: '${{ secrets.GKE_SA_KEY }}'

      - name: 'Set up Cloud SDK'
        uses: google-github-actions/setup-gcloud@v2

      - name: 'Use gcloud CLI'
        run: gcloud info

      - name: 'Enable Google Container Registry'
        run: gcloud --quiet auth configure-docker

      - name: 'Get GKE credentials'
        uses: 'google-github-actions/get-gke-credentials@v2'
        with:
          cluster_name: '${{ env.GKE_CLUSTER }}'
          project_id: '${{ env.PROJECT_ID }}'
          location: '${{ env.GKE_ZONE }}'

      - name: Build and publish
        run: |-
          docker build -t "gcr.io/$PROJECT_ID/$IMAGE_APP:$BRANCH-$GITHUB_SHA" ./todo_app
          docker push "gcr.io/$PROJECT_ID/$IMAGE_APP:$BRANCH-$GITHUB_SHA"

      - name: Set up Kustomize
        uses: imranismail/setup-kustomize@v2

      - name: Deploy
        run: |-
          kustomize edit set image PROJECT/IMAGE_APP=gcr.io/$PROJECT_ID/$IMAGE_APP:${GITHUB_REF#refs/heads/}-$GITHUB_SHA
          kustomize build . | kubectl apply -f -
          kubectl rollout status deployment $DEPLOYMENT
          kubectl get services -o wide
```

## Commands

A cluster with two nodes was created with virtual machine type [n4-standard-2](https://cloud.google.com/compute/docs/general-purpose-machines#n4_series). This type of VM was chosen to meet the demand for resources required by the Rust yew framework on the frontend, since yew app compilation needs to happen on the node at startup. 

![Commands for Exercise 3.03](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.03/Exercise_3.03_commands.png)

![Commands for Exercise 3.03](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.03/Exercise_3.03_commands2.png)

![Commands for Exercise 3.03](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.03/Exercise_3.03_commands3.png)

## Frontend

Since v.0.4 does not have an ingress yet, front end was accessed via Service port in Lense:

![Commands for Exercise 3.03](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.03/Exercise_3.03_deployment.png)