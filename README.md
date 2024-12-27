# uwais

**uwais**&mdash;formerly named **ayapingping**&mdash;is a standard project structure generator to build applications that follow **Clean Architecture** and **Feature-Driven Design** concept in various programming languages (such as Golang, Python, Typescript, etc).
**uwais** aims to be a seamless and very simple project structure while avoiding unnecessary complexity.

## Requirements

- Operating systems that support `/bin/sh` with **POSIX** standards. **Linux** and **MacOS** should work without issues as they support it by default. For **Windows** users, consider using WSL instead
- curl
- git

## Installation

To install or upgrade **uwais** to the latest version, run the following command:

```sh
curl -L https://raw.githubusercontent.com/dalikewara/uwais/master/install.sh | sh
```

If you already have **uwais** installed, simply run the following command to upgrade to the latest version:

```sh
uwais update
```

## Usage

To use **uwais**, just run this command:

```sh
uwais
```

It will show you all the information you need.

## Backward Compatibility

There are two types of versions:

- **uwais** version: This refers to the version of the **uwais** source code. Always update **uwais** to the latest version to get the latest fixes and features
- Project Structure version: This refers to the version of the generated project structure. Different versions may produce different project structures, which is useful for ensuring backward compatibility

## Project Structure

To implement the concept of **Clean Architecture** and ~~Domain-Driven Design~~ **Feature-Driven Design**, and to keep them as simple and understandable as possible, we structure the project like this:

Example (Golang):

```text
- common
    - error.go
    - fiber.go
    - mysql.go
    - response.go
- domain
    - user.go
    - product.go
- features
    - user
        - httpService_fiber_v1.go
        - repository_mysql.go
        - usecase_v1.go
    - product
    	- httpService_fiber_v1.go
        - repository_mysql.go
        - usecase_v1.go
- main.go
```

> Current version is **v4**

### main.[extension]

- In this file, you initialize dependencies, injections, and anything required to start and run your application
- This is the starting or entry point of your application

### domain

- The **Domain** represents your primary business model or entity
- Define your main object models or properties for your business here, including database models, DTOs (Data Transfer Objects), etc
- Keep this package as straightforward as possible. Avoid including any code that is not directly related to the model itself

### common

- In this place, you can implement various functions to assist you in performing common tasks—consider them as helpers
- Common functions can be directly called from anywhere

### features

- A **Feature** encapsulates your main business feature, logic, or service
- Here, you include everything necessary to ensure the proper functioning of the feature
- Please prioritize **Feature-Driven Design**, ensuring that features should can be easily adapted and seamlessly integrated and imported into different projects
- A standard **Feature** may comprise the following parts: `repository`, `use case`, `http/grpc/cron/etc service`. But, these are **OPTIONAL**, so feel free to adopt your own style as long as it aligns with the core concept:
  - **repository**
    - Handles communication with external data resources like databases, cloud services, or external services
    - Keep your repositories as simple as possible, avoid adding excessive logic
    - If necessary, separate operations into smaller methods
    - Changes outside the `repository` **SHOULD NOT** affect it (except changes for business domain/model/entity)
    - For config variables, database frameworks, or external clients, pass or inject them as dependencies
  - **use case**
    - Contains the main feature logic
    - Changes outside the `use case` **SHOULD NOT** affect it (except changes for business domain/model/entity and repository)
    - For config variables, external clients, or repositories, pass or inject them as dependencies
  - **http/grpc/cron/etc service**
    - Hosts feature handlers like HTTP handlers, gRPC handlers, cron jobs, or anything serving between the client and your feature or application
    - Changes outside the `service` **SHOULD NOT** affect it (except changes for business domain/model/entity, repository and use case)
    - For config variables, external clients, or use cases, pass or inject them as dependencies
- The `dependency.json` is **OPTIONAL**, and only useful when you use the `import feature` command. It serves to define the **Feature** dependencies and avoids possible missing package errors

### infra (OPTIONAL)

- This is the location to house infrastructure configurations or scripts to facilitate the deployment of your project on a server or VM

### Make It Your Own

Feel free to create your own style to suit your requirements, as long as you still follow the main architecture concept.
You can create folders such as `migration` to store your database migrations, `tmp` for temporary files, etc.

## Importing Features from Another Project

To seamlessly incorporate or import features from another project, use the `import feature` command:

```bash
uwais import feature [feature1,feature2,...] [/local/project or https://example.com/user/project.git or git@example.com:user/project.git]
```

For example:

```bash
uwais import feature exampleFeature /path/to/your/project
```

```bash
uwais import feature exampleFeature1,exampleFeature2 git@github.com:username/project.git
```

### Feature dependency

This is **OPTIONAL**. But, if your feature relies on external packages, it's crucial to address dependencies properly during the import process.
Failure to import necessary dependencies may result in missing packages.
To prevent this, please put your feature dependencies in the `dependency.json` file.
Supported dependencies are limited to the following directories: `domain`, `common`, and `features`.
Ensure that your feature dependencies strictly adhere to these directories, avoiding reliance on other locations.
You can also include any external packages to `externals` param to install them automatically.

Example `dependency.json` file (`features/myFeature/dependency.json`):

```json
{
  "domains": [
    "domain1.go",
    "domain2.go"
  ],
  "features": [
    "anotherFeature1",
    "anotherFeature2"
  ],
  "commons": [
    "commonFunction1.go",
    "commonFunction2.go"
  ],
  "externals": [
    "github.com/go-sql-driver/mysql",
    "github.com/jmoiron/sqlx"
  ]
}
```

## Other Commands

There are several commands similar to `import feature` above, such as `import domain` and `import common`.
They function in the same way, for example:

```bash
uwais import domain example.go /path/to/your/project
```

```bash
uwais import common commonFunction1.go https://example.com/user/project.git
```

## Release

### Changelog

Read at [CHANGELOG.md](https://github.com/dalikewara/uwais/blob/master/CHANGELOG.md)

### Credits

Copyright &copy; 2024 [Dali Kewara](https://www.dalikewara.com)

### License

[MIT License](https://github.com/dalikewara/uwais/blob/master/LICENSE)
