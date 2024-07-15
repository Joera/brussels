
### Build image for Nox (Fluence peer) with a Lotus --lite node included

```
cd nox-with-lotus
docker build -t nox-with-lotus 

```

### Deploy local Fluence network with above images. 

```
# first create the nox tomls with intended whitelisting
fluence local up 
# redeploy network with correct images 
docker compose -f _docker-compose.yaml up -d

```

### temp hacky solution: start lotus daemon manually 

you can use one of these api's. 
wss://wss.node.glif.io/apigw/lotus
wss://api.chain.love


```
docker exec -ti fluence-nox-0-1 bash
FULLNODE_API_INFO=wss://wss.node.glif.io/apigw/lotus && nohup lotus daemon --lite > /dev/null 2>&1
docker exec -ti fluence-nox-1-1 bash
FULLNODE_API_INFO=wss://wss.node.glif.io/apigw/lotus && nohup lotus daemon --lite > /dev/null 2>&1
docker exec -ti fluence-nox-2-1 bash
FULLNODE_API_INFO=wss://api.chain.love && nohup lotus daemon --lite > /dev/null 2>&1

```