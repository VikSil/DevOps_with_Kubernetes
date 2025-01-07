# Assignment

> In [part 2](https://devopswithkubernetes.com/part-2/4-statefulsets-and-jobs#jobs-and-cronjobs) we did a Job that made a backup of our Database using the command *pg_dump*. Unfortunately, the backup was not saved anywhere. Create now a CronJob that makes a backup of your database (once per 24 hours) and saves it to [Google Object Storage](https://cloud.google.com/storage?hl=en).
> 
> In this exercise, you can create the secret for the cloud access from the command line, thus, there is no need to create it in the GitHub action.
> 
> When the cron job is working, you can e.g. download the backups using the Google Cloud Console:
> 
> ![Google Cloud bucket example](https://devopswithkubernetes.com/static/2f16bb93b2a9a692374d734985e0187e/09096/bucket.png)

# Solution

## Manifests

The following `CronJob` and `ConfigMap` were added to `kustomize` resources:

### [CronJob](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.07/manifests/cronjob.yaml)

```
apiVersion: v1
kind: ConfigMap
metadata:
  namespace:  backup
  name: backup-configmap
data:
  backup-script.sh: |
    #!/bin/bash

    set -e

    if [[ -z "${SERVICE_ACCOUNT_KEY}" ]]; then
      echo "Service account key missing"
      exit 1
    fi

    if [[ -z "${DB_URL}" ]]; then
      echo "DBA URL missing"
      exit 1
    fi

    apt-get update && apt-get install -y wget \
    lsb-release && \
    echo "deb http://apt.postgresql.org/pub/repos/apt/ $(lsb_release -c | awk '{print $2}')-pgdg main" > /etc/apt/sources.list.d/pgdg.list && \
    wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add - && \
    apt-get update && apt-get install -y postgresql-client-17 && \
    rm -rf /var/lib/apt/lists/*

    echo "Postgres installed"

    touch /tmp/gcloud_key.json
    echo "$SERVICE_ACCOUNT_KEY" > /tmp/gcloud_key.json
    echo "echoed to key json"

    gcloud auth activate-service-account --key-file=/tmp/gcloud_key.json
    echo "GCloud activated"

    BACKUP="/tmp/backup-$(date +%FT%T.%3N).sql.gz"
    pg_dump -v $DB_URL | gzip > $BACKUP
    echo "Dump complete"

    gsutil cp $BACKUP gs://dwk-gke-backup-bucket/
    echo "Backup stored"

```

### [ConfigMap](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part3/Exercise_3.07/manifests/configmap.yaml)

```
apiVersion: batch/v1
kind: CronJob
metadata:
  namespace:  backup
  name: backup-db
spec:
  schedule: "0 0 * * *"
  concurrencyPolicy: Forbid 
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: backup-cronjob
            image: google/cloud-sdk:slim
            command: ["/script/backup-script.sh"]
            volumeMounts:
              - name: script
                mountPath: "/script"
              - name: temp
                mountPath: "/etc/temp"
            env:
              - name: SERVICE_ACCOUNT_KEY
                valueFrom:
                  secretKeyRef:
                    name: gc-secret
                    key: SERVICE_ACCOUNT_KEY
              - name: DB_URL
                valueFrom:
                  secretKeyRef:
                    name: todo-secret
                    key: DB_URL
          volumes:
            - name: script
              configMap:
                name: backup-configmap
                defaultMode: 0777
            - name: temp
              emptyDir: {}
          restartPolicy: Never
```

### Secrets

Two secrets were added into the namespace manually:

- `todo-secret` containing the database connection string.
- `gc-secret` containing Google Cloud account key


## Storage bucket

![Google Cloud sotrage bucket for Exercise 3.07](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part3/Exercise_3.07/Exercise_3.07_GC_bucket.png)
