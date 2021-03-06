# {{project-name}}
{{description}}

## Features
** --- Enter the features of your app here --- **.

## Contributing

**Important** : make sure to follow the Code of Conduct of this project. See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for details.

### Build instructions

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* Clone the project by running `git clone https://github.com/{{git-username}}/{{project-name}}`
* Go in the project root directory and run `cargo run`

### Reporting issues

Reports an issue if there is an error or a feature you want to be implemented.

Before reporting an issue on the
[issue tracker](https://github.com/{{git-username}}/{{project-name}}/issues),
please check that it has not already been reported by searching for some related
keywords.

## License
{% if license == "MIT" %}This project is licensed under the MIT license.
{% endif %}{% if license == "GNU v3" %}This project is licensed under the GNU GPL v3 license.
{% endif %}{% if license == "Apache v2" %}This project is licensed under the Apache v2 license.
{% endif %}
See the [LICENSE](LICENSE) for details.