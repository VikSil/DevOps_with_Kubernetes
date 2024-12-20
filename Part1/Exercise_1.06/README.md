## Assignment

> 
> Use a NodePort Service to enable access to the project.

## Solution

### Service Manifest

```
apiVersion: v1
kind: Service
metadata:
  name: todo-app-service
spec:
  type: NodePort
  selector:
    app: todo-app
  ports:
    - name: http
      nodePort: 30088
      protocol: TCP
      port: 1234
      targetPort: 8088
```

### Commands

![Commands for Exercise 1.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.06/Exercise_1.06_commands.png)

### GET Request to Forwarded Port

![Landing Page for Exercise 1.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.06/Exercise_1.06_landing_page.png)
