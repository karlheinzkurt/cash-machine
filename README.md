![build](https://github.com/karlheinzkurt/cash-machine/workflows/Rust/badge.svg)

# Motivation
Just a playground project to explore the Rust programming language and OpenFaaS. Since I'm 
quite used to C++, I wanted to find out the advantages of Rust. Furthermore, I'm 
concerned about size of docker images in real world projects and I would like to find ways to implement
features in languages providing a sufficient ecosystem but keeping the image size of the increasing
number of docker images as small as possible. 

# What it does?
Takes an amount of money as an argument and calculates the notes to be paid out to the customer (without any practical use, just to play).

# Requirements
- https://medium.com/@alexellisuk/walk-through-install-kubernetes-to-your-raspberry-pi-in-15-minutes-84a8492dc95a
- https://www.docker.com/blog/multi-arch-images/

# Deploy to local k8s cluster hosting OpenFaaS and running on RasberryPis
Build image locally to ensure faas-cli copies the template stuff to the right places.
```
faas-cli build -f cash-machine.yml
```

Build images for amd64, arm64, arm/v7 and push them.
```
pushd build/cash-machine
docker buildx build --platform linux/amd64,linux/arm64,linux/arm/v7 -t karlheinzkurt/cash-machine:latest --push .
popd
```

Deploy function to local cluster, while 192.168.23.10 is the IPv4 of my local cluster and
should be changed properly.
```
export OPENFAAS_URL=http://192.168.23.10:31112
faas-cli deploy -f cash-machine.yml
```

Use the function.
```
curl -s http://192.168.23.10:31112/function/cash-machine -d '{"amount":188}' | json_pp
{
   "notes" : {
      "1" : 1,
      "50" : 1,
      "5" : 1,
      "20" : 1,
      "2" : 1,
      "10" : 1,
      "100" : 1
   }
}
```

# TODOs
- Make error handling RFC7807 conform: https://tools.ietf.org/html/rfc7807
- To avoid forking the process for each request: https://github.com/openfaas-incubator/of-watchdog/blob/master/README.md

# OpenFaaS helper
- https://docs.openfaas.com/deployment/troubleshooting/

# k3s helper
Clean-up docker images
```
sudo k3s crictl images
sudo k3s crictl rmi --prune
```
