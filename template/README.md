# {{project-name}}
{{description}}

## Features
** --- Enter the features of your app here --- **.

### Contributing

## Build instructions

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* clone the project by running `git clone https://github.com/{{git-username}}/{{project-name}}`
* go in the project root and run `cargo run`

## Reporting issues

Before reporting an issue on the
[issue tracker](https://github.com/{{gh-username}}/{{project-name}}/issues),
please check that it has not already been reported by searching for some related
keywords.

### License
{% if license == "MIT" %}
This project is licensed under the MIT license.
{% endif %}
{% if license == "GNU v3" %}
This project is licensed under the GNU GPL v3 license.
{% endif %}
{% if license == "Apache v2" %}
This project is licensed under the Apache v2 license.
{% endif %}

See the [LICENSE](LICENSE) for details.