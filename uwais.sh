#!/bin/sh

# DEPRECATED: This shell script has been migrated to Rust in version 2.0.0
# Please use the Rust implementation instead.

version="v1.2.0"
original_ifs="$IFS"
language="unknown"
structure_version="v4"
module=""
module_2=""
base_dir=""
common_dir="common"
domain_dir="domain"
features_dir="features"
features_example_dir="$features_dir/example"
infra_dir="infra"
color_green='\033[0;32m'
color_blue='\033[0;34m'
color_cyan='\033[0;36m'
color_yellow='\033[1;33m'
color_reset='\033[0m'
bold_start='\033[1m'
bold_end='\033[0m'

uwais() {
    if is_no_argument "$@"; then
        printf "%b" "
Welcome to ${bold_start}Uwais${bold_end} (${color_yellow}${bold_start}$version${bold_end}${color_reset})\
—formerly named AyaPingPing, \
a standard project structure generator to build applications that follow Clean Architecture and Feature-Driven Design \
concept in various programming languages (such as Golang, Python, Typescript, etc). It aims to be a seamless and very \
simple project structure while avoiding unnecessary complexity.

Usage:

"

        printf "%b" "\
${color_green}${bold_start}version ${bold_end}${color_reset}                      :Get the current uwais version
${color_green}${bold_start}update ${bold_end}${color_reset}                       :Update to the latest uwais version
${color_green}${bold_start}import ${bold_end}${color_reset}[VERSION] OPTION       :Use import function(s)
${color_cyan}${bold_start}    common ${bold_end}${color_reset}NAMES... SOURCE    :Import common function(s) from the source project
${color_cyan}${bold_start}    domain ${bold_end}${color_reset}NAMES... SOURCE    :Import domain(s) from the source project
${color_cyan}${bold_start}    feature ${bold_end}${color_reset}NAMES... SOURCE   :Import features(s) from the source project
${color_green}${bold_start}go ${bold_end}${color_reset}[VERSION]                  :Generate new Golang project
${color_green}${bold_start}python ${bold_end}${color_reset}[VERSION]              :Generate new Python project
${color_green}${bold_start}typescript ${bold_end}${color_reset}[VERSION]          :Generate new TypeScript project
${color_green}${bold_start}nodejs ${bold_end}${color_reset}[VERSION]              :Generate new NodeJS project
${color_green}${bold_start}rust ${bold_end}${color_reset}[VERSION]                :Generate new Rust project (NOT READY YET)
        " | while IFS=: read -r _name _description; do
            printf "%b" "$_name"
            printf "%s\n" "$_description" | fold -s -w $(($(tput cols) - 30)) | sed '2,$s/^/                                /'
        done

        printf "%b" "\nNote:\n\n"

        printf "%b" "\
- :${bold_start}[ ]${bold_end} is OPTIONAL
- :${bold_start}...${bold_end} is multiple value separated by comma
- :${bold_start}VERSION${bold_end} is the structure version
- :Available structure ${bold_start}VERSION${bold_end} (Default is ${bold_start}v4${bold_end}): v4
- :${bold_start}SOURCE${bold_end} project can be one of these: /your/local/path | git@github.com:username/your/project.git | \
https://github.com/username/your/project.git
- :Example usage:
  :
  :generate new Golang project:
  :
  :${color_green}${bold_start}    uwais go${bold_end}${color_reset}
  :
  :import feature from another project:
  :
  :${color_green}${bold_start}    uwais import feature ${bold_end}${color_reset}user,product ../my-project
\n" | while IFS=: read -r _name _description; do
            printf "%b" "$_name"
            printf "%s\n" "$_description" | fold -s -w $(($(tput cols) - 2)) | sed '2,$s/^/  /'
        done

        return 0
    fi

    if is_equal "$1" "version"; then
        echo $version

        return 0
    elif is_equal "$1" "update"; then
        _install_url="https://raw.githubusercontent.com/dalikewara/uwais/master/install.sh"
        _tmp_dir="/tmp"
        _tmp_filepath="$_tmp_dir/uwais_install.sh"

        echo "Migrating to Rust version (2.0.0+)..."
        echo "Downloading installation script..."

        if ! is_dir_exist "$_tmp_dir"; then
          create_dir "$_tmp_dir"
        fi

        if command -v curl > /dev/null 2>&1; then
            curl -L "$_install_url" -o "$_tmp_filepath"
        elif command -v wget > /dev/null 2>&1; then
            wget -O "$_tmp_filepath" "$_install_url"
        else
            echo "Error: Neither curl nor wget is available"
            return 1
        fi

        if [ ! -f "$_tmp_filepath" ]; then
            echo "Failed to download installation script"
            return 1
        fi

        chmod +x "$_tmp_filepath"

        echo "Running installation script..."
        sh "$_tmp_filepath"

        _exit_code=$?

        rm -f "$_tmp_filepath"

        if [ $_exit_code -eq 0 ]; then
            echo ""
            echo "Migration completed successfully!"
            echo "You are now using the Rust version of uwais."
            exit 0
        else
            echo "Installation failed with exit code $_exit_code"
            return 1
        fi
    fi

    language="$1"

    if is_equal "$2" "v4"; then
        structure_version="$2"
    fi

    if is_equal "$1" "import"; then
        printf "%b" "\nImporting...\n"

        module="$2"
        _command="import_$structure_version"

        $_command "$3" "$4"

        echo "Importing... [DONE]"
        printf "%b" "\n${color_green}${bold_start}[DONE]${bold_end}${color_reset}\n\n"

        return 0
    elif is_go || is_python || is_typescript || is_nodejs; then
        echo ""

        read_input "Enter project name (ex: my-project)... " "check_project_name"

        base_dir="$input"
    elif is_rust; then
        echo "Rust is not ready yet!" && exit 1
    else
        echo "Invalid command!" && exit 1
    fi

    if is_go; then
        read_input "Enter go module (ex: my-project, or example.com/user_example/my-project)... " "check_empty_input"

        module="$input"

        read_input "We use vendoring by default, Type '${bold_start}n${bold_end}' and press Enter if you don't want to use it.... " ""

        module_2="vendor"

        if is_equal "$input" "n"; then
            module_2=""
        fi
    elif is_python; then
        read_input "We use virtual environment (venv) by default, Type '${bold_start}n${bold_end}' and press Enter if you don't want to use it.... " ""

        module="venv"

        if is_equal "$input" "n"; then
            module=""
        fi
    fi

    if is_empty "$base_dir"; then
        echo "Project name is empty, so aborted!" && exit 1
    fi

    printf "%b" "
Summary:

Language: ${bold_start}$language${bold_end}
Structure version: ${bold_start}$structure_version${bold_end}
Project name: ${bold_start}$base_dir${bold_end}\n"

    if is_go; then
        printf "%b" "Go module: ${bold_start}$module${bold_end}\n"

        if is_equal "$module_2" "vendor"; then
            printf "%b" "Vendoring: ${bold_start}yes${bold_end}\n"
        else
            printf "%b" "Vendoring: ${bold_start}no${bold_end}\n"
        fi
    elif is_python; then
        if is_equal "$module" "venv"; then
            printf "%b" "Virtual environment (venv): ${bold_start}yes${bold_end}\n"
        else
            printf "%b" "Virtual environment (venv): ${bold_start}no${bold_end}\n"
        fi
    fi

    echo ""

    read_input "Type '${bold_start}y${bold_end}' and press Enter to confirm. Otherwise, the process will be aborted... " ""

    if ! is_equal "$input" "y"; then
        echo "Aborted!" && exit 1
    fi

    printf "%b" "\nGenerating...\n"

    create_dir "$base_dir"

    cd "$base_dir" || exit 1

    echo "cd \"$base_dir\"... [DONE]"

    _command="generate_$structure_version"

    $_command

    echo "..."

    if is_go; then
        go mod init "$module" || printf "%b" "${color_yellow}WARNING: Failed to create go module!\nYour application may not working properly until you create it manually!${color_reset}\n"
        go mod tidy  || printf "%b" "${color_yellow}WARNING: Failed to install dependencies!\nYour application may not working properly until you install them manually!${color_reset}\n"

        if is_equal "$module_2" "vendor"; then
            go mod vendor  || true
        fi

        _running_command="${color_cyan}${bold_start}go run ${bold_end}${color_reset}main.go"
    elif is_python; then
        if ! is_dir_exist "venv"; then
            if is_equal "$module" "venv"; then
                python3 -m venv venv || python -m venv venv || printf "%b" "${color_yellow}WARNING: Failed to create virtual environment!\nYour application may not working properly!${color_reset}\n"
                venv/bin/pip install -r requirements.txt || printf "%b" "${color_yellow}WARNING: Failed to install \"requirements.txt\"!\nYour application may not working properly until you install them manually!${color_reset}\n"
                venv/bin/pip freeze > requirements.txt || true
            else
                pip install -r requirements.txt || printf "%b" "${color_yellow}WARNING: Failed to install \"requirements.txt\"!\nYour application may not working properly until you install them manually!${color_reset}\n"
                pip freeze > requirements.txt || true
            fi
        else
            venv/bin/pip install -r requirements.txt || pip install -r requirements.txt || printf "%b" "${color_yellow}WARNING: Failed to install \"requirements.txt\"!\nYour application may not working properly until you install them manually!${color_reset}\n"
            venv/bin/pip freeze > requirements.txt || pip freeze > requirements.txt || true
        fi

        if is_equal "$module" "venv"; then
            _running_command="${color_cyan}${bold_start}venv/bin/python ${bold_end}${color_reset}main.py"
        else
            _running_command="${color_cyan}${bold_start}python ${bold_end}${color_reset}main.py"
        fi
    elif is_typescript; then
        npm install || printf "%b" "${color_yellow}WARNING: Failed to install dependencies!\nYour application may not working properly until you install them manually!${color_reset}\n"
        ./node_modules/.bin/tsc || tsc || printf "%b" "${color_yellow}WARNING: Failed to execute tsc!\nYour application may not working properly until you execute it manually!${color_reset}\n"
        ./node_modules/.bin/tsc-alias || tsc-alias || printf "%b" "${color_yellow}WARNING: Failed to execute tsc-alias!\nYour application may not working properly until you execute it manually!${color_reset}\n"

        _running_command="${color_cyan}${bold_start}node ${bold_end}${color_reset}./dist/main.js"
    elif is_nodejs; then
        npm install || printf "%b" "${color_yellow}WARNING: Failed to install dependencies!\nYour application may not working properly until you install them manually!${color_reset}\n"

        _running_command="${color_cyan}${bold_start}node ${bold_end}${color_reset}main.js"
    elif is_rust; then
        cargo build || printf "%b" "${color_yellow}WARNING: Failed to install dependencies!\nYour application may not working properly until you install them manually!${color_reset}\n"

        _running_command="cargo run"
    fi

    git init || true

    printf "%b" "\
Generating... [DONE]

To get started, you can enter to your project directory:

    ${color_cyan}${bold_start}cd ${bold_end}${color_reset}$base_dir

and run your application (for example):

    $_running_command

then your application will be available at:

    ${color_blue}${bold_start}http://localhost:8080${bold_end}${color_reset}

For example, try:

    ${color_cyan}${bold_start}curl ${bold_end}${color_reset}${color_blue}${bold_start}http://localhost:8080/example${bold_end}${color_reset}

${color_green}${bold_start}[DONE]${bold_end}${color_reset}

"
}

is_no_argument() {
    [ $# -eq 0 ]
}

is_empty() {
    is_equal "$1" "" || is_equal "$1" " "
}

is_equal() {
    [ "$1" = "$2" ]
}

is_contain() {
    echo "$1" | grep -qF "$2"
}

is_go() {
    is_equal "$language" "go"
}

is_python() {
    is_equal "$language" "python"
}

is_typescript() {
    is_equal "$language" "typescript"
}

is_nodejs() {
    is_equal "$language" "nodejs"
}

is_rust() {
    is_equal "$language" "rust"
}

is_file_exist() {
    [ -f "$1" ]
}

is_dir_exist() {
    [ -d "$1" ]
}

is_git_url() {
    # shellcheck disable=SC2143
    [ "$(echo "$1" | grep -E '\.git$' | grep -E 'https://|http://|git@')" ]
}

get_raw_json_from_file() {
    # shellcheck disable=SC2002
    cat "$1" | tr -d '[:space:]'
}

get_raw_json_from_file_for_external() {
    # shellcheck disable=SC2002
    cat "$1" | tr -d '\n' | tr -s '[:blank:]' ' ' | sed 's/* //g'
}

get_json_value_by_key() {
    get_clean_json_value "$(echo "$1" | tr -d '[:space:]' | sed -n -e "s/.*\"$2\":\[\([^]]*\)\].*/\1/p")"
}

get_json_value_by_key_for_external() {
    get_clean_json_value_for_external "$(echo "$1" | sed -n 's/.*"'$2'":\(\[ *\| \[ *\| *\[\)\([^]]*\).*/\2/p')"
}

get_clean_json_value() {
    get_clean_string_from_space "$1" | tr -d '"' | tr ',' '\n' | tr -s '\n' ','
}

get_clean_json_value_for_external() {
    echo "$1" | tr -d '"' |  tr ',' '\n' | tr -s '\n' ',' | tr -d '[:blank:]' | sed -e 's/, */,/g' -e 's/, *$/ /'
}

get_clean_string_from_space() {
    echo "$1" | tr -d '[:space:]'
}

get_go_module_from_dir() {
    if is_empty "$1"; then
        go list -m -modfile "go.mod" || echo ""

        return 0
    fi

    if ! is_dir_exist "$1"; then
        echo ""

        return 0
    fi

    go list -m -modfile "$1/go.mod" || echo ""
}

read_input() {
    printf "%b" "$1"

    read -r input < /dev/tty

    if is_contain "$2" "check_project_name"; then
        if is_dir_exist "$input"; then
            printf "%s" "(EXIST) "

            read_input "$1" "$2"

            return 1
        fi

        if is_empty "$input"; then
            printf "%s" "(EMPTY) "

            read_input "$1" "$2"

            return 1
        fi

        if is_contain "$input" "/"; then
            printf "%s" "(INVALID) "

            read_input "$1" "$2"

            return 1
        fi

        if is_contain "$input" " "; then
            printf "%s" "(INVALID) "

            read_input "$1" "$2"

            return 1
        fi
    fi

    if is_contain "$2" "check_empty_input" && is_empty "$input"; then
        printf "%s" "(EMPTY) "

        read_input "$1" "$2"

        return 1
    fi
}

create_dir() {
    if is_dir_exist "$1"; then
        echo "mkdir \"$1\"... [EXIST]"

        return 1
    fi

    mkdir "$1"

    echo "mkdir \"$1\"... [DONE]"
}

remove_dir() {
    if ! is_dir_exist "$1"; then
        echo "rm -rf \"$1\"... [NOT_EXIST]"

        return 1
    fi

    rm -rf "$1"

    echo "rm -rf \"$1\"... [DONE]"
}

write_to_file() {
    echo "$1" > "$2"
    echo "touch \"$2\"... [DONE]"
}

copy_content() {
    if ! is_dir_exist "$1" && ! is_file_exist "$1"; then
        echo "cp -rf \"$1\" \"$2\"... [NOT_EXIST]"

        return 1
    fi

    if is_dir_exist "$2" || is_file_exist "$2"; then
        echo "cp -rf \"$1\" \"$2\"... [EXIST]"

        return 1
    fi

    cp -rf "$1" "$2"

    echo "cp -rf \"$1\" \"$2\"... [DONE]"
}

replace_string_in_file() {
    if ! is_file_exist "$1"; then
        echo "sed -i \"$1\" (\"$2\" replaced with \"$3\")... [NOT_EXIST]"

        return 1
    fi

    sed -i "s/$2/$3/g" "$1"

    echo "sed -i \"$1\" (\"$2\" replaced with \"$3\")... [DONE]"
}

replace_string_in_dir() {
    if ! is_dir_exist "$1"; then
        echo "find & sed -i \"$1\" (\"$2\" replaced with \"$3\" on all contents)... [NOT_EXIST]"

        return 1
    fi

    find "$1" -type f -exec sed -i "s#$2#$3#g" {} +

    echo "find & sed -i \"$1\" (\"$2\" replaced with \"$3\" on all contents)... [DONE]"
}

install_go_package() {
    go get "$(get_clean_string_from_space "$@")" || true
}

install_python_package() {
    venv/bin/pip install "$(get_clean_string_from_space "$@")" || pip install "$(get_clean_string_from_space "$@")" || true
}

install_typescript_package() {
    npm install "$(get_clean_string_from_space "$@")" || true
}

install_nodejs_package() {
    npm install "$(get_clean_string_from_space "$@")" || true
}

install_rust_package() {
    cargo add "$(get_clean_string_from_space "$@")" || true
}

determine_dir_language() {
    if ! is_empty "$1" && ! is_dir_exist "$1"; then
        return 0
    fi

    _dir="$1/"

    if is_empty "$1"; then
        _dir=""
    fi

    if is_file_exist "${_dir}go.mod" && is_file_exist "${_dir}go.sum" && is_file_exist "${_dir}main.go" && is_dir_exist "${_dir}vendor"; then
        language="go"

        return 0
    elif is_file_exist "${_dir}__init__.py" && is_file_exist "${_dir}main.py" && is_file_exist "${_dir}requirements.txt" && is_dir_exist "${_dir}venv"; then
        language="python"

        return 0
    elif is_file_exist "${_dir}tsconfig.json" && is_file_exist "${_dir}package.json" && is_file_exist "${_dir}main.ts" && is_dir_exist "${_dir}node_modules"; then
        language="typescript"

        return 0
    fi

    if is_file_exist "${_dir}go.mod" && is_file_exist "${_dir}go.sum" && is_file_exist "${_dir}main.go"; then
        language="go"

        return 0
    elif is_file_exist "${_dir}go.mod" && is_file_exist "${_dir}go.sum" && is_dir_exist "${_dir}vendor"; then
        language="go"

        return 0
    elif is_file_exist "${_dir}go.mod" && is_file_exist "${_dir}main.go" && is_dir_exist "${_dir}vendor"; then
        language="go"

        return 0
    elif is_file_exist "${_dir}__init__.py" && is_file_exist "${_dir}main.py" && is_dir_exist "${_dir}venv"; then
        language="python"

        return 0
    elif is_file_exist "${_dir}__init__.py" && is_file_exist "${_dir}main.py" && is_file_exist "${_dir}requirements.txt"; then
        language="python"

        return 0
    elif is_file_exist "${_dir}main.py" && is_file_exist "${_dir}requirements.txt" && is_dir_exist "${_dir}venv"; then
        language="python"

        return 0
    elif is_file_exist "${_dir}__init__.py" && is_file_exist "${_dir}requirements.txt" && is_dir_exist "${_dir}venv"; then
        language="python"

        return 0
    elif is_file_exist "${_dir}tsconfig.json" && is_file_exist "${_dir}package.json" && is_file_exist "${_dir}main.ts"; then
        language="typescript"

        return 0
    elif is_file_exist "${_dir}tsconfig.json" && is_file_exist "${_dir}main.ts" && is_dir_exist "${_dir}node_modules"; then
        language="typescript"

        return 0
    elif is_file_exist "${_dir}tsconfig.json" && is_file_exist "${_dir}package.json" && is_dir_exist "${_dir}node_modules"; then
        language="typescript"

        return 0
    elif is_file_exist "${_dir}main.ts" && is_file_exist "${_dir}package.json" && is_dir_exist "${_dir}node_modules"; then
        language="typescript"

        return 0
    elif is_file_exist "${_dir}package.json" && is_file_exist "${_dir}main.js" && is_dir_exist "${_dir}node_modules"; then
        language="nodejs"

        return 0
    fi

    if is_file_exist "${_dir}go.mod" && is_file_exist "${_dir}go.sum"; then
        language="go"

        return 0
    elif is_file_exist "${_dir}go.mod" && is_file_exist "${_dir}main.go"; then
        language="go"

        return 0
    elif is_file_exist "${_dir}go.mod" && is_dir_exist "${_dir}vendor"; then
        language="go"

        return 0
    elif is_file_exist "${_dir}__init__.py" && is_file_exist "${_dir}main.py"; then
        language="python"

        return 0
    elif is_file_exist "${_dir}__init__.py" && is_dir_exist "${_dir}venv"; then
        language="python"

        return 0
    elif is_file_exist "${_dir}main.py" && is_dir_exist "${_dir}venv"; then
        language="python"

        return 0
    elif is_file_exist "${_dir}__init__.py" && is_file_exist "${_dir}requirements.txt"; then
        language="python"

        return 0
    elif is_file_exist "${_dir}main.py" && is_file_exist "${_dir}requirements.txt"; then
        language="python"

        return 0
    elif is_file_exist "${_dir}requirements.txt" && is_dir_exist "${_dir}venv"; then
        language="python"

        return 0
    elif is_file_exist "${_dir}tsconfig.json" && is_file_exist "${_dir}package.json"; then
        language="typescript"

        return 0
    elif is_file_exist "${_dir}tsconfig.json" && is_file_exist "${_dir}main.ts"; then
        language="typescript"

        return 0
    elif is_file_exist "${_dir}tsconfig.json" && is_dir_exist "${_dir}node_modules"; then
        language="typescript"

        return 0
    elif is_file_exist "${_dir}main.ts" && is_dir_exist "${_dir}node_modules"; then
        language="typescript"

        return 0
    elif is_file_exist "${_dir}main.ts" && is_file_exist "${_dir}package.json"; then
        language="typescript"

        return 0
    elif is_file_exist "${_dir}package.json" && is_file_exist "${_dir}main.js"; then
        language="nodejs"

        return 0
    elif is_file_exist "${_dir}main.js" && is_dir_exist "${_dir}node_modules"; then
        language="nodejs"

        return 0
    fi

    if is_file_exist "${_dir}go.mod" || is_file_exist "${_dir}main.go"; then
        language="go"

        return 0
    elif is_file_exist "${_dir}__init__.py" || is_file_exist "${_dir}main.py"; then
        language="python"

        return 0
    elif is_file_exist "${_dir}tsconfig.json" || is_file_exist "${_dir}main.ts"; then
        language="typescript"

        return 0
    elif is_file_exist "${_dir}package.json" || is_file_exist "${_dir}main.js"; then
        language="nodejs"

        return 0
    fi
}

determine_dir_language_module() {
    _dir="$1/"

    if is_empty "$1"; then
        _dir=""
    fi

    if is_go && is_dir_exist "${_dir}vendor"; then
        module="vendor"
    elif is_python && is_dir_exist "${_dir}venv"; then
        module="venv"
    fi
}

generate_v4() {
    create_dir "$common_dir"
    create_dir "$domain_dir"
    create_dir "$features_dir"
    create_dir "$features_example_dir"
    create_dir "$infra_dir"

    if is_go; then
        _go_mod_vendor="
go mod vendor"

        if ! is_equal "$module_2" "vendor"; then
            _go_mod_vendor=""
        fi

        write_to_file "\
package common

import \"net/http\"

func NewNetHttpServer() *http.Server {
    return &http.Server{}
}

func NewNetHttpServerMux() *http.ServeMux {
    return http.NewServeMux()
}\
" "$common_dir/netHttp.go"

        write_to_file "\
package common

type ResponseJSON struct {
    Status  bool        \`json:\"status\"\`
    Message string      \`json:\"message\"\`
    Data    interface{} \`json:\"data\"\`
    Errors  []string    \`json:\"errors\"\`
}

func NewResponseJSONSuccess(data interface{}) *ResponseJSON {
    return &ResponseJSON{
        Status:  true,
        Message: \"ok\",
        Data:    data,
    }
}

func NewResponseJSONError(err error) *ResponseJSON {
    return &ResponseJSON{
        Status:  false,
        Message: \"error\",
        Errors: []string{
            err.Error(),
        },
    }
}\
" "$common_dir/response.go"

        write_to_file "\
package common

import \"time\"

func TimeNowUTC() time.Time {
    return time.Now().UTC()
}\
" "$common_dir/time.go"

        write_to_file "\
package domain

import (
    \"context\"
    \"$module/common\"
    \"time\"
)

type ExampleRepository interface {
    FindByIDCtx(ctx context.Context, id uint64) (*Example, error)
}

type ExampleUseCase interface {
    GetDetailCtx(ctx context.Context, id uint64) (*ExampleDTO1, error)
}

type ExampleHttpService interface {
    Detail(method string, path string)
}

type Example struct {
    ID        uint64    \`json:\"id\"\`
    Username  string    \`json:\"username\"\`
    Password  string    \`json:\"password\"\`
    CreatedAt time.Time \`json:\"created_at\"\`
}

func (e *Example) SetCreatedAtNow() {
    e.CreatedAt = common.TimeNowUTC()
}

func (e *Example) ToDTO1() *ExampleDTO1 {
    return &ExampleDTO1{
        ID:        e.ID,
        Username:  e.Username,
        CreatedAt: e.CreatedAt,
    }
}

type ExampleDTO1 struct {
    ID        uint64    \`json:\"id\"\`
    Username  string    \`json:\"username\"\`
    CreatedAt time.Time \`json:\"created_at\"\`
}\
" "$domain_dir/example.go"

        write_to_file "\
package example

import (
    \"encoding/json\"
    \"errors\"
    \"$module/common\"
    \"$module/domain\"
    \"net/http\"
    \"strings\"
)

type httpServiceNetHttpV1 struct {
    client         *http.ServeMux
    exampleUseCase domain.ExampleUseCase
}

func NewHttpServiceNetHttpV1(client *http.ServeMux, exampleUseCase domain.ExampleUseCase) domain.ExampleHttpService {
    return &httpServiceNetHttpV1{
        client:         client,
        exampleUseCase: exampleUseCase,
    }
}

func (h *httpServiceNetHttpV1) Detail(method string, path string) {
    h.client.HandleFunc(path, func(w http.ResponseWriter, r *http.Request) {
        ctx := r.Context()

        if r.Method != strings.ToUpper(method) {
            resultBytes, _ := json.Marshal(common.NewResponseJSONError(errors.New(\"invalid method\")))

            w.WriteHeader(400)

            _, _ = w.Write(resultBytes)

            return
        }

        result, err := h.exampleUseCase.GetDetailCtx(ctx, 1)
        if err != nil {
            resultBytes, _ := json.Marshal(common.NewResponseJSONError(err))

            w.WriteHeader(500)

            _, _ = w.Write(resultBytes)

            return
        }

        resultBytes, _ := json.Marshal(common.NewResponseJSONSuccess(result))

        w.WriteHeader(200)

        _, _ = w.Write(resultBytes)

        return
    })
}\
" "$features_example_dir/httpService_netHttp_v1.go"

        write_to_file "\
package example

import (
    \"context\"
    \"$module/domain\"
)

type repositoryMySQL struct {
    client interface{}
}

func NewRepositoryMySQL(client interface{}) domain.ExampleRepository {
    return &repositoryMySQL{
        client: client,
    }
}

func (r *repositoryMySQL) FindByIDCtx(ctx context.Context, id uint64) (*domain.Example, error) {
    example := &domain.Example{
        ID:       id,
        Username: \"dalikewara\",
        Password: \"admin123\",
    }

    example.SetCreatedAtNow()

    return example, nil
}\
" "$features_example_dir/repository_mysql.go"

        write_to_file "\
package example

import (
    \"context\"
    \"$module/domain\"
)

type useCaseV1 struct {
    exampleRepository domain.ExampleRepository
}

func NewUseCaseV1(exampleRepository domain.ExampleRepository) domain.ExampleUseCase {
    return &useCaseV1{
        exampleRepository: exampleRepository,
    }
}

func (u *useCaseV1) GetDetailCtx(ctx context.Context, id uint64) (*domain.ExampleDTO1, error) {
    example, err := u.exampleRepository.FindByIDCtx(ctx, id)
    if err != nil {
        return nil, err
    }
    if example == nil {
        return nil, nil
    }

    return example.ToDTO1(), nil
}\
" "$features_example_dir/usecase_v1.go"

        write_to_file "\
#!/bin/sh

mkdir tmp || true
go mod tidy$_go_mod_vendor
go build -o main\
" "$infra_dir/build.sh"

        chmod +x "$infra_dir/build.sh"

        write_to_file "\
#!/bin/sh

go mod tidy$_go_mod_vendor
(./main 2>/dev/null && echo \"running './main'\") || (echo \"running 'go run main.go'\" && go run main.go)\
" "$infra_dir/start.sh"

        chmod +x "$infra_dir/start.sh"

        write_to_file "\
package main

import (
    \"fmt\"
    \"$module/common\"
    \"$module/features/example\"
    \"net/http\"
)

func main() {
    // Http server initialization

    httpServer := common.NewNetHttpServer()
    httpServerMux := common.NewNetHttpServerMux()

    // Repositories

    exampleRepository := example.NewRepositoryMySQL(nil)

    // Use cases

    exampleUseCaseV1 := example.NewUseCaseV1(exampleRepository)

    // Services

    exampleHttpServiceV1 := example.NewHttpServiceNetHttpV1(httpServerMux, exampleUseCaseV1)

    // Service handlers

    exampleHttpServiceV1.Detail(http.MethodGet, \"/example\")

    // Start & listen application

    httpServer.Handler = httpServerMux
    httpServer.Addr = \":8080\"

    fmt.Println(\"App running on port: 8080\")

    if err := httpServer.ListenAndServe(); err != nil {
        panic(err)
    }
}\
" "main.go"

    elif is_python; then
        write_to_file "" "__init__.py"
        write_to_file "" "$common_dir/__init__.py"
        write_to_file "" "$domain_dir/__init__.py"
        write_to_file "" "$features_example_dir/__init__.py"

        write_to_file "\
from flask import Flask


def flask_server() -> tuple[Flask | None, Exception | None]:
    try:
        return Flask(__name__), None
    except Exception as e:
        return None, e
" "$common_dir/flask.py"

        write_to_file "\
from dataclasses import dataclass, field
from typing import List, Any


@dataclass(frozen=True)
class ResponseJSON:
    status: bool = False
    message: str = ''
    data: dict = field(default_factory=dict)
    error: List[str] = field(default_factory=list)


def new_response_json_success(data: Any = None) -> dict:
    if data is None:
        data = {}

    data = data.__dict__

    return ResponseJSON(
        status=True,
        message='success',
        data=data
    ).__dict__


def new_response_json_error(err: Exception = None) -> dict:
    if err is None:
        err = ValueError('error')

    return ResponseJSON(
        status=False,
        message='error',
        error=[err.args[0]]
    ).__dict__
" "$common_dir/response.py"

        write_to_file "\
import datetime


def time_now_utc() -> datetime:
    return datetime.datetime.now(datetime.UTC)
" "$common_dir/time.py"

        write_to_file "\
from dataclasses import dataclass
from datetime import datetime
from abc import ABC, abstractmethod
from common.time import time_now_utc


@dataclass(frozen=True)
class ExampleDTO1:
    id: int = 0
    username: str = ''
    created_at: str = ''


@dataclass
class Example:
    id: int = 0
    username: str = ''
    password: str = ''
    created_at: datetime | None = None

    def set_created_at_now(self):
        self.created_at = time_now_utc()

    def get_created_at_str(self) -> str:
        if self.created_at is None:
            return ''

        return self.created_at.strftime('%Y-%m-%d %H:%M:%S')

    def to_dto1(self) -> ExampleDTO1:
        return ExampleDTO1(
            id=self.id,
            username=self.username,
            created_at=self.get_created_at_str()
        )


class ExampleRepository(ABC):

    @abstractmethod
    def find_by_id(self, example_id: int) -> tuple[Example | None, Exception | None]:
        raise NotImplementedError


class ExampleUseCase(ABC):

    @abstractmethod
    def get_detail(self, example_id: int) -> tuple[ExampleDTO1 | None, Exception | None]:
        raise NotImplementedError


class ExampleHttpService(ABC):

    @abstractmethod
    def detail(self, method: str, path: str):
        raise NotImplementedError
" "$domain_dir/example.py"

        write_to_file "\
from flask import Flask, jsonify
from domain.example import ExampleHttpService, ExampleUseCase
from common.response import new_response_json_success, new_response_json_error


class HttpServiceFlaskV1(ExampleHttpService):

    def __init__(self, client: Flask, example_usecase: ExampleUseCase):
        self.client = client
        self.example_usecase = example_usecase

    def detail(self, method: str, path: str):
        @self.client.route(rule=path, methods=[method])
        def example_detail_route_handler():
            try:
                result, err = self.example_usecase.get_detail(1)
                if err is not None:
                    return jsonify(new_response_json_error(err))

                return jsonify(new_response_json_success(result))
            except Exception as e:
                return jsonify(new_response_json_error(e))
" "$features_example_dir/http_service_flask_v1.py"

        write_to_file "\
from domain.example import Example, ExampleRepository


class RepositoryMySQL(ExampleRepository):

    def __init__(self, db: None):
        self.db = db

    def find_by_id(self, example_id: int) -> tuple[Example | None, Exception | None]:
        try:
            example = Example(
                id=example_id,
                username='dalikewara',
                password='admin123'
            )

            example.set_created_at_now()

            return example, None
        except Exception as e:
            return None, e
" "$features_example_dir/repository_mysql.py"

        write_to_file "\
from domain.example import ExampleDTO1, ExampleUseCase, ExampleRepository


class UseCaseV1(ExampleUseCase):

    def __init__(self, example_repository: ExampleRepository):
        self.example_repository = example_repository

    def get_detail(self, example_id: int) -> tuple[ExampleDTO1 | None, Exception | None]:
        try:
            example, err = self.example_repository.find_by_id(example_id)
            if err is not None:
                return None, err

            return example.to_dto1(), None
        except Exception as e:
            return None, e
" "$features_example_dir/usecase_v1.py"

        write_to_file "\
#!/bin/sh

mkdir tmp || true
venv/bin/python -m venv venv || python -m venv venv
venv/bin/pip install -r requirements.txt || pip install -r requirements.txt\
" "$infra_dir/build.sh"

        chmod +x "$infra_dir/build.sh"

        write_to_file "\
#!/bin/sh

venv/bin/python main.py || python main.py\
" "$infra_dir/start.sh"

        chmod +x "$infra_dir/start.sh"

        write_to_file "\
from common.flask import flask_server
from features.example import usecase_v1 as example_usecase_v1, repository_mysql as example_repository_mysql, \
    http_service_flask_v1 as example_http_service_flask_v1

# Http server initialization

flask_svr, err = flask_server()
if err is not None:
    raise err

# Repositories

example_repository_mysql = example_repository_mysql.RepositoryMySQL(db=None)

# Use cases

example_usecase_v1 = example_usecase_v1.UseCaseV1(example_repository=example_repository_mysql)

# Services

example_http_service_flask_v1 = example_http_service_flask_v1.HttpServiceFlaskV1(client=flask_svr, example_usecase=example_usecase_v1)

# Service handlers

example_http_service_flask_v1.detail(method='GET', path='/example')

# Start & listen application

if __name__ == '__main__':
    flask_svr.run(port=8080, debug=True)
" "main.py"

        write_to_file "\
blinker==1.8.2
click==8.1.7
Flask==3.0.2
itsdangerous==2.2.0
Jinja2==3.1.4
MarkupSafe==2.1.5
Werkzeug==3.0.3\
" "requirements.txt"

    elif is_typescript; then
        write_to_file "\
import express, { Express } from \"express\";

export function newExpressClient(): [Express | null,  Error | null] {
    try {
        return [express(), null];
    } catch (err) {
        return [null, err instanceof Error ? err : new Error(\"general error\")];
    }
}\
" "$common_dir/expressClient.ts"

        write_to_file "\
export type ResponseJSON = {
    status: boolean;
    message: string;
    data: any;
    errors: string[];
}

export function newResponseJSONSuccess(data: any): ResponseJSON {
    return {
        status: true,
        message: \"ok\",
        data: data,
        errors: []
    };
}

export function newResponseJSONError(err: Error): ResponseJSON {
    return {
        status: false,
        message: \"error\",
        data: {},
        errors: [err.message]
    };
}\
" "$common_dir/response.ts"

        write_to_file "\
export function timeNowUTC(): Date {
    return new Date();
}\
" "$common_dir/time.ts"

        write_to_file "\
import { timeNowUTC } from \"@common/time\";

export interface ExampleRepository {
    findByID(id: number): [Example | null, Error | null];
}

export interface ExampleUseCase {
    getDetail(id: number): [ExampleDTO1 | null, Error | null];
}

export interface ExampleHttpService {
    detail(method: string, path: string): void;
}

export class Example {
    public id: number = 0;
    public username: string = \"\";
    public password: string = \"\";
    public createdAt: Date | null = null;

    constructor(obj?: {
        id?: number,
        username?: string,
        password?: string,
        createdAt?: Date | null,
    }) {
        if (obj) {
            Object.assign(this, obj);
        }
    }

    public setCreatedAtNow() {
        this.createdAt = timeNowUTC();
    }

    public toDTO1(): ExampleDTO1 {
        return {
            id: this.id,
            username: this.username,
            createdAt: this.createdAt,
        };
    }
}

export type ExampleDTO1 = {
    id: number;
    username: string;
    createdAt: Date | null;
}\
" "$domain_dir/example.ts"

        write_to_file "\
import {Express, Request, Response} from \"express\";
import {ExampleHttpService, ExampleUseCase} from \"@domain/example\";
import {newResponseJSONSuccess, newResponseJSONError} from \"@common/response\";

export class HttpServiceExpressV1 implements ExampleHttpService {
    private readonly client: Express;
    private readonly exampleUseCase: ExampleUseCase;

    constructor(client: Express, exampleUseCase: ExampleUseCase) {
        this.client = client;
        this.exampleUseCase = exampleUseCase;
    }

    detail(method: string, path: string): void {
        const exampleUseCase = this.exampleUseCase;

        // @ts-ignore
        this.client[method.toLowerCase()](path, function (req: Request, res: Response) {
            try {
                const [result, err] = exampleUseCase.getDetail(1);
                if (err instanceof Error) {
                    res.json(newResponseJSONError(err));
                    return;
                }

                res.json(newResponseJSONSuccess(result!));
            } catch (err) {
                res.json(newResponseJSONError(err instanceof Error ? err : new Error(\"general error\")));
            }
        });
    }
}\
" "$features_example_dir/httpService_express_v1.ts"

        write_to_file "\
import {Example, ExampleRepository} from \"@domain/example\";

export class RepositoryMySQL implements ExampleRepository {
    private readonly db: null;

    constructor(db: null) {
        this.db = db;
    }

    findByID(id: number): [Example | null, Error | null] {
        try {
            let example = new Example({
                id:id,
                username: \"dalikewara\",
                password: \"admin123\"
            });

            example.setCreatedAtNow();

            return [example, null];
        } catch (err) {
            return [null, err instanceof Error ? err : new Error(\"general error\")];
        }
    }
}\
" "$features_example_dir/repository_mysql.ts"

        write_to_file "\
import {ExampleDTO1, ExampleRepository, ExampleUseCase} from \"@domain/example\";

export class UseCaseV1 implements ExampleUseCase {
    private readonly exampleRepository: ExampleRepository;

    constructor(exampleRepository: ExampleRepository) {
        this.exampleRepository = exampleRepository;
    }

    getDetail(id: number): [ExampleDTO1 | null, Error | null] {
        let [example, err] = this.exampleRepository.findByID(id);
        if (err instanceof Error) {
            return [null, err];
        }

        return [example!.toDTO1(), null];
    }
}\
" "$features_example_dir/usecase_v1.ts"

        write_to_file "\
#!/bin/sh

mkdir tmp || true
npm install\
" "$infra_dir/build.sh"

        chmod +x "$infra_dir/build.sh"

        write_to_file "\
#!/bin/sh

./node_modules/.bin/tsc || tsc
./node_modules/.bin/tsc-alias || tsc-alias
node ./dist/main.js\
" "$infra_dir/start.sh"

        chmod +x "$infra_dir/start.sh"

        write_to_file "\
import {newExpressClient} from \"@common/expressClient\"
import {RepositoryMySQL as ExampleRepositoryMySQL} from \"@features/example/repository_mysql\"
import {UseCaseV1 as ExampleUseCaseV1} from \"@features/example/usecase_v1\"
import {HttpServiceExpressV1 as ExampleHttpServiceExpressV1} from \"@features/example/httpService_express_v1\"

// Http server initialization

const [expressClient, expressClientErr] = newExpressClient();

if (expressClientErr instanceof Error) {
    throw expressClientErr;
}

// Repositories

const exampleRepository = new ExampleRepositoryMySQL(null);

// Use cases

const exampleUseCaseV1 = new ExampleUseCaseV1(exampleRepository);

// Services

const exampleHttpServiceExpressV1 = new ExampleHttpServiceExpressV1(expressClient!, exampleUseCaseV1);

// Service handlers

exampleHttpServiceExpressV1.detail(\"GET\", \"/example\");

// Start & listen application

expressClient!.listen(8080, function()  {
    console.log(\"Application is running on port: \" + 8080);
});\
" "main.ts"

        write_to_file "\
{
  \"name\": \"$base_dir\",
  \"version\": \"1.0.0\",
  \"description\": \"\",
  \"author\": \"\",
  \"license\": \"\",
  \"main\": \"./dist/main.js\",
  \"scripts\": {
    \"build\": \"./node_modules/.bin/tsc || tsc && ./node_modules/.bin/tsc-alias || tsc-alias\",
    \"start\": \"node ./dist/main.js\",
    \"test\": \"echo \\\"Error: no test specified\\\" && exit 1\"
  },
  \"dependencies\": {
    \"express\": \"^4.18.3\"
  },
  \"devDependencies\": {
    \"@types/express\": \"^4.17.21\",
    \"ts-node\": \"^10.9.2\",
    \"tsc-alias\": \"^1.8.8\",
    \"tsconfig-paths\": \"^4.2.0\",
    \"typescript\": \"^5.4.2\"
  }
}\
" "package.json"

        write_to_file "\
{
  \"compilerOptions\": {
    \"target\": \"es6\",
    \"module\": \"commonjs\",
    \"moduleResolution\": \"Node\",
    \"rootDir\": \".\",
    \"baseUrl\": \".\",
    \"paths\": {
      \"@*\": [\"*\"],
    },
    \"outDir\": \"./dist\",
    \"esModuleInterop\": true,
    \"forceConsistentCasingInFileNames\": true,
    \"strict\": true,
    \"skipLibCheck\": true
  },
  \"include\": [
    \"**/*\"
  ],
  \"exclude\": [
    \"node_modules\"
  ],
  \"tsc-alias\": {
    \"verbose\": false,
    \"resolveFullPaths\": true
  },
  \"ts-node\": {
    \"require\": [\"tsconfig-paths/register\"]
  }
}\
" "tsconfig.json"

    elif is_nodejs; then
        write_to_file "\
const express = require(\"express\");

function newClient() {
    try {
        return [express(), null];
    } catch (err) {
        return [null, err instanceof Error ? err : new Error(\"general error\")];
    }
}

module.exports = {
    newClient,
};\
" "$common_dir/expressClient.js"

        write_to_file "\
const JSON = {
    status: false,
    message: \"\",
    data: null,
    errors: [],
}

function newJSONSuccess(data) {
    let result = Object.create(JSON);

    result.status = true;
    result.message = \"ok\";
    result.data = data || null;

    return result;
}

function newJSONError(err) {
    let result = Object.create(JSON);

    result.message = \"error\";
    result.errors = [err.message];

    return result;
}

module.exports = {
    newJSONSuccess,
    newJSONError,
};\
" "$common_dir/response.js"

        write_to_file "\
function nowUTC() {
    return new Date();
}

module.exports = {
    nowUTC,
};\
" "$common_dir/time.js"

        write_to_file "\
const comTime = require(\"../common/time\");

class Repository {
    findByID(id) {
        throw new Error(\"findByID() method must be implemented\");
    }
}

class UseCase {
    getDetail(id) {
        throw new Error(\"getDetail() method must be implemented\");
    }
}

class HttpService {
    detail(method, path) {
        throw new Error(\"detail() method must be implemented\");
    }
}

class Example {
    constructor({
        id,
        username,
        password,
    }) {
        this.id = id || 0;
        this.username = username || \"\";
        this.password = password || \"\";
        this.createdAt = null;
    }

    setCreatedAtNow() {
        this.createdAt = comTime.nowUTC();
    }

    toDTO1() {
        return new ExampleDTO1({
            id: this.id,
            username: this.username,
            createdAt: this.createdAt,
        });
    }
}

class ExampleDTO1 {
    constructor({
        id,
        username,
        createdAt,
    }) {
        this.id= id || 0;
        this.username= username || \"\";
        this.createdAt= createdAt || null;
    }
}

module.exports = {
    Repository,
    UseCase,
    HttpService,
    Example,
    ExampleDTO1,
};\
" "$domain_dir/example.js"

        write_to_file "\
const comResponse = require(\"../../common/response\");
const domExample = require(\"../../domain/example\");

class HttpService extends domExample.HttpService {
    constructor(client, exampleUseCase) {
        super();
        this.client = client;
        this.exampleUseCase = exampleUseCase;
    }

    detail(method, path) {
        const exampleUseCase = this.exampleUseCase;

        this.client[method.toLowerCase()](path, function (req, res) {
            try {
                const [result, err] = exampleUseCase.getDetail(1);
                if (err instanceof Error) {
                    res.json(comResponse.newJSONError(err));

                    return;
                }

                res.json(comResponse.newJSONSuccess(result));
            } catch (err) {
                res.json(comResponse.newJSONError(err instanceof Error ? err : new Error(\"general error\")));
            }
        });
    }
}

module.exports = {
    HttpService,
};\
" "$features_example_dir/httpService_express_v1.js"

        write_to_file "\
const domExample = require(\"../../domain/example\");

class Repository extends domExample.Repository {
    constructor(db) {
        super();
        this.db = db;
    }

    findByID(id) {
        try {
            let example = new domExample.Example({
                id: id,
                username: \"dalikewara\",
                password: \"admin123\",
            });

            example.setCreatedAtNow();

            return [example, null];
        } catch (err) {
            return [null, err instanceof Error ? err : new Error(\"general error\")];
        }
    }
}

module.exports = {
    Repository,
};\
" "$features_example_dir/repository_mysql.js"

        write_to_file "\
const domExample = require(\"../../domain/example\");

class UseCase extends domExample.UseCase {
    constructor(exampleRepository) {
        super();
        this.exampleRepository = exampleRepository;
    }

    getDetail(id) {
        let [example, err] = this.exampleRepository.findByID(id);
        if (err instanceof Error) {
            return [null, err];
        }

        return [example.toDTO1(), null];
    }
}

module.exports = {
    UseCase,
};\
" "$features_example_dir/usecase_v1.js"

        write_to_file "\
#!/bin/sh

mkdir tmp || true
npm install\
" "$infra_dir/build.sh"

        chmod +x "$infra_dir/build.sh"

        write_to_file "\
#!/bin/sh

node ./main.js\
" "$infra_dir/start.sh"

        chmod +x "$infra_dir/start.sh"

        write_to_file "\
const comExpressClient = require(\"./common/expressClient\");
const ftrExampleHttpServiceExpressV1 = require(\"./features/example/httpService_express_v1\");
const ftrExampleRepositoryMySQL = require(\"./features/example/repository_mysql\");
const ftrExampleUseCaseV1 = require(\"./features/example/usecase_v1\");

// Http server initialization

const [expressClient, err] = comExpressClient.newClient();
if (err instanceof Error) {
    throw err;
}

// Repositories

const exampleRepositoryMySQL = new ftrExampleRepositoryMySQL.Repository(null);

// Use cases

const exampleUseCaseV1 = new ftrExampleUseCaseV1.UseCase(exampleRepositoryMySQL);

// Services

const exampleHttpServiceExpressV1 = new ftrExampleHttpServiceExpressV1.HttpService(expressClient, exampleUseCaseV1);

// Service handlers

exampleHttpServiceExpressV1.detail(\"GET\", \"/example\");

// Start & listen application

expressClient.listen(8080, function()  {
    console.log(\"Application is running on port: \" + 8080)
});\
" "main.js"

        write_to_file "\
{
  \"name\": \"$base_dir\",
  \"version\": \"1.0.0\",
  \"description\": \"\",
  \"author\": \"\",
  \"license\": \"\",
  \"main\": \"main.js\",
  \"scripts\": {
    \"build\": \"\",
    \"start\": \"node main.js\",
    \"test\": \"echo \\\"Error: no test specified\\\" && exit 1\"
  },
  \"dependencies\": {
    \"express\": \"^4.18.3\"
  },
  \"devDependencies\": {
    \"@types/express\": \"^4.17.21\"
  }
}\
" "package.json"

    elif is_rust; then
        echo "v4 structure for Rust is not ready yet!" && exit 1
    else
        echo "v4 structure for \"$language\" is not supported!" && exit 1
    fi

    write_to_file "\
#!/bin/sh

chmod +x infra/build.sh
chmod +x infra/start.sh
chmod +x infra/docker-up.sh
chmod +x infra/docker-stop.sh
chmod +x infra/docker-down.sh\
" "$infra_dir/chmod.sh"

    chmod +x "$infra_dir/chmod.sh"

    write_to_file "\
#!/bin/sh

docker compose -f docker-compose.yml --project-directory . down\
" "$infra_dir/docker-down.sh"

    chmod +x "$infra_dir/docker-down.sh"

    write_to_file "\
#!/bin/sh

docker compose -f docker-compose.yml --project-directory . stop\
" "$infra_dir/docker-stop.sh"

    chmod +x "$infra_dir/docker-stop.sh"

    write_to_file "\
#!/bin/sh

docker compose -f docker-compose.yml --project-directory . up -d --force-recreate --build\
" "$infra_dir/docker-up.sh"

    chmod +x "$infra_dir/docker-up.sh"

    write_to_file "\
.git
tmp
vendor
node_modules
venv
dist\
" ".dockerignore"

    write_to_file "\
APP_ENV=development
REST_PORT=8080

MYSQL_HOST=localhost
MYSQL_PORT=3306
MYSQL_USER=
MYSQL_PASSWORD=
MYSQL_DB_NAME=\
" ".env.example"

    cp ".env.example" ".env"

    write_to_file "\
# .env
.env*
*.env
!.env.example

# OS X
.DS_Store*
Icon?
._*

# Windows
Thumbs.db
thumbs.db
ehthumbs.db
Desktop.ini

# Linux
.directory
*~

# Binaries for programs and plugins
*.exe
*.exe~
*.dll
*.so
*.dylib

# Test binary, built with \`go test -c\`
*.test

# Output of the go coverage tool, specifically when used with LiteIDE
*.out

# Node artifact files
node_modules/
dist/

# Compiled Java class files
*.class

# Compiled Python bytecode
*.py[cod]

# Log files
*.log

# Package files
*.jar

# Maven
target/

# JetBrains IDE
.idea/

# Unit test reports
TEST*.xml

# Generated by MacOS
.DS_Store

# Applications
*.app
*.war

# Large media files
*.mp4
*.tiff
*.avi
*.flv
*.mov
*.wmv

# Others
bin
.idea
.vscode
.cache
build
*.egg-info
.temp
temp
.tmp
tmp
scripts
venv
venv/*
*pycache*
.pypirc
vendor\
" ".gitignore"

    write_to_file "\
version: '3.7'
services:
  $base_dir-$language:
    image: $base_dir-$language
    env_file:
      - .env
    ports:
      - \"8080:8080\"
    build:
      context: .
    restart: always
    network_mode: \"host\"\
" "docker-compose.yml"

    write_to_file "\
FROM golang:1.19-alpine

# Install system dependecies
RUN apk update
RUN apk add --no-cache git
RUN apk add --no-cache tzdata
RUN apk add --no-cache build-base
RUN apk add --no-cache make
ENV TZ Asia/Jakarta

# Setup the app
WORKDIR /$base_dir-$language
COPY . .
RUN chmod +x infra/chmod.sh
RUN ./infra/chmod.sh

# Build the app
RUN ./infra/build.sh

# Run apps
ENTRYPOINT [\"/$base_dir-$language/infra/start.sh\"]\
" "Dockerfile"

    write_to_file "\
info:
    @echo \"Makefile is your friend\"
    @awk 'BEGIN {FS = \":.*?## \"} /^[a-zA-Z_-]+:.*?## / {printf \"\\033[36m%-20s\\033[0m %s\n\", \$\$1, \$\$2}' \$(MAKEFILE_LIST)

chmod: ## chmod shell scripts
    chmod +x infra/chmod.sh
    ./infra/chmod.sh

start: chmod ## starts service
    ./infra/start.sh

docker-up: chmod ## ups docker service
    ./infra/docker-up.sh

docker-stop: chmod ## stops docker service
    ./infra/docker-stop.sh

docker-down: chmod ## removes docker service
    ./infra/docker-down.sh\
" "Makefile"

    sed -i 's/^    /\t/' "Makefile"

    _extension=""
    _example_domain_1=""
    _example_domain_2=""
    _example_domain_3=""
    _example_common_1=""
    _example_common_2=""
    _example_common_3=""
    _example_external_1=""
    _example_external_2=""

    if is_go; then
        _extension="go"
        _example_domain_1="domain1.go"
        _example_domain_2="domain2.go"
        _example_domain_3="example.go"
        _example_common_1="commonFunction1.go"
        _example_common_2="commonFunction2.go"
        _example_common_3="commonFunction1.go"
        _example_external_1="github.com/go-sql-driver/mysql"
        _example_external_2="github.com/jmoiron/sqlx"
    elif is_python; then
        _extension="py"
        _example_domain_1="domain_1.py"
        _example_domain_2="domain_2.py"
        _example_domain_3="example.py"
        _example_common_1="common_function_1.py"
        _example_common_2="common_function_2.py"
        _example_common_3="common_function_1.py"
        _example_external_1="Flask==3.0.2"
        _example_external_2="Jinja2==3.1.4"
    elif is_typescript; then
        _extension="ts"
        _example_domain_1="domain1.ts"
        _example_domain_2="domain2.ts"
        _example_domain_3="example.ts"
        _example_common_1="commonFunction1.ts"
        _example_common_2="commonFunction2.ts"
        _example_common_3="commonFunction1.ts"
        _example_external_1="express@4.18.3"
        _example_external_2="mysql2@3.12.0"
    elif is_nodejs; then
        _extension="js"
        _example_domain_1="domain1.js"
        _example_domain_2="domain2.js"
        _example_domain_3="example.js"
        _example_common_1="commonFunction1.js"
        _example_common_2="commonFunction2.js"
        _example_common_3="commonFunction1.js"
        _example_external_1="express@4.18.3"
        _example_external_2="mysql2@3.12.0"
    elif is_rust; then
        _extension="rs"
        _example_domain_1="domain_1.rs"
        _example_domain_2="domain_2.rs"
        _example_domain_3="example.rs"
        _example_common_1="commonFunction_1.rs"
        _example_common_2="commonFunction_2.rs"
        _example_common_3="commonFunction_1.rs"
        _example_external_1="hyper@=1.5.2"
        _example_external_2="mysql@=25.0.1"
    fi

    write_to_file "\
# $base_dir

This repository follows **uwais** project structure **$structure_version**.

## Project Structure

To implement the concept of **Clean Architecture** and ~~Domain-Driven Design~~ **Feature-Driven Design**, and to keep them as simple and understandable as possible, **uwais** structures the project like this:

### main.$_extension

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
- A standard **Feature** may comprise the following parts: \`repository\`, \`use case\`, \`http/grpc/cron/etc service\`. But, these are **OPTIONAL**, so feel free to adopt your own style as long as it aligns with the core concept:
  - **repository**
    - Handles communication with external data resources like databases, cloud services, or external services
    - Keep your repositories as simple as possible, avoid adding excessive logic
    - If necessary, separate operations into smaller methods
    - Changes outside the \`repository\` **SHOULD NOT** affect it (except changes for business domain/model/entity)
    - For config variables, database frameworks, or external clients, pass or inject them as dependencies
  - **use case**
    - Contains the main feature logic
    - Changes outside the \`use case\` **SHOULD NOT** affect it (except changes for business domain/model/entity and repository)
    - For config variables, external clients, or repositories, pass or inject them as dependencies
  - **http/grpc/cron/etc service**
    - Hosts feature handlers like HTTP handlers, gRPC handlers, cron jobs, or anything serving between the client and your feature or application
    - Changes outside the \`service\` **SHOULD NOT** affect it (except changes for business domain/model/entity, repository and use case)
    - For config variables, external clients, or use cases, pass or inject them as dependencies
- The \`dependency.json\` is **OPTIONAL**, and only useful when you use the \`import feature\` command. It serves to define the **Feature** dependencies and avoids possible missing package errors

### infra (OPTIONAL)

- This is the location to house infrastructure configurations or scripts to facilitate the deployment of your project on a server or VM

### Make It Your Own

Feel free to create your own style to suit your requirements, as long as you still follow the main architecture concept.
You can create folders such as \`migration\` to store your database migrations, \`tmp\` for temporary files, etc.

## Importing Features from Another Project

To seamlessly incorporate or import features from another project, use the \`import feature\` command:

\`\`\`bash
uwais import feature [feature1,feature2,...] [/local/project or https://example.com/user/project.git or git@example.com:user/project.git]
\`\`\`

For example:

\`\`\`bash
uwais import feature exampleFeature /path/to/your/project
\`\`\`

\`\`\`bash
uwais import feature exampleFeature1,exampleFeature2 git@github.com:username/project.git
\`\`\`

### Feature dependency

This is **OPTIONAL**. But, if your feature relies on external packages, it's crucial to address dependencies properly during the import process.
Failure to import necessary dependencies may result in missing packages.
To prevent this, please put your feature dependencies in the \`dependency.json\` file.
Supported dependencies are limited to the following directories: \`domain\`, \`common\`, and \`features\`.
Ensure that your feature dependencies strictly adhere to these directories, avoiding reliance on other locations.
You can also include any external packages to \`externals\` param to install them automatically.

Example \`dependency.json\` file (\`features/myFeature/dependency.json\`):

\`\`\`json
{
  \"domains\": [
    \"$_example_domain_1\",
    \"$_example_domain_2\"
  ],
  \"features\": [
    \"anotherFeature1\",
    \"anotherFeature2\"
  ],
  \"commons\": [
    \"$_example_common_1\",
    \"$_example_common_1\"
  ],
  \"externals\": [
    \"$_example_external_1\",
    \"$_example_external_2\"
  ]
}
\`\`\`

## Other Commands

There are several commands similar to \`import feature\` above, such as \`import domain\` and \`import common\`.
They function in the same way, for example:

\`\`\`bash
uwais import domain $_example_domain_3 /path/to/your/project
\`\`\`

\`\`\`bash
uwais import common $_example_common_3 https://example.com/user/project.git
\`\`\`\
" "README.md"
}

import_v4() {
    _common_functions_to_be_imported=""
    _domains_to_be_imported=""
    _features_to_be_imported=""
    _external_dependencies_to_be_installed=""

    inspect() {
        IFS=","
        __module="$1"
        __names="$2"
        __source_dir="$3"

        # shellcheck disable=SC2046
        set -- $(get_clean_string_from_space "$__names")

        while [ $# -gt 0 ]; do
            __name="$1"

            if is_empty "$__name"; then
                shift

                continue
            fi

            if is_equal "$__module" "common"; then
                if is_contain "$_common_functions_to_be_imported" "$__name"; then
                    shift

                    continue
                fi

                if is_empty "$_common_functions_to_be_imported"; then
                    _common_functions_to_be_imported="$__name"
                else
                    _common_functions_to_be_imported="$_common_functions_to_be_imported,$__name"
                fi
            elif is_equal "$__module" "domain"; then
                if is_contain "$_domains_to_be_imported" "$__name"; then
                    shift

                    continue
                fi

                if is_empty "$_domains_to_be_imported"; then
                    _domains_to_be_imported="$__name"
                else
                    _domains_to_be_imported="$_domains_to_be_imported,$__name"
                fi
            elif is_equal "$__module" "external"; then
                if is_contain "$_external_dependencies_to_be_installed" "$__name"; then
                    shift

                    continue
                fi

                if is_empty "$_external_dependencies_to_be_installed"; then
                    _external_dependencies_to_be_installed="$__name"
                else
                    _external_dependencies_to_be_installed="$_external_dependencies_to_be_installed,$__name"
                fi
            elif is_equal "$__module" "feature"; then
                if is_contain "$_features_to_be_imported" "$__name"; then
                    shift

                    continue
                fi

                if is_empty "$_features_to_be_imported"; then
                    _features_to_be_imported="$__name"
                else
                    _features_to_be_imported="$_features_to_be_imported,$__name"
                fi

                __dependency_path="$__source_dir/$features_dir/$__name/dependency.json"

                if is_file_exist "$__dependency_path"; then
                    __dependency_json_raw=$(get_raw_json_from_file "$__dependency_path")
                    __dependency_json_raw=$(get_clean_string_from_space "$__dependency_json_raw")
                    __dependency_commons="$(get_json_value_by_key "$__dependency_json_raw" "commons")"
                    __dependency_domains="$(get_json_value_by_key "$__dependency_json_raw" "domains")"
                    __dependency_features="$(get_json_value_by_key "$__dependency_json_raw" "features")"
                    __dependency_external_json_raw=$(get_raw_json_from_file_for_external "$__dependency_path")
                    __dependency_externals="$(get_json_value_by_key_for_external "$__dependency_external_json_raw" "externals")"

                    inspect "common" "$__dependency_commons" "$__source_dir"
                    inspect "domain" "$__dependency_domains" "$__source_dir"
                    inspect "external" "$__dependency_externals" "$__source_dir"
                    inspect "feature" "$__dependency_features" "$__source_dir"
                fi
            fi

            shift
        done

        IFS=$original_ifs
    }

    _names="$1"
    _source="$2"
    _source_dir="$_source"
    _is_need_to_rm_source_dir="no"

    if is_empty "$_names"; then
        echo "Nothing to be imported!" && exit 1
    fi

    if is_empty "$_source_dir"; then
        echo "No source project provided!" && exit 1
    fi

    if is_file_exist "$_source_dir"; then
        echo "Source project can't be a file!" && exit 1
    fi

    if ! is_dir_exist "$_source_dir" && is_git_url "$_source"; then
        _source_dir="tmp-git-project-$(date +%s%N)"

        _is_need_to_rm_source_dir="yes"

        git clone "$_source" "$_source_dir" || true

        if ! is_dir_exist "$_source_dir"; then
            echo "git clone \"$_source\" \"$_source_dir\"... [FAILED]"
        else
            echo "git clone \"$_source\" \"$_source_dir\"... [DONE]"
        fi
    fi

    if ! is_dir_exist "$_source_dir"; then
        if is_equal "$_is_need_to_rm_source_dir" "yes"; then
            remove_dir "$_source_dir"
        fi

        echo "Failed to get contents from the source project, so aborted!" && exit 1
    fi

    inspect "$module" "$_names" "$_source_dir"

    if is_empty "$_common_functions_to_be_imported" && is_empty "$_domains_to_be_imported" && is_empty "$_features_to_be_imported" && \
        is_empty "$_external_dependencies_to_be_installed"; then
        if is_equal "$_is_need_to_rm_source_dir" "yes"; then
            remove_dir "$_source_dir"
        fi

        echo "Failed to get contents from the source project, so aborted!" && exit 1
    fi

    echo "Inspecting possible contents (including dependencies)... [DONE]"

    printf "%b" "\nThese contents will be imported from \"${bold_start}$_source${bold_end}\" into your current project:\n"

    if ! is_empty "$_common_functions_to_be_imported"; then
        printf "%b" "\nCommon functions: ${bold_start}$(echo "$_common_functions_to_be_imported" | sed 's/,/, /g')${bold_end}"
    fi

    if ! is_empty "$_domains_to_be_imported"; then
        printf "%b" "\nDomains: ${bold_start}$(echo "$_domains_to_be_imported" | sed 's/,/, /g')${bold_end}"
    fi

    if ! is_empty "$_features_to_be_imported"; then
        printf "%b" "\nFeatures: ${bold_start}$(echo "$_features_to_be_imported" | sed 's/,/, /g')${bold_end}"
    fi

    if ! is_empty "$_external_dependencies_to_be_installed"; then
        printf "%b" "\nExternal dependencies: ${bold_start}$(echo "$_external_dependencies_to_be_installed" | sed 's/,/, /g')${bold_end}"
    fi

    printf "%b" "\n\nNOTE: Existing files or directories ${bold_start}WILL NOT${bold_end} be overwritten."
    printf "%b" "\n\n"

    read_input "Type '${bold_start}y${bold_end}' and press Enter to confirm. Otherwise, the process will be aborted... " ""

    if ! is_equal "$input" "y"; then
        if is_equal "$_is_need_to_rm_source_dir" "yes"; then
            remove_dir "$_source_dir"
        fi

        echo "Aborted!" && exit 1
    fi

    echo ""

    determine_dir_language "$_source_dir"

    if is_equal "$language" "unknown" || is_empty "$language"; then
        determine_dir_language ""
    fi

    determine_dir_language_module ""

    IFS=","

    # shellcheck disable=SC2046
    set -- $(get_clean_string_from_space "$_common_functions_to_be_imported")

    while [ $# -gt 0 ]; do
        if is_empty "$1"; then
            shift

            continue
        fi

        _target_path="$common_dir/$1"
        _is_need_to_replace_module="no"

        if ! is_file_exist "$_target_path" && ! is_dir_exist "$_target_path"; then
            _is_need_to_replace_module="yes"
        fi

        copy_content "$_source_dir/$common_dir/$1" "$_target_path"

        if is_go && is_equal "$_is_need_to_replace_module" "yes"; then
            _source_go_module=$(get_go_module_from_dir "$_source_dir")
            _target_go_module=$(get_go_module_from_dir "")

            if ! is_empty "$_source_go_module" && ! is_empty "$_target_go_module"; then
                if is_file_exist "$_target_path"; then
                    replace_string_in_file "$_target_path" "$_source_go_module" "$_target_go_module"
                elif is_dir_exist "$_target_path"; then
                    replace_string_in_dir "$_target_path" "$_source_go_module" "$_target_go_module"
                fi
            fi
        fi

        shift
    done

    # shellcheck disable=SC2046
    set -- $(get_clean_string_from_space "$_domains_to_be_imported")

    while [ $# -gt 0 ]; do
        if is_empty "$1"; then
            shift

            continue
        fi

        _target_path="$domain_dir/$1"
        _is_need_to_replace_module="no"

        if ! is_file_exist "$_target_path" && ! is_dir_exist "$_target_path"; then
            _is_need_to_replace_module="yes"
        fi

        copy_content "$_source_dir/$domain_dir/$1" "$_target_path"

        if is_go && is_equal "$_is_need_to_replace_module" "yes"; then
            _source_go_module=$(get_go_module_from_dir "$_source_dir")
            _target_go_module=$(get_go_module_from_dir "")

            if ! is_empty "$_source_go_module" && ! is_empty "$_target_go_module"; then
                if is_file_exist "$_target_path"; then
                    replace_string_in_file "$_target_path" "$_source_go_module" "$_target_go_module"
                elif is_dir_exist "$_target_path"; then
                    replace_string_in_dir "$_target_path" "$_source_go_module" "$_target_go_module"
                fi
            fi
        fi

        shift
    done

    # shellcheck disable=SC2046
    set -- $(get_clean_string_from_space "$_features_to_be_imported")

    while [ $# -gt 0 ]; do
        if is_empty "$1"; then
            shift

            continue
        fi

        _target_path="$features_dir/$1"
        _is_need_to_replace_module="no"

        if ! is_file_exist "$_target_path" && ! is_dir_exist "$_target_path"; then
            _is_need_to_replace_module="yes"
        fi

        copy_content "$_source_dir/$features_dir/$1" "$_target_path"

        if is_go && is_equal "$_is_need_to_replace_module" "yes"; then
            _source_go_module=$(get_go_module_from_dir "$_source_dir")
            _target_go_module=$(get_go_module_from_dir "")

            if ! is_empty "$_source_go_module" && ! is_empty "$_target_go_module"; then
                if is_file_exist "$_target_path"; then
                    replace_string_in_file "$_target_path" "$_source_go_module" "$_target_go_module"
                elif is_dir_exist "$_target_path"; then
                    replace_string_in_dir "$_target_path" "$_source_go_module" "$_target_go_module"
                fi
            fi
        fi

        shift
    done

    if is_equal "$_is_need_to_rm_source_dir" "yes"; then
        remove_dir "$_source_dir"
    fi

    echo "..."

    # shellcheck disable=SC2046
    set -- $(get_clean_string_from_space "$_external_dependencies_to_be_installed")

    while [ $# -gt 0 ]; do
        if is_empty "$1"; then
            shift

            continue
        fi

        if is_go; then
            install_go_package $1
        elif is_python; then
            install_python_package $1
        elif is_typescript; then
            install_typescript_package $1
        elif is_nodejs; then
            install_nodejs_package $1
        elif is_rust; then
            install_rust_package $1
        fi

        shift
    done

    if is_go; then
        go mod tidy || true

        if is_equal "$module" "vendor" || is_equal "$module_2" "vendor"; then
            go mod vendor || true
        fi
    elif is_python; then
        venv/bin/pip freeze > requirements.txt || pip freeze > requirements.txt || true
    elif is_rust; then
        cargo build || true
    fi

    IFS=$original_ifs
}

uwais "$@"
