# Assignment

> Speaking of updating. Our todo application could use "Done" field for todos that are already done. It should be a PUT request to `/todos/<id>`.

# Solution

## Binaries

### Todo backend

- Application was modified by adding a PUT request handling to the `\todos` route. The applicaiton expects the todo `id` and `completed` fields, and sets the value of the `completed` field in the database for the received id accordingly. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.05/app/todo_backend).
- Image was pushed to Docker Hub repo [viksil/todo_backend:4.05](https://hub.docker.com/r/viksil/todo_backend/tags?name=4.05).

### Todo app

- Application was modified by replacing the buller points in the unordered list with buttons. The button displays text `TODO` or `DONE` depending on the state of the todo item. Pressing the button, sends a PUT request to the backend and flips the state of the coresponding todo item. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part4/Exercise_4.05/app/todo_app).
- Image was pushed to Docker Hub repo [viksil/todo_app:4.05](https://hub.docker.com/r/viksil/todo_app/tags?name=4.05).

## Manifests

- Rollout manifests for Todo backend and Todo app were modified by changing the image version to `4.05`.
- ConfigMap for Todo app from from [Exercise 3.08](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.08) was used, and environment variable was copied from the Deployment manifest into the Rollout manifest .

## Front end

![Front end for Exercise 4.05](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.05/Exercise_4.05_frontend.png)
