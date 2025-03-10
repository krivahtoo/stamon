<div align="center">
  <h1>
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/1b384c38-7d94-4d42-90c4-098cfddf61af">
      <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/a7d933c6-e07a-43f6-989f-8f2224f4fdee">
      <img alt="Shows a black logo in light color mode and a white one in dark color mode." src="https://github.com/user-attachments/assets/a7d933c6-e07a-43f6-989f-8f2224f4fdee">
    </picture>
  </h1>

  *A lightweight self-hosted status monitoring tool, written in Rust.*

  [![Rust CI](https://github.com/krivahtoo/stamon/actions/workflows/rust.yml/badge.svg)](https://github.com/krivahtoo/stamon/actions/workflows/rust.yml)
  [![Node CI](https://github.com/krivahtoo/stamon/actions/workflows/node.yml/badge.svg)](https://github.com/krivahtoo/stamon/actions/workflows/node.yml)
  [![ALX Portfolio Project](https://img.shields.io/badge/ALX-Portfolio_Project-%2331c554?logo=alx&label=&labelColor=%23002B56)](https://www.alxafrica.com/)
  ![Docker Image Version](https://img.shields.io/docker/v/k4htoo/stamon?logo=docker&sort=semver)
  ![GitHub last commit](https://img.shields.io/github/last-commit/krivahtoo/stamon?style=flat&logo=git&logoColor=%23F05032)
  ![GitHub License](https://img.shields.io/github/license/krivahtoo/stamon)
</div>


> [!NOTE]
> Stamon is currently in active development and is not yet fully functional. However, you are welcome to try it out and provide feedback! ;-)
>

![2024-09-19-180624_1366x665_scrot](https://github.com/user-attachments/assets/13d63921-0316-4182-af01-3c0786769f8d)

## ✨ Features

- **Real-Time**: Keep track of your services and resources with up-to-the-second updates.
- **Lightweight**: Minimal resource usage, ideal for environments with limited resources.
- *More features coming soon!*

## 🚀 Quick Start

Although some features are missing the basic monitoring is working as expected. You can try it out and [open an issue](https://github.com/krivahtoo/stamon/issues) if you encounter any bugs or any feature you would like to be added to stamon.

### 🚢 Docker

#### `docker run`

```sh
docker run -it -e JWT_SECRET="hello-world" -p 3000:3000 k4htoo/stamon
```

#### `docker compose`

```yaml
services:
  stamon:
    image: k4htoo/stamon

    environment:
      JWT_SECRET: "A_very_secure_secret"
    ports:
      - "3000:3000"
```
Then, start the container:
```sh
docker compose up stamon
```

Visit http://localhost:3000.

## 🏗️ Development

To start the frontend:
```shell
cd ./frontend
pnpm dev
```

Then on other terminal start the backend

```shell
cargo run
```

Then visit http://localhost:5173

## 📝 Contributing

Thank you for considering contributing to Stamon! Contributions are welcome, including:

  - **New Issues**: Feature requests, bug reports, questions, ideas, and more.
  - **Pull Requests**: Documentation updates, code improvements, new features, etc.

## 👷 Contributors
<a href="https://github.com/krivahtoo/stamon/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=krivahtoo/stamon" alt="Contributors to krivahtoo/stamon" />
</a>

Made with [contrib.rocks](https://contrib.rocks).

## 🛣️ Roadmap

- [ ] Implement notification system for alerts
  - [ ] Integration with third-party services (e.g., Telegram, Slack, email notifications)     
- [ ] Add more monitoring types (e.g. DNS, TCP, docker)
- [ ] Add support for custom alert thresholds


## 📜 License
This project is licensed under the [MIT license](./LICENSE).
