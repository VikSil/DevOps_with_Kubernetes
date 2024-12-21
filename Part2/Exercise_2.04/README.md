## Assignment

> Create a namespace for the project and move everything related to the project to that namespace.

## Solution

### Binaries

The same applications as for [Exercise 2.02](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.02) were used.


### Manifests

The same manifests as for [Exercise 2.02](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.02) were used, with namespace declaration added at the top of the `metadata` section of all `.yaml` files.

```
metadata:
  namespace: todo-namespace
```

### Commands

- This exercise used the same cluster that was created for [Exercise 2.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.02/Exercise_2.02_commands.png). 

- Namespace was create manually by running a command:

      kubectl create namespace todo-namespace 

![Commands for Exercise 2.04](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.04/Exercise_2.04_commands.png)

