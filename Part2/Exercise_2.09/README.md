# Assignment

> Create a CronJob that generates a new todo every hour to remind you to do 'Read < URL >'.
> 
> Where < URL > is a Wikipedia article that was decided by the job randomly. It does not have to be a hyperlink, the user can copy-paste the URL from the todo.
> 
> https://en.wikipedia.org/wiki/Special:Random responds with a redirect to a random Wikipedia page so you can ask it to provide a random article for you to read. TIP: Check location header

# Solution

## Manifests

For the solution of this exercise a `ConfigMap` was created with a definition of a batch script to retrieve a random Wikiedia page and send a `POST` request with that page to `todo-backend-service` endpoint. The `ConfigMap` was then mounted as a volume in a `CronJob`.

### ConfigMap

```
apiVersion: v1
kind: ConfigMap
metadata:
  namespace: todo-namespace
  name: cronjob-configmap
data:
  cronjob-script.sh: |
    #!/bin/bash

    REDIRECT=`curl -Ls -w %{url_effective} -o /dev/null https://en.wikipedia.org/wiki/Special:Random`
    echo "Read ${REDIRECT}"
    echo "{\"title\":\"Read ${REDIRECT}\"}" >req.json
    curl -X POST -H 'Content-Type: application/json' -d @req.json http://todo-backend-service:3055/todos
```

### CronJob

```
apiVersion: batch/v1
kind: CronJob
metadata:
  namespace: todo-namespace
  name: read-random-wiki-page
spec:
  schedule: "0 * * * *"
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: todo-cronjob
            image: centos
            command: ["/script/cronjob-script.sh"]
            volumeMounts:
              - name: script
                mountPath: "/script"
          volumes:
            - name: script
              configMap:
                name: cronjob-configmap
                defaultMode: 0500
          restartPolicy: Never  
```

## Commands

![Commands for Exercise 2.09](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.09/Exercise_2.09_commands.png)

## Frontend

![CronJob todos in the Frontend](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part2/Exercise_2.09/Exercise_2.09_frontend.png)

