#!/bin/bash
set -o errexit

# User Args
COMMAND=$1

# Static Args
reg_name='kind-registry'
reg_port='5000'

# Create Cluster
if [[ $1 == "create" ]]; then
  # Check for and start a local Docker registry
  running="$(docker inspect -f '{{.State.Running}}' "${reg_name}" 2>/dev/null || true)"
  if [ "${running}" != 'true' ]; then
    docker run \
      -d --restart=always -p "${reg_port}:5000" --name "${reg_name}" \
      registry:2
  fi

  # Create a KinD cluster
  kind create cluster --name dev-stack --config=./kind-cluster/dev-stack.yaml

  # Connect the local Docker registry with the kind network
  docker network connect "kind" "${reg_name}" > /dev/null 2>&1 &

  # Deploy Nginx Ingress
  kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/master/deploy/static/provider/kind/deploy.yaml

  echo -e "\n\n Cluster Spun Up!"

# Delete Cluster
elif [[ $1 == "delete" ]]; then
  kind delete cluster --name dev-stack
  echo -e "\nCluster Deleted!"

# Notify user of arg choices
else
  echo -e "Please provide an argument that is equal to either:\n(1) create\n(2) delete"
fi

exit 0