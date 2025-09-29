# Dasharr
Dashboard of your indexers' usage

## Quickstart

- Copy [compose.yml](./compose.yml)
- Create a `.env` file following [this](./backend/.env.example) one, and place it in the same folder as the compose file
- Run dasharr: `docker compose up -d` (this will run the postgres database as well)
- Visit the webui and configure the indexers that you want
- New stats will be collected for the enabled indexers every 6h
