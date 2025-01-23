const k8s = require('@kubernetes/client-node')
const mustache = require('mustache')
const request = require('request')
const JSONStream = require('json-stream')
const fs = require('fs').promises

const timeouts = {}

const kc = new k8s.KubeConfig();

process.env.NODE_ENV === 'development' ? kc.loadFromDefault() : kc.loadFromCluster()

const opts = {}
kc.applyToRequest(opts)

const client = kc.makeApiClient(k8s.CoreV1Api);

const sendRequestToApi = async (api, method = 'get', options = {}) => 
    new Promise((resolve, reject) => 
        request[method](`${kc.getCurrentCluster().server}${api}`, 
            { ...opts,
              ...options,
              headers: { ...options.headers, ...opts.headers } },
             (err, res) => err ? reject(err) : resolve(JSON.parse(res.body))))

const fieldsFromDummySite = (object) => ({
  dummysite_name: object.metadata.name,
  container_name: object.metadata.name,
  namespace: object.metadata.namespace,
  website_url: object.spec.website_url,
})

const fieldsFromDeployment = (object) => ({
  dummysite_name: object.metadata.labels.dummysite,
  container_name: object.metadata.labels.dummysite,
  namespace: object.metadata.namespace,
  website_url: object.spec.website_url,
})

const getDeploymentYAML = async (fields) => {
  const deploymentTemplate = await fs.readFile("deployment.mustache", "utf-8")
  return mustache.render(deploymentTemplate, fields)
}

const getServiceYAML = async (fields) => {
  const serviceTemplate = await fs.readFile("service.mustache", "utf-8")
  return mustache.render(serviceTemplate, fields)
}

const getIngressYAML = async (fields) => {
  const serviceTemplate = await fs.readFile("ingress.mustache", "utf-8")
  return mustache.render(serviceTemplate, fields)
}


const createDeployment = async (fields) => {
  console.log('Scheduling new deployment for dummysite ', fields.dummysite_name, ' to namespace ', fields.namespace)

  const yaml = await getDeploymentYAML(fields)

  return sendRequestToApi(`/apis/apps/v1/namespaces/${fields.namespace}/deployments`, 'post', {
    headers: {
      'Content-Type': 'application/yaml'
    },
    body: yaml
  })
}

const createService = async (fields) => {
  console.log('Scheduling new service for dummysite ', fields.dummysite_name, ' to namespace ', fields.namespace)

  const yaml = await getServiceYAML(fields)

  console.log(yaml)

  return sendRequestToApi(`/api/v1/namespaces/${fields.namespace}/services`, 'post', {
    headers: {
      'Content-Type': 'application/yaml'
    },
    body: yaml
  })
}

const createIngress = async (fields) => {
  console.log('Scheduling new ingress for dummysite ', fields.dummysite_name, ' to namespace ', fields.namespace)

  const yaml = await getIngressYAML(fields)

  return sendRequestToApi(`/apis/networking.k8s.io/v1/namespaces/${fields.namespace}/ingresses`, 'post', {
    headers: {
      'Content-Type': 'application/yaml'
    },
    body: yaml
  })
}

const removeDeployment = async ({ namespace, deployment_name }) => {
  const pods = await sendRequestToApi(`/api/v1/namespaces/${namespace}/pods/`)
  pods.items.filter(pod => pod.metadata.labels['deployment-name'] === deployment_name).forEach(pod => removePod({ namespace, pod_name: pod.metadata.name }))

  return sendRequestToApi(`/apis/apps/v1/namespaces/${namespace}/deployments/${deployment_name}`, 'delete')
}

const removeDummySite = ({ namespace, dummysite_name }) => sendRequestToApi(`/apis/stable.dwk/v1/namespaces/${namespace}/dummysites/${dummysite_name}`, 'delete')

const removePod = ({ namespace, pod_name }) => sendRequestToApi(`/api/v1/namespaces/${namespace}/pods/${pod_name}`, 'delete')

const cleanupForDummySite = async ({ namespace, dummysite_name }) => {
  console.log('Doing cleanup')
  clearTimeout(timeouts[dummysite_name])

  const deployments = await sendRequestToApi(`/apis/apps/v1/namespaces/${namespace}/deployments`)
  deployments.items.forEach(deployment => {
    if (!deployment.metadata.labels.dummysite === dummysite_name) return

    removeDeployment({ namespace, deployment_name: deployment.metadata.name })
  })
}

const rescheduleDeployment = (deploymentObject) => {
  const fields = fieldsFromDeployment(deploymentObject)
  if (Number(fields.length) <= 1) {
    console.log('dummysite ended. Removing dummysite.')
    return removeDummySite(fields)
  }
}

const maintainStatus = async () => {
  (await client.listPodForAllNamespaces()).body // A bug in the client(?) was fixed by sending a request and not caring about response

  /**
   * Watch DummySites
   */

  const dummysite_stream = new JSONStream()

  dummysite_stream.on('data', async ({ type, object }) => {
    const fields = fieldsFromDummySite(object)

    if (type === 'ADDED') {
      createDeployment(fields)
      createService(fields)
      createIngress(fields)
    }
    if (type === 'DELETED') cleanupForDummySite(fields)
  })

  request.get(`${kc.getCurrentCluster().server}/apis/stable.dwk/v1/dummysites?watch=true`, opts).pipe(dummysite_stream)

  /**
   * Watch Deployments
   */

  const deployment_stream = new JSONStream()

  deployment_stream.on('data', async ({ type, object }) => {
    if (!object.metadata.labels) return // If the object has no labels, it is not a summysite deployment
    if (!object.metadata.labels.dummysite) return // If it's not dummysite deployment don't handle
    if (type === 'DELETED' || object.metadata.deletionTimestamp) return // Do not handle deleted deployments
    if (!object?.status?.succeeded) return

    rescheduleDeployment(object)
  })

  request.get(`${kc.getCurrentCluster().server}/apis/apps/v1/deployments?watch=true`, opts).pipe(deployment_stream)
}

maintainStatus()