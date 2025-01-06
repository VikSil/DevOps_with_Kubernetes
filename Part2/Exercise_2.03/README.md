## Assignment

> Create a namespace for the applications in the exercises. Move the "Log output" and "Ping-pong" to that namespace and use that in the future for all of the exercises. You can follow the material in the default namespace.

## Solution

### Binaries

The same applications as for [Exercise 2.01](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.01) were used.


### Manifests

**Applications and volume**

The same manifests as for [Exercise 2.01](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.01) were used, with namespace declaration added at the top of the `metadata` section of all `.yaml` files.

```
metadata:
  namespace: pinglog-namespace
```

[**Namespace**](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.03/pinglog_namespace.yaml)

```
apiVersion: v1
kind: Namespace
metadata:
  name: pinglog-namespace
  labels:
    name: pinglog-namespace
```

### Commands

This exercise used the same cluster that was created for [Exercise 2.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.02/Exercise_2.02_commands.png).

![Commands for Exercise 2.03](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.03/Exercise_2.03_commands.png)
