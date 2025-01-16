# Assignment

> Move the Project to use GitOps.
> 
>  - Create two separate environments, production and staging that are in their own namespaces
>  - Each commit to the main branch should result in deployment to the staging environment
>  - Each tagged commit results in deployment to the production environment
>  - In staging the broadcaster just logs all the messages, it does not forward those to any external service
>  - In staging database is not backed up
>  - You may assume that secrets are readily applied outside of the ArgoCD
>  - Bonus: use different repositories for the code and configurations

# Solution

## Binaries

No changes were made to the binaries.

## Repositories

Two new repositories were created for this exercise:
- [Configuration repository](https://github.com/VikSil/HU_MOOC_todo_app_config) containing base and patch `.yaml` files for staging and production environments.
- [Source repository](https://github.com/VikSil/HU_MOOC_todo_app) containing code for the todo_backend and todo_app, as well as GitHub action that builds the binaries, pushes them to DockerHub and updates kustomization files in the configuration repository.

## Manifests

### Base

All of the existing manifests were placed in the [`base` directory](https://github.com/VikSil/HU_MOOC_todo_app_config/blob/trunk/base) along with the `kustomization.yaml` file. The following alterations were introduced to the base manifests from previous exercises:

 - `BACKEND_HOST` environment variable from `todo-app-cfgmap` was introduced into the `cronjob.yaml`, since CronJob is run in a separate pod from todo_app.
 - `spec.ports.port` was switched to the default HTTP port `80` in all Service manifests.
 - All ports in `shared_ingress.yaml` were switched to the default HTTP port `80`.

### Overlays

Two overlay deployments were introduced: [prod](https://github.com/VikSil/HU_MOOC_todo_app_config/tree/trunk/overlays/prod) and [staging](https://github.com/VikSil/HU_MOOC_todo_app_config/tree/trunk/overlays/staging):

- staging `.yaml` files are sourced from the base and are deployed when code changes are pushed to the main (`trunk`) branch of the code repository. Since [Exercise 4.06](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.06) was skipped due to course deadline, broadcaster is not included in either overlay.
- production overlay contains `.yaml` files for database backup CronJob and patches todo_app ConfigMap with backend host IP address in the production environment.

## Commands

![Commands for Exercise 4.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.08/Exercise_4.08_commands.png)

![Commands for Exercise 4.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.08/Exercise_4.08_commands2.png)

![Commands for Exercise 4.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.08/Exercise_4.08_commands3.png)

## Frontend

![Actions for Exercise 4.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.08/Exercise_4.08_actions.png)

![Frontend for Exercise 4.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.08/Exercise_4.08_frontend.png)

![Frontend for Exercise 4.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.08/Exercise_4.08_frontend3.png)

![Frontend for Exercise 4.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.08/Exercise_4.08_frontend2.png)

![Frontend for Exercise 4.08](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.08/Exercise_4.08_frontend4.png)
