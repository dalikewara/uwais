# {{PROJECT_NAME}}

This repository follows **uwais** project structure **{{STRUCTURE_VERSION}}**.

## Project Structure

To implement the concept of **Clean Architecture** and ~~Domain-Driven Design~~ **Feature-Driven Design**, and to keep them as simple and understandable as possible, **uwais** structures the project like this:

### main{{LANGUAGE_EXTENSION}}

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
uwais import feature authentication /path/to/your/project
```

```bash
uwais import feature authentication,subscription git@github.com:username/project.git
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
    "user",
    "product"
  ],
  "features": [
    "user",
    "product"
  ],
  "commons": [
    "validator",
    "formatter"
  ],
  "externals": [
    "put the external package you want to add/install here (can be a name or a url, depend on the language/package manager)"
  ]
}
```

## Other Commands

There are several commands similar to `import feature` above, such as `import domain` and `import common`.
They function in the same way, for example:

```bash
uwais import domain store,order /path/to/your/project
```

```bash
uwais import common string,time https://example.com/user/project.git
```
