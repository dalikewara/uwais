# {{PROJECT_NAME}}

This repository follows **Uwais** project structure version `{{STRUCTURE_VERSION}}`. For more information, visit [GitHub](https://github.com/dalikewara/uwais) or the [Docs](https://dalikewara.com/docs/uwais).

## Explanation

### 📄 src/main{{LANGUAGE_EXTENSION}}

In this file, you initialize dependencies, injections, and anything required to start and run your application. This is the
starting or entry point of your application.

### 📁 src/domain

The Domain represents your primary business model or entity. Define your main object models or properties for your business here,
including database models, DTOs (Data Transfer Objects), etc. Keep this package as straightforward as possible. Avoid including any code that is
not directly related to the model itself.

### 📁 src/common

In this place, you can implement various functions to assist you in performing common tasks—consider them as helpers. Common functions
can be directly called from anywhere.

### 📁 src/features

A Feature encapsulates your main business feature, logic, or service. Here, you include everything necessary to ensure the proper functioning of the feature.
Please prioritize Feature-Driven Design, ensuring that features should be easily adapted and seamlessly integrated and imported into different projects.

A standard Feature may comprise the following parts: `repository`, `use case`, and `service`. But, these are OPTIONAL, so feel free to adopt your
own style as long as it aligns with the core concept.

> `repository`
>
> Handles communication with external data resources like databases, cloud services, or external services. Keep your repositories as simple as possible,
> avoid adding excessive logic. If necessary, separate operations into smaller methods. Changes outside the `repository` SHOULD NOT affect the repository itself (except
> changes for business domain/model/entity). For config variables, database frameworks, or external clients, pass or inject them as dependencies.

> `use case`
>
> Contains the main feature logic. Changes outside the `use case` SHOULD NOT affect the use case itself (except changes for business domain/model/entity and repository).
> For config variables, external clients, or repositories, pass or inject them as dependencies.

> `service`
>
> Hosts feature handlers like HTTP handlers, gRPC handlers, cron jobs, or anything serving between the client and your feature or application.
> Changes outside the `service` SHOULD NOT affect the service itself (except changes for business domain/model/entity, repository and use case).
> For config variables, external clients, or use cases, pass or inject them as dependencies.

#### Feature Dependencies (OPTIONAL)

If your feature relies on other modules or external packages, you must define them to ensure a smooth import process. Failure to do so may result in missing package errors. To handle this, place a `dependency.json` file inside your feature directory. You can link dependencies from the `domain`, `common`, and `features` directories. Note: Dependencies from other directories are not supported. You can also use the `externals` field to list third-party libraries that should be installed automatically. Example:

```json
{
  "domains": [
    "user{{LANGUAGE_EXTENSION}}",
    "product{{LANGUAGE_EXTENSION}}"
  ],
  "features": [
    "user",
    "product"
  ],
  "commons": [
    "validator{{LANGUAGE_EXTENSION}}",
    "formatter{{LANGUAGE_EXTENSION}}"
  ],
  "externals": [
    "put the external package you want to add or install here (can be a name or a url, depend on the language or package manager)"
  ]
}
```

## Make It Your Own

Feel free to create your own style to suit your requirements, as long as you still follow the main architecture concept.
For example, in the project root, you can create folders such as `migration` to store your database migrations, `tmp` for
temporary files, or even `infra` to house infrastructure configurations or scripts to facilitate the deployment of your project on a server or VM.

## Seamless Integration

By using this project structure, you can easily import or move your business logic to another project. There are also commands available to help with this; simply run the `uwais` command.
