# Dasharr
Dashboard of your indexers' usage

<p align="center">
  <a href="https://discord.gg/4vd7qAaFwX">
    <img src="https://img.shields.io/badge/Discord-Chat-5865F2?logo=discord&logoColor=white" alt="Join Our Discord">
  </a>
</p>

## Quickstart

- Copy [compose.yml](./compose.yml)
- Create a `.env` file following [this](./backend/.env.example) one, and place it in the same folder as the compose file
- Run dasharr: `docker compose up -d` (this will run the postgres database as well)
- Visit the webui and configure the indexers that you want
- New stats will be collected for the enabled indexers every 6h
