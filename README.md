# VSCIENCE

vscience is a tool for managing, testing and documenting data science projects. Providing a CLI that allows you to setup and manage your workspace within seconds, vscience is designed to help you focus on your data science work without worrying about the setup and management of your project environment.

## Installation

To install vscience, you can use pip:

```bash
pip install vscience
```

## Features

The main features of vscience include:

### Virtual Environment Management

Relying on uv to manage virtual environments for your data science projects. It allows you to create, activate, and manage virtual environments with ease, ensuring that your project dependencies are isolated and well-organized.

### Project Structure

Providing a standardized project structure for your data science projects. It helps you organize your code, data, and documentation in a way that is easy to navigate and maintain.

### Experimentation

Creates experiment files for your models, allowing you to run different configurations, make rapid comparisions and keeping a record of what you've tried. If you own a Notion account, vscience can connect to it via the Notion API to automatically document your progress and results, making it easier to track your work and share it with others.

### Automatic Documentation

Generates documentation for your experiments and models, including model architecture, hyperparameters, and performance metrics in Markdown format or Notion pages.

### Model evaluation

Auto generates evaluation reports for your models, including learning curves and weights, allowing you to track their performance over time and make informed decisions about model selection and tuning.


### Error analysis

Allows to calculate precision, recall, and F1 score for your models, helping you to evaluate their performance and identify areas for improvement. Moreover, it provides the confusion matrix and plots to visualize the performance of your models, making it easier to understand their strengths and weaknesses.

## Documentation
For more detailed information on how to use vscience, please refer to the [documentation](docs/getting_started.md).

## Contributing
We welcome contributions to vscience! If you would like to contribute, please fork the repository and submit a pull request. For major changes, please open an issue first to discuss what you would like to change.

## Acknowledgements
Virtual Environment Management is powered by [uv](https://github.com/astral-sh/uv), we're grateful for their super fast tool that would speed up development processes.

The vscience CLI relies on [clap](https://github.com/clap-rs/clap) and [inquire](https://github.com/mikaelmello/inquire), which provide a robust and user-friendly interface for interacting with the tool.

For backend implementation, [cargo](https://github.com/rust-lang/cargo) is used to build and package the project, ensuring that it can be easily installed and used by others.

## License
This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.




