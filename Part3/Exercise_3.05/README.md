# Assignment

> Finally, create a new workflow so that deleting a branch deletes the environment.


# Solution

## Manifest

In order for a github action to be triggered when a branch in a repo is deleted the following `.yaml` file has to be placed in `.github/workflows` directory in the main branch. Placing the action in the branch that will be deleted does nothing, since the action will be deleted with the branch. There is no need to check which branch is being deleted, since it is not possible to delete the default (main) branch in git. 

```
name: project-delete-branch

env:
  PROJECT_ID: ${{ secrets.GKE_PROJECT }}
  GKE_CLUSTER: dwk-cluster
  GKE_ZONE: europe-west2-c

on:
  delete:

jobs:
  build-publish-deploy:
    name: Delete branch
    runs-on: ubuntu-latest

    steps:
      - name: 'Log into Google Cloud'
        uses: google-github-actions/auth@v2
        with:
          credentials_json: '${{ secrets.GKE_SA_KEY }}'

      - name: 'Set up Cloud SDK'
        uses: google-github-actions/setup-gcloud@v2

      - name: 'Use gcloud CLI'
        run: gcloud info

      - name: 'Get GKE credentials'
        uses: 'google-github-actions/get-gke-credentials@v2'
        with:
          cluster_name: '${{ env.GKE_CLUSTER }}'
          project_id: '${{ env.PROJECT_ID }}'
          location: '${{ env.GKE_ZONE }}'

      - name: 'Delete namespace'
        run: |-
          kubectl delete all --all -n ${{ github.event.ref }}
          kubectl delete namespace ${{ github.event.ref }}
```

## Commands

![Commands for Exercise 3.05](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.05/Exercise_3.05_commands.png)

![Commands for Exercise 3.05](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.05/Exercise_3.05_commands2.png)

![Commands for Exercise 3.05](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.05/Exercise_3.05_commands3.png)

![Commands for Exercise 3.05](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.05/Exercise_3.05_commands4.png)
