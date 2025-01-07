# Assignment

> Improve the deployment so that each branch creates a separate environment. The main branch should still be deployed in the *default* namespace.


# Solution

A [separate repo](https://github.com/VikSil/HU_MOOC_GKE_deployment) was set up for GKE deployment. The project was deployed iterrativelly starting with v.0.4 that was deployed to the main (`trunk`) branch. Consecutive versions were first deployed in their respective branches/namespaces. Each branch was then applied to main/default namespace via a pull request and merge. The following alterations were made to the solutions developed for [Part2](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2):

- [Version 1.1](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.04) was skipped, since it adds namespace directly into the `.yaml` files and the purpose of this exercise is to control namespaces via repo branch.
- Namespace definition was removed from all `.yaml` files in consecutive versions. 
- Pod deployment strategy was added to the Deployment manifest for todo_backend:

      spec:
        strategy:
        type: Recreate

- Code for todo_app was modified to use `BACKEND_HOST` environment variable as connection string for backend starting version 1.1.
- Starting version 1.1. Deployment manifest for todo_app was altered by adding an environment variable `BACKEND_HOST` sourced from a ConfigMap.
- Starting version 1.1. ConfigMap was added to todo_app, containing `backend_host` data string. After deployment, once ingress was established, the ConfigMap was updated with the external IP of the ingress and todo_app was restarted to apply the new connection string.
- [Version 1.3](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.10) was added later as part of the [Exercise 3.10](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.10).

## GitHub action

`Deploy` section of the GitHub action was modified as follows to take into account the branch where changes are being pushed to:

```
      - name: Deploy
        run: |-
          NAMESPACE=${GITHUB_REF#refs/heads/}
          if [[ "$NAMESPACE" == "trunk" ]]; then
            NAMESPACE="default"
          fi
          kubectl create namespace $NAMESPACE || true
          kubectl config set-context --current --namespace=$NAMESPACE
          kustomize edit set namespace $NAMESPACE
          kustomize edit set image PROJECT/IMAGE_APP=gcr.io/$PROJECT_ID/$IMAGE_APP:${GITHUB_REF#refs/heads/}-$GITHUB_SHA
          kustomize build . | kubectl apply -f -
          kubectl rollout status deployment $DEPLOYMENT
          kubectl get services -o wide
```

## Deployment

![Commits for Exercise 3.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.04/Exercise_3.04_commits.png)

![Actions for Exercise 3.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.04/Exercise_3.04_actions.png)

![Branches for Exercise 3.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.04/Exercise_3.04_branches.png)

![Namespaces for Exercise 3.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.04/Exercise_3.04_namespaces.png)

![Deployments for Exercise 3.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.04/Exercise_3.04_deployments.png)

![Default ingress for Exercise 3.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.04/Exercise_3.04_default_namespace.png)

![Frontend for Exercise 3.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.04/Exercise_3.04_frontend.png)


