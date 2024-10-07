# Injective Substreams-powered Subgraph (EntityChanges-based)

This is an example application of an Injective Substreams-powered Subgraph that uses the EntityChanges method, instead of the trigger-based method.

In the EntityChanges flow, the Graph Node entities are directly created in the Susbtreams code (Rust code) and pushed to the Postgres database of the Graph Node. This way, you can benefit from the parallelization engine of Susbtreams, thus reducing the index times of your Subgraph.

## Getting Started

### Deploy the Subgraph

All the subgraph-related code in located in the `subgraph` folder.
 
1. Have a local Graph Node running (the `dev-environment` directory contains a `docker-compose.yaml` file that spins up a local Graph Node with all the dependencies).

2. Create the subgraph in the local Graph Node instance:

```bash
graph create --node http://localhost:8020/ my_project
```

3. Deploy the subgraph to the Graph Node.

```bash
graph deploy --node http://localhost:8020/ --ipfs http://localhost:5001 my_project --version-label=v0.0.1
```

The subgraph will read the data from the Substreams package contained in the root folder (`my-project-v0.1.0.spkg`).

### Modify the Substreams

The Substreams defines two modules:
- `map_events`: this module uses the Injective Foundational modules to filter the events that you want to index. The events are defined in the _params_ section. In this example, only the `wasm` events are tracked:

```yaml
params:
  map_events: type:wasm
```

- `graph_out`: this is the _special_ module that outputs EntityChanges objects (i.e. the entities of the Graph Node). It receives the filtered events as input and converts them into subgraph entities.