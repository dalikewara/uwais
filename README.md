# What Is Uwais? 🤔

Uwais—formerly named ayapingping—is a standard project structure generator to build applications that follow Clean Architecture and Feature-Driven Design concept in various programming languages (such as Golang, Python, Typescript, etc). Uwais aims to be a seamless and very simple project structure while avoiding unnecessary complexity.

Keep in mind that this is not a framework. It's just a standard project structure. So, you will not be forced to follow the APIs that other frameworks require. You're completely free to use your own API or implementation style—as long as you still follow the project structure.

Yes, the only thing you have to follow is the project structure itself. Uwais can't control your coding style, but Uwais can arrange your project structure so that everyone who looks at it will understand it because of its simplicity.

# 🏃‍♂️ Getting Started

## Requirements

Uwais has been fully migrated to Rust starting from version `v2.0.0`. This ensures seamless compatibility with all major operating systems (Linux, Windows, macOS).

However, the legacy version is still available for download. If you specifically need to use the old version, please ensure you meet these requirements, as it was built using POSIX shell scripts:

- Operating systems that support `/bin/sh` with POSIX standards. Linux and MacOS should work without issues as they support it by default. For Windows users, consider using WSL instead
- curl
- git

## Installation

### Linux and MacOS

To install Uwais, run the following command:

```bash
curl --proto '=https' --tlsv1.2 -L https://dalikewara.com/uwais/install.sh | sh
```

> If the `install.sh` URL above doesn’t work, you can use this URL instead: `https://raw.githubusercontent.com/dalikewara/uwais/master/install.sh`.

> If you’re concerned about safety and security, you can download the [install.sh](https://dalikewara.com/uwais/install.sh) file first and execute it manually. Or, you can view the `install.sh` source code at [https://raw.githubusercontent.com/dalikewara/uwais/master/install.sh](https://raw.githubusercontent.com/dalikewara/uwais/master/install.sh).
  
### Windows

If you are using WSL or a tool like Git Bash, you can use the `curl` command mentioned above. Alternatively, you can download the installer from the [GitHub release Page](https://github.com/dalikewara/uwais/releases). The installer file is the one ending with `windows-installer.exe`.

### Manual Installation

If you prefer to install Uwais manually, download the archive for your operating system from the [GitHub release Page](https://github.com/dalikewara/uwais/releases). Once extracted, you will find an executable file. You should place this file in a directory included in your system's PATH (e.g., `/usr/local/bin` on Linux) so it can be easily accessed from the command line.

### Upgrade

If you already have Uwais installed, simply run the following command to upgrade to the latest version:

```bash
uwais update
```

> If you are upgrading from version `v1.1.2`, you must run `uwais update` twice to reach the latest version (`v2.0.0+`). This is necessary because the first run updates you to the final legacy version (`v1.2.0`), which includes the migration scripts required for the transition. The second run then performs the actual upgrade to the new Rust-based version (`v2.0.0+`).

## Usage

Simply run:

```bash
uwais
```

This will list all available commands and usage info.

# ♻ Backward Compatibility

Uwais maintains backward compatibility by separating the version of the source/app from the version of the generated project structure.
With this approach, Uwais can introduce future changes without breaking any existing generated project structures.

## Source or App Version

This refers to the version of the Uwais source code. You should always update Uwais to the latest version to receive the newest fixes and features.
Don’t worry, upgrading Uwais will not break your generated project structure; it will remain automatically compatible.

## Project Structure Version

This refers to the version of the generated project structure. Different versions may produce different structures, which helps ensure backward compatibility.

> The current default project structure version is `v4`.

# 🗄 Project Structure (v4)

To implement the concept of Clean Architecture and Feature-Driven Design, and to keep them as simple and understandable as
possible, Uwais structures the project like this:

> This is project structure `v4`, which is currently the only available version.

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

## Explanation

### 📄 main.[extension]

In this file, you initialize dependencies, injections, and anything required to start and run your application. This is the
starting or entry point of your application.

### 📁 domain

The Domain represents your primary business model or entity. Define your main object models or properties for your business here,
including database models, DTOs (Data Transfer Objects), etc. Keep this package as straightforward as possible. Avoid including any code that is
not directly related to the model itself.

### 📁 common

In this place, you can implement various functions to assist you in performing common tasks—consider them as helpers. Common functions
can be directly called from anywhere.

### 📁 features

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
    "user.go",
    "product.go"
  ],
  "features": [
    "user",
    "product"
  ],
  "commons": [
    "validator.go",
    "formatter.go"
  ],
  "externals": [
    "github.com/go-sql-driver/mysql",
    "github.com/jmoiron/sqlx"
  ]
}
```

## Make It Your Own

Feel free to create your own style to suit your requirements, as long as you still follow the main architecture concept.
For example, in the project root, you can create folders such as `migration` to store your database migrations, `tmp` for
temporary files, or even `infra` to house infrastructure configurations or scripts to facilitate the deployment of your project on a server or VM.

## Seamless Integration

By using this project structure, you can easily import or move your business logic to another project. There are also commands available to help with this; simply run the `uwais` command.

## Release

### Changelog

Read at [CHANGELOG.md](https://github.com/dalikewara/uwais/blob/master/CHANGELOG.md)

### Credits

Copyright &copy; 2024 [Dali Kewara](https://www.dalikewara.com)

### License

[MIT License](https://github.com/dalikewara/uwais/blob/master/LICENSE)
